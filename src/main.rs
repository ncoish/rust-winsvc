extern crate rust_winsvc;
extern crate parking_lot;
#[macro_use]
extern crate lazy_static;
use parking_lot::Mutex;

use rust_winsvc::service::{ServiceTable, ServiceStatus, ServiceType, ServiceState, ServiceAccept};

lazy_static! {
    static ref SERVICE_STATUS_HANDLER: Mutex<Option<ServiceStatus>> = Mutex::new(None);
}

unsafe extern "system" fn testfunc(_argc: u32, _argv: *mut *mut u16) {
    let service_status = ServiceStatus::new(
        ServiceType::WIN32,
        ServiceState::START_PENDING,
        vec!{ServiceAccept::STOP, ServiceAccept::SHUTDOWN},
        0,
        0,
        0,
        0
    );
    *SERVICE_STATUS_HANDLER.lock() = Some(service_status);

    loop {
        println!("Hello");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

// unsafe extern "system" fn control_handler(request: DWORD) {
//     match request {

//     }
// }

fn main() {
    let mut service_table = ServiceTable::new();
    service_table.register_new_service("hello_service", testfunc);
    service_table.start();
}
