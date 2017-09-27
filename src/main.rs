extern crate rust_winsvc;

use rust_winsvc::service::{ServiceTable, ServiceStatus, ServiceType, ServiceState, ServiceAccept};

unsafe extern "system" fn testfunc(argc: u32, argv: *mut *mut u16) {
    let service_status = ServiceStatus::new(
        service_type: ServiceType::WIN32,
        current_state: ServiceState::START_PENDING,
        controls_accepted: vec!{ServiceAccept::STOP, ServiceAccept::SHUTDOWN},
        win32_exit_code: 0,
        service_specific_exit_code: 0,
        check_point: 0,
        wait_hint: 0,
    );

    loop {
        println!("Hello");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn main() {
    let mut service_table = ServiceTable::new();
    service_table.register_new_service("hello_service", testfunc);
    service_table.start();
}
