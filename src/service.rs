use winapi::winsvc;
use winapi::winnt;
use winapi::minwindef::DWORD;

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

struct ServiceStatus {
    dwServiceType: ServiceType,

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let a = ServiceAccept::STOP;
        let b = ServiceAccept::TRIGGEREVENT;
        let c = a | b;
        assert_eq!(c, 5);
    }
}