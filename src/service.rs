use winapi::winsvc;
use winapi::winnt;
use winapi::{DWORD, LPWSTR};
use advapi32;

use util;
use std;

#[derive(Debug)]
pub enum ServiceAccept {
    STOP,
    SHUTDOWN,
    POWEREVENT,
    TIMECHANGE,
    PARAMCHANGE,
    PRESHUTDOWN,
    TRIGGEREVENT,
    NETBINDCHANGE,
    SESSIONCHANGE,
    PAUSE_CONTINUE,
    HARDWAREPROFILECHANGE,
}

impl ServiceAccept {
    pub fn value(&self) -> DWORD {
        match *self {
            ServiceAccept::STOP                     =>  winsvc::SERVICE_ACCEPT_STOP,
            ServiceAccept::SHUTDOWN                 =>  winsvc::SERVICE_ACCEPT_SHUTDOWN,
            ServiceAccept::POWEREVENT               =>  winsvc::SERVICE_ACCEPT_POWEREVENT,
            ServiceAccept::TIMECHANGE               =>  winsvc::SERVICE_ACCEPT_TIMECHANGE,
            ServiceAccept::PARAMCHANGE              =>  winsvc::SERVICE_ACCEPT_PARAMCHANGE,
            ServiceAccept::PRESHUTDOWN              =>  winsvc::SERVICE_ACCEPT_PRESHUTDOWN,
            ServiceAccept::TRIGGEREVENT             =>  winsvc::SERVICE_ACCEPT_TRIGGEREVENT,
            ServiceAccept::NETBINDCHANGE            =>  winsvc::SERVICE_ACCEPT_NETBINDCHANGE,
            ServiceAccept::SESSIONCHANGE            =>  winsvc::SERVICE_ACCEPT_SESSIONCHANGE,
            ServiceAccept::PAUSE_CONTINUE           =>  winsvc::SERVICE_ACCEPT_PAUSE_CONTINUE,
            ServiceAccept::HARDWAREPROFILECHANGE    =>  winsvc::SERVICE_ACCEPT_HARDWAREPROFILECHANGE,
        }
    }
}

impl Into<DWORD> for ServiceAccept {
    fn into(self) -> DWORD {
        self.value()
    }
}

#[derive(Debug)]
pub enum ServiceType {
    ADAPTER,
    DRIVER,
    FILE_SYSTEM_DRIVER,
    INTERACTIVE_PROCESS,
    KERNEL_DRIVER,
    RECOGNIZER_DRIVER,
    TYPE_ALL,
    WIN32,
    WIN32_OWN_PROCESS,
    WIN32_SHARE_PROCESS,
}

impl ServiceType {
    pub fn value(&self) -> DWORD {
        match *self {
            ServiceType::ADAPTER                =>  winnt::SERVICE_ADAPTER,
            ServiceType::DRIVER                 =>  winnt::SERVICE_DRIVER,
            ServiceType::FILE_SYSTEM_DRIVER     =>  winnt::SERVICE_FILE_SYSTEM_DRIVER,
            ServiceType::INTERACTIVE_PROCESS    =>  winnt::SERVICE_INTERACTIVE_PROCESS,
            ServiceType::KERNEL_DRIVER          =>  winnt::SERVICE_KERNEL_DRIVER,
            ServiceType::RECOGNIZER_DRIVER      =>  winnt::SERVICE_RECOGNIZER_DRIVER,
            ServiceType::TYPE_ALL               =>  winnt::SERVICE_TYPE_ALL,
            ServiceType::WIN32                  =>  winnt::SERVICE_WIN32,
            ServiceType::WIN32_OWN_PROCESS      =>  winnt::SERVICE_WIN32_OWN_PROCESS,
            ServiceType::WIN32_SHARE_PROCESS    =>  winnt::SERVICE_WIN32_SHARE_PROCESS,
        }
    }
}

