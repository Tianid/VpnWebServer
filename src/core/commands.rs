use std::process::Command;

pub fn vpn_connect(location: Option<&str>) -> Command {
    let mut cmd = Command::new("adguardvpn-cli");
    cmd.arg("connect");
    match location {
        Some(city) => { cmd.arg("-l").arg(city); }
        None       => { cmd.arg("--fastest"); }
    }
    cmd
}

pub fn vpn_disconnect() -> Command {
    let mut cmd = Command::new("adguardvpn-cli");
    cmd.arg("disconnect");
    cmd
}

pub fn vpn_status() -> Command {
    let mut cmd = Command::new("adguardvpn-cli");
    cmd.arg("status");
    cmd
}

pub fn vpn_list_locations(count: u32) -> Command {
    let mut cmd = Command::new("adguardvpn-cli");
    cmd.arg("list-locations").arg(count.to_string());
    cmd
}

pub fn system_restart() -> Command {
    let mut cmd = Command::new("shutdown");
    cmd.arg("-r").arg("now");
    cmd
}

pub fn wifi_get_ssid() -> Command {
    let mut cmd = Command::new("sh");
    cmd.arg("-c")
       .arg(r#"nmcli -t -f active,ssid dev wifi | grep '^yes:' | cut -d':' -f2-"#);
    cmd
}

pub fn wifi_reconnect(ssid: &str) -> Command {
    let mut cmd = Command::new("nmcli");
    cmd.args(["connection", "up", "id", ssid, "--ask"]);
    cmd
}
