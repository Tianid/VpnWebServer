use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};

pub struct VpnManager {
    status: Mutex<VpnStatus>,
    callback: Arc<dyn Fn(VpnStatus) + Send + Sync>
}


impl VpnManager {

    pub fn new(callback: Arc<dyn Fn(VpnStatus) + Send + Sync>) -> Self {
        Self {
            status: Mutex::new(VpnStatus::Disconnected),
            callback
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
        (self.callback)(status)
    }

    pub fn get_status(&self) -> VpnStatus {
        *self.status.lock().unwrap()
    }

    pub fn get_vpn_status(&self) -> VpnStatus {
        let output = Command::new("ip")
            .arg("link")
            .arg("show")
            .output()
            .expect("Не удалось выполнить ip link");

        let actual_status: VpnStatus = if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.contains("tun") { VpnStatus::Connected } else { VpnStatus::Disconnected }
        } else {
            VpnStatus::Disconnected
        };

        self.set_status(actual_status);

        actual_status
    }
}









#[derive(Clone, Copy, PartialEq, Debug)]
pub enum VpnStatus {
    Disconnected,
    Connected,
    Disconnecting,
    Connecting,
    Reconnecting,
}
