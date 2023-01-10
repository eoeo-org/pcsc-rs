mod status;

use dotenvy::dotenv;
use rust_socketio::{ClientBuilder, Payload, RawClient};
use serde_json::json;
use std::{
    env, process,
    sync::{Arc, Mutex},
};
use sysinfo::{CpuExt, System, SystemExt};

use crate::status::{CpuData, StatusDataWithPass};

struct App {
    pub finish: bool,
}

impl App {
    pub fn new() -> Self {
        App { finish: false }
    }

    fn on_message(&mut self, payload: Payload, _socket: RawClient) {
        println!("message: {:#?}", payload);
        //socket.emit("disconnect", "received message").expect("Server unreachable");
        self.finish = true;
    }
}

fn main() {
    dotenv().expect(".env file not found");

    let pcsc_uri = match env::var("PCSC_URI") {
        Ok(val) => val,
        Err(_) => "https://pcss.eov2.com".to_string(),
    };

    if System::IS_SUPPORTED {
        println!("This OS is supported!");
        println!("Hello, world! {}", pcsc_uri);
    } else {
        println!("This OS isn't supported (yet?).");
        process::exit(0x0004);
    }

    let app = Arc::new(Mutex::new(App::new()));
    let event_app = app.clone();

    ClientBuilder::new(pcsc_uri)
        .namespace("/server")
        .on("open", |_, _| println!("Connected"))
        .on("close", |_, _| println!("Disconnected"))
        .on("hi", |payload: Payload, socket: RawClient| {
            match payload {
                Payload::String(str) => println!("Received: {}", str),
                Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
            };
            init(socket);
        })
        //.on("sync", send_system_info)
        .on("message", move |msg, client| {
            event_app.lock().unwrap().on_message(msg, client)
        })
        .on("error", |err, _| match err {
            Payload::String(str) => eprintln!("Error: {}", str),
            Payload::Binary(bin_data) => eprintln!("Error: {:#?}", bin_data),
        })
        .connect()
        .expect("Connection failed");

    loop {
        if app.lock().unwrap().finish {
            break;
        }
    }
}

fn init(socket: RawClient) {
    print!("hi from server");

    let mut sys = System::new_all();
    sys.refresh_all();

    let _pass = match env::var("PASS") {
        Ok(val) => val,
        Err(_) => "".to_string(),
    };

    println!("{}", _pass);

    let cpu_name = sys.cpus()[0].brand().to_string();
    let os_name = sys.name().expect("Failed to get os name");
    let os_version = sys
        .os_version()
        .or(sys.kernel_version())
        .expect("Failed to get os version");
    let hostname = sys.host_name().expect("Failed to get hostname");

    socket
        .emit(
            "hi",
            json!(StatusDataWithPass {
                pass: _pass,
                _os: format!("{} {}", os_name, os_version),
                hostname: hostname,
                version: "rust".to_string(),
                cpu: CpuData {
                    model: cpu_name,
                    cpus: vec![],
                    percent: 0,
                },
                loadavg: None,
            }),
        )
        .expect("Failed to emit.");
}