impl Into<DWORD> for ServiceType {
    fn into(self) -> DWORD {
        self.value()
    }
}

#[derive(Debug)]
pub enum ServiceControl {
    STOP,
    PAUSE,
    CONTINUE,
}

#[derive(Debug)]
pub enum ServiceState {
    CONTINUE_PENDING,
    PAUSE_PENDING,
    PAUSED,
    RUNNING,
    START_PENDING,
    STOP_PENDING,
    STOPPED,
}

impl ServiceState {
    pub fn value(&self) -> DWORD {
        match *self {
            ServiceState::CONTINUE_PENDING  =>  winsvc::SERVICE_CONTINUE_PENDING,
            ServiceState::PAUSE_PENDING     =>  winsvc::SERVICE_PAUSE_PENDING,
            ServiceState::PAUSED            =>  winsvc::SERVICE_PAUSED,
            ServiceState::RUNNING           =>  winsvc::SERVICE_RUNNING,
            ServiceState::START_PENDING     =>  winsvc::SERVICE_START_PENDING,
            ServiceState::STOP_PENDING      =>  winsvc::SERVICE_STOP_PENDING,
            ServiceState::STOPPED           =>  winsvc::SERVICE_STOPPED,
        }
    }
}

pub struct ServiceStatus {
    service_type: ServiceType,
    current_state: ServiceState,
    controls_accepted: Vec<ServiceAccept>,
    win32_exit_code: DWORD,
    service_specific_exit_code: DWORD,
    check_point: DWORD,
    wait_hint: DWORD,
}

impl ServiceStatus {
    pub fn new(
        service_type: ServiceType,
        current_state: ServiceState,
        controls_accepted: Vec<ServiceAccept>,
        win32_exit_code: DWORD,
        service_specific_exit_code: DWORD,
        check_point: DWORD,
        wait_hint: DWORD,
    ) -> Self {
        ServiceStatus {
            service_type,
            current_state,
            controls_accepted,
            win32_exit_code,
            service_specific_exit_code,
            check_point,
            wait_hint,
        }
    }

    pub fn get_inner(&self) -> winsvc::SERVICE_STATUS {
        let dwServiceType = self.service_type.value();
        let dwCurrentState = self.current_state.value();
        let dwControlsAccepted: DWORD = self.controls_accepted.iter().fold(0, |mut bitsum, val| { bitsum |= val.value(); bitsum});
        winsvc::SERVICE_STATUS {
            dwServiceType,
            dwCurrentState,
            dwControlsAccepted,
            dwWin32ExitCode: self.win32_exit_code,
            dwServiceSpecificExitCode: self.service_specific_exit_code,
            dwCheckPoint: self.check_point,
            dwWaitHint: self.wait_hint,
        }
    }
}

pub struct ServiceTable {
    services: Vec<winsvc::SERVICE_TABLE_ENTRYW>,
}

impl ServiceTable {
    pub fn new() -> Self {
        ServiceTable {
            services: Vec::new(),
        }
    }

    pub fn register_new_service(
        &mut self,
        service_name: &str,
        service_main: unsafe extern "system" fn(dwNumServicesArgs: DWORD, lpServiceArgVectors: *mut LPWSTR)
    ) {
        let service = winsvc::SERVICE_TABLE_ENTRYW {
            lpServiceName: util::win32_string(service_name).as_ptr(),
            lpServiceProc: Some(service_main),
        };
        self.services.push(service);
    }

    pub fn start(mut self) {
        self.services.push(winsvc::SERVICE_TABLE_ENTRYW { lpServiceName: std::ptr::null(), lpServiceProc: None});
        unsafe { advapi32::StartServiceCtrlDispatcherW(self.services.as_ptr()) };
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn basic_test() {
//         let a = ServiceAccept::STOP;
//         let b = ServiceAccept::TRIGGEREVENT;
//         let c = a | b;
//         assert_eq!(c, 5);
//     }
// }
