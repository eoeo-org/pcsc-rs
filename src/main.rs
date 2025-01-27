#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod gpu;
mod status;
mod sysinfo_instance;
mod thread_message;
mod threads;
mod unix_to_date;

use anyhow::{Ok, Result};
use bytes::Bytes;
use fastwebsockets::{FragmentCollector, Frame, OpCode, Payload};
use http_body_util::Empty;
use hyper::{header::{CONNECTION, UPGRADE}, upgrade::Upgraded, Request};
use hyper_util::rt::TokioIo;
use pcsc_rs::start;
use serde_json::json;
use status::SystemStatus;
use sysinfo_instance::SysinfoInstance;
use tokio::net::TcpStream;
use std::future::Future;

struct SpawnExecutor;

impl<Fut> hyper::rt::Executor<Fut> for SpawnExecutor
where
    Fut: Future + Send + 'static,
    Fut::Output: Send + 'static,
{
    fn execute(&self, fut: Fut) {
        tokio::task::spawn(fut);
    }
}

async fn connect() -> Result<FragmentCollector<TokioIo<Upgraded>>> {
    let stream = TcpStream::connect("localhost:3000").await?;

    let req = Request::builder()
        .method("GET")
        .uri(format!("http://localhost:3000"))
        .header("Host", "localhost:3000")
        .header(UPGRADE, "websocket")
        .header(CONNECTION, "upgrade")
        .header(
            "Sec-WebSocket-Key",
            fastwebsockets::handshake::generate_key(),
        )
        .header("Sec-WebSocket-Version", "13")
        .body(Empty::<Bytes>::new())?;

    let (ws, _) = fastwebsockets::handshake::client(&SpawnExecutor, req, stream).await?;
    Ok(FragmentCollector::new(ws))
}

// pub async fn start() {
//     let mut _ws = connect().await.expect("Failed to connect websocket!");
//     let sys = SysinfoInstance::new();
//     loop {
//         let _msg = _ws.read_frame().await.expect("Failed to read websocket frame");
//         match _msg.opcode {
//             OpCode::Text | OpCode::Binary => {
//                 let mut payload_data: Vec<u8> = Vec::new();
//                 _msg.payload.clone_into(&mut payload_data);
//                 let _t = std::str::from_utf8(&payload_data).expect("Failed to convert to utf8 string");
//                 if _t.eq("sync") {
//                     println!("syncing...");
//                     let raw_status = format!("{}",json!(SystemStatus::get(&sys)));
//                     if let Err(e) = _ws.write_frame(Frame::text(Payload::Borrowed(raw_status.as_bytes()))).await {
//                         panic!("Failed to send system status: {:?}", e)
//                     }
//                     println!("sync was successfully!")
//                 } else {
//                     println!("Normal text: {}", _t);
//                 }
//             }
//             _ => {}
//         }
//     }
// }

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    start().await;
    Ok(())
    // let mut ws = connect().await?;

    // // get fake pc status
    // let instance = SysinfoInstance::new();
    // let status = SystemStatus::get(&instance);
    // let encoded = format!("{}", json!(status));

    // let value: &[u8] = encoded.as_bytes();

    // let msg = Frame::new(true, OpCode::Text, None, fastwebsockets::Payload::Borrowed(&value));

    // match msg.opcode {
    //     OpCode::Text | OpCode::Binary => {
    //         ws.write_frame(Frame::new(true, msg.opcode, None, msg.payload))
    //             .await?;
    //     }
    //     OpCode::Close => {
    //         return Result::Ok(());
    //     }
    //     _ => {}
    // }

    // sleep(Duration::from_secs(2)).await;

    // let _close_value: &[u8] = "".as_bytes();
    // // ws.write_frame(Frame::close(1000, &[])).await?;
    // ws.write_frame(Frame::new(true, OpCode::Close, None, fastwebsockets::Payload::Borrowed(&_close_value))).await?;

    // Ok(())
}

//#[tokio::main(flavor = "multi_thread")]
//async fn main() {
//    let rs = Path::new(".env").exists();
//    if rs {
//        dotenv().expect(".env file not found");
//    }
//
//    if !IS_SUPPORTED_SYSTEM {
//        println!("This OS isn't supported (yet?).");
//        process::exit(95);
//    }
//
//    if !env::var("PASS").is_ok() {
//        println!("The environment variable Password (PASS) is not specified.");
//        process::exit(95);
//    }
//
//    //pcsc_rs::start().await;
//    start().await;
//}
