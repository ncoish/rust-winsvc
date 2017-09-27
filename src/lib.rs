extern crate winapi;
extern crate advapi32;

mod util;
pub mod service;

use winapi::winsvc;

use winapi::{DWORD, LPWSTR};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_register_new_service() {
        unsafe extern "system" fn testfunc(argc: u32, argv: *mut *mut u16) {
            loop {
                println!("Hello");
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
        let mut service_table = service::ServiceTable::new();
        service_table.register_new_service("hello_service", testfunc);
        service_table.start();
    }
}
