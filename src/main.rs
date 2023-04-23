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
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use std::{
    env, hint, process,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use sysinfo::{System, SystemExt};

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

fn main() {
    dotenv().expect(".env file not found");

    if !System::IS_SUPPORTED {
        println!("This OS isn't supported (yet?).");
        process::exit(95);
    }

    start();
}

fn start() {
    let mut system = System::new_all();

    let shared_data = Arc::new(ArcSwap::from_pointee(SystemStatus::get(&mut system)));

    threads::spawn_monitor(Arc::clone(&shared_data));

    let pcsc_uri = env::var("PCSC_URI").unwrap_or_else(|_| "https://pcss.eov2.com".into());

    println!("This OS is supported!");
    println!("Hello, world! {}", pcsc_uri);

    let app = Arc::new(Mutex::new(App::new()));
    let event_app = app.clone();

    ClientBuilder::new(pcsc_uri)
        .namespace("/server")
        .on(Event::Connect, |_, _| println!("Connected"))
        .on(Event::Close, |_, _| println!("Disconnected"))
        .on("hi", |payload, socket| {
            match payload {
                Payload::String(str) => println!("Received: {}", str),
                Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
            };
            init(socket);
        })
        .on("sync", move |payload, socket| {
            match payload {
                Payload::String(str) => println!("Received: {}", str),
                Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
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
            Payload::String(str) => eprintln!("Error: {}", str),
            Payload::Binary(bin_data) => eprintln!("Error: {:#?}", bin_data),
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

    let mut sys = System::new_all();
    sys.refresh_all();

    let pass = env::var("PASS").unwrap_or_default();

    let status = SystemStatus::get(&mut sys);
    socket
        .emit("hi", json!(status.with_pass(pass)))
        .expect("Failed to emit.");
}
