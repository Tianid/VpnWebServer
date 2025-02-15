use std::process::{Command};
use std::sync::{Mutex};

pub struct VpnManager {
    status: Mutex<VpnStatus>,
}


impl VpnManager {

    pub fn new() -> Self {
        Self {
            status: Mutex::new(VpnStatus::Disconnected),
        }
    }

    pub fn start(&mut self) {
        self.set_status(VpnStatus::Connecting);

        let result = Command::new("adguardvpn-cli")
            .arg("connect")
            .arg("-l")
            .arg("DE")
            .output();

        match result {
            Ok(output) => {
                if output.status.success() {
                    // FIXME calculate real status and set it
                    self.set_status(VpnStatus::Connected);
                    println!("adguardvpn-cli connected successfully");
                    return;
                }
                // FIXME calculate real status and set it
                self.set_status(VpnStatus::Disconnected);
                eprintln!("adguardvpn-cli connect failed: {:?}", output.stderr);
            }
            Err(e) => {
                // FIXME calculate real status and set it
                self.set_status(VpnStatus::Disconnected);
                eprintln!("adguardvpn-cli failed to start: {}", e);
            }
        }
    }

    pub fn stop(&self) {
        self.set_status(VpnStatus::Disconnecting);

        let result = Command::new("adguardvpn-cli")
            .arg("disconnect")
            .output();

        match result {
            Ok(output) => {
                if output.status.success() {
                    self.set_status(VpnStatus::Disconnected);
                    println!("adguardvpn-cli disconnected successfully");
                    return;
                }
                // FIXME calculate real status and set it
                // self.set_status(VpnStatus::Disconnected);
                eprintln!("adguardvpn-cli disconnect failed: {:?}", output.stderr);
            }
            Err(e) => {
                self.set_status(VpnStatus::Disconnected);
                eprintln!("adguardvpn-cli failed to stop: {}", e);
            }
        }
    }

    fn set_status(&self, status: VpnStatus) {
        *self.status.lock().unwrap() = status;
        // FIXME call callback
    }

    pub fn get_status(&self) -> VpnStatus {
        *self.status.lock().unwrap()
    }
}









#[derive(Clone, Copy, PartialEq)]
pub enum VpnStatus {
    Disconnected,
    Connected,
    Disconnecting,
    Connecting,
    Reconnecting,
}
