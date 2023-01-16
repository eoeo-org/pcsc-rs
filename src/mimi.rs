use arc_swap::ArcSwap;
use serde::Serialize;
use serde_json::{json, Value};
use sysinfo::{CpuExt, System, SystemExt};

use std::{sync::Arc, thread, time::Duration};

#[derive(Serialize)]
struct CoreData {
    usage: f32,
}

fn get_bob(system: &mut System) -> Box<[CoreData]> {
    system
        .cpus()
        .iter()
        .map(|core| CoreData {
            usage: core.cpu_usage(),
        })
        .collect()
}

fn pretend_its_listening_to_an_event(_event: &str, _callback: impl FnMut()) {}
fn fake_send(_: Value) {}

// rahu ko-do
pub(crate) fn main() {
    // system tukuru
    let mut system = System::new_all();

    let shared_data: Arc<
        // ArcSwap wo thread to kyouyuu suru tameno Arc
        ArcSwap<
            // system kara get sita data wo butikomu ArcSwap wo tukuru
            Box<[CoreData]>, // CoreData no hairetu
                             // Vec<CoreData> // koredemoii kedo yousowo huyasanai tokiha Box<[T]> de juubun
        >,
    > = Arc::new(ArcSwap::from_pointee(get_bob(&mut system)));

    // kataha kaisetu no tameni kaitadakede kakanakute ii
    let _ = Arc::new(ArcSwap::from_pointee("hello".to_string()));

    // data gathering thread
    thread::spawn({
        // block wo tukatte hokanokotomo suru
        // Arc<ArcSwap> wo clone site thread no scope de tukaeru youni suru
        let shared_data = Arc::clone(&shared_data);
        // thread de jikkou sareru closure wo kaesu
        move || {
            loop {
                // 1byou gotoni arc ni butikomu
                thread::sleep(Duration::from_secs(1));

                system.refresh_cpu();
                let data = get_bob(&mut system);
                shared_data.store(Arc::new(data));
            }
        }
    });

    // socket.on("sync", () => {
    //     socket.send("sync")
    // })
    pretend_its_listening_to_an_event("sync", move || {
        let data = shared_data.load();
        fake_send(json!(data.as_ref()));
    });
}
