#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[test]
fn test_function() {
    if false {
        panic!("Test Failed.");
    }
}

mod gpu;
mod status;
mod thread_message;
mod threads;
mod unix_to_date;

use arc_swap::ArcSwap;
use dotenvy::dotenv;
use rust_socketio::{ClientBuilder, Event, Payload, RawClient};
use self_update::cargo_crate_version;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use std::{
    env, hint,
    path::{Path, PathBuf},
    process,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use sysinfo::{CpuRefreshKind, Disks, MemoryRefreshKind, RefreshKind, System, IS_SUPPORTED_SYSTEM};

use crate::status::SystemStatus;

struct App {
    pub finish: bool,
}

impl App {
    pub fn new() -> Self {
        App { finish: false }
    }

    fn on_message(&mut self, payload: Payload, _socket: RawClient) {
        println!("message: {:#?}", payload);
        self.finish = true;
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct AppConfig {
    uri: String,
    password: Option<String>,
    hostname: Option<String>,
}

impl ::std::default::Default for AppConfig {
    fn default() -> Self {
        Self {
            uri: "https://pcss.eov2.com".into(),
            password: None,
            hostname: None,
        }
    }
}

fn restart_program(bin_install_path: PathBuf) {
    use std::process::{exit, Command};

    {
        Command::new(bin_install_path)
            .spawn()
            .expect("Failed to restart the program");
    }

    exit(0);
}

fn update() -> Result<(), Box<dyn (::std::error::Error)>> {
    let config = self_update::backends::github::Update::configure()
        .repo_owner("eoeo-org")
        .repo_name("pcsc-rs")
        .bin_name("pcsc-rs")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .no_confirm(true)
        .build()?;

    let bin_install_path = config.bin_install_path();

    println!("{:?}", bin_install_path);

    let status = config.update()?;

    if status.updated() {
        if let Ok(x) = env::var("PCSC_UPDATED") {
            match x.as_str() {
                "restart" => restart_program(bin_install_path),
                "terminate" => process::exit(0),
                "none" | _ => {}
            }
        }
    };

    Ok(())
}

fn main() {
    let rs = Path::new(".env").exists();
    if rs {
        dotenv().expect(".env file not found");
    }

    if !IS_SUPPORTED_SYSTEM {
        println!("This OS isn't supported (yet?).");
        process::exit(95);
    }

    if !env::var("PASS").is_ok() {
        println!("The environment variable Password (PASS) is not specified.");
        process::exit(95);
    }

    start();
}

fn start() {
    let _ = update();

    let mut system = System::new_with_specifics(
        RefreshKind::new()
            .with_cpu(CpuRefreshKind::new().with_cpu_usage())
            .with_memory(MemoryRefreshKind::everything()),
    );
    let mut disks = Disks::new_with_refreshed_list();

    let shared_data = Arc::new(ArcSwap::from_pointee(SystemStatus::get(
        &mut system,
        &mut disks,
    )));

    threads::spawn_monitor(Arc::clone(&shared_data));

    let pcsc_uri = env::var("PCSC_URI").unwrap_or_else(|_| "https://pcss.eov2.com".into());

    println!("This OS is supported!");
    println!("Hello, world! {}", pcsc_uri);

    let app = Arc::new(Mutex::new(App::new()));
    let event_app = app.clone();

    ClientBuilder::new(pcsc_uri)
        .namespace("/server")
        .reconnect_on_disconnect(true)
        .on(Event::Connect, |_, _| println!("Connected"))
        .on(Event::Close, |_, _| println!("Disconnected"))
        .on("hi", |payload, socket| {
            match payload {
                Payload::Text(values) => println!("Received: {}", values[0]),
                Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
                _ => (),
            };
            init(socket);
        })
        .on("sync", move |payload, socket| {
            match payload {
                Payload::Text(values) => println!("Received: {}", values[0]),
                Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
                _ => (),
            };

            let status = shared_data.load();
            if let Err(e) = socket.emit("sync", json!(status.as_ref())) {
                dbg!(e);
            }
        })
        .on(Event::Message, move |payload, client| {
            event_app.lock().unwrap().on_message(payload, client)
        })
        .on(Event::Error, |err, _| match err {
            Payload::Text(values) => eprintln!("Error: {}", values[0]),
            Payload::Binary(bin_data) => eprintln!("Error: {:#?}", bin_data),
            _ => (),
        })
        .connect()
        .expect("Connection failed");

    while !app.lock().unwrap().finish {
        hint::spin_loop();
        thread::sleep(Duration::from_secs(1));
    }
}

fn init(socket: RawClient) {
    print!("hi from server");

    let mut sys = System::new_with_specifics(
        RefreshKind::new()
            .with_cpu(CpuRefreshKind::new().with_cpu_usage())
            .with_memory(MemoryRefreshKind::everything()),
    );
    let mut disks = Disks::new_with_refreshed_list();

    let pass = env::var("PASS").unwrap_or_default();
    let status = SystemStatus::get(&mut sys, &mut disks);
    socket
        .emit("hi", json!(status.with_pass(pass)))
        .expect("Failed to emit.");
}
