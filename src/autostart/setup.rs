use crate::config::args::ServerConfig;

pub fn setup(cfg: &ServerConfig) -> Result<(), String> {
    let home         = super::resolve_home()?;
    let terminal     = super::terminal::detect()
        .ok_or_else(|| "No terminal emulator found. Install one with: sudo apt install xterm".to_string())?;
    let binary       = std::env::current_exe()
        .map_err(|e| format!("Cannot determine binary path: {}", e))?;
    let project_root = std::env::current_dir()
        .map_err(|e| format!("Cannot determine working directory: {}", e))?;

    let bin_dir      = format!("{}/.local/bin",           home);
    let autostart_dir = format!("{}/.config/autostart",  home);

    std::fs::create_dir_all(&bin_dir)
        .map_err(|e| format!("Cannot create {}: {}", bin_dir, e))?;
    std::fs::create_dir_all(&autostart_dir)
        .map_err(|e| format!("Cannot create {}: {}", autostart_dir, e))?;

    let server_script  = format!("{}/haven-autostart.sh",          bin_dir);
    let vpn_script     = format!("{}/haven-vpn-connect.sh",        bin_dir);
    let server_desktop = format!("{}/haven-server.desktop",  autostart_dir);
    let vpn_desktop    = format!("{}/haven-vpn.desktop",     autostart_dir);

    write_server_script(
        &server_script,
        binary.to_string_lossy().as_ref(),
        project_root.to_string_lossy().as_ref(),
        &cfg.address,
        cfg.port,
        &format!("{:?}", cfg.log_level).to_lowercase(),
    )?;

    write_vpn_script(&vpn_script)?;

    set_executable(&server_script)?;
    set_executable(&vpn_script)?;

    write_desktop_file(
        &server_desktop,
        "Haven VPN Server",
        &terminal.name,
        &terminal.exec_flag,
        &server_script,
    )?;

    write_desktop_file(
        &vpn_desktop,
        "Haven VPN Connect",
        &terminal.name,
        &terminal.exec_flag,
        &vpn_script,
    )?;

    println!("Autostart configured:");
    println!("  {}", server_script);
    println!("  {}", vpn_script);
    println!("  {}", server_desktop);
    println!("  {}", vpn_desktop);

    Ok(())
}

fn write_server_script(
    path: &str,
    binary: &str,
    project_root: &str,
    address: &str,
    port: u16,
    log_level: &str,
) -> Result<(), String> {
    let content = format!(
        "#!/bin/bash\n\
         \n\
         echo \"[haven] Waiting for network (up to 60 s)...\"\n\
         DEADLINE=$((SECONDS + 60))\n\
         while [ $SECONDS -lt $DEADLINE ]; do\n\
         \x20\x20\x20\x20if ip route get 8.8.8.8 >/dev/null 2>&1; then\n\
         \x20\x20\x20\x20\x20\x20\x20\x20echo \"[haven] Network available.\"\n\
         \x20\x20\x20\x20\x20\x20\x20\x20break\n\
         \x20\x20\x20\x20fi\n\
         \x20\x20\x20\x20echo \"[haven] No route yet, retrying in 3 s...\"\n\
         \x20\x20\x20\x20sleep 3\n\
         done\n\
         if ! ip route get 8.8.8.8 >/dev/null 2>&1; then\n\
         \x20\x20\x20\x20echo \"[haven] WARNING: network not available after 60 s, starting anyway.\"\n\
         fi\n\
         \n\
         cd \"{project_root}\" || {{\n\
         \x20\x20\x20\x20echo \"[haven] ERROR: cannot cd to {project_root}\"\n\
         \x20\x20\x20\x20exit 1\n\
         }}\n\
         \n\
         exec \"{binary}\" --address {address} --port {port} --log-level {log_level}\n",
        project_root = project_root,
        binary       = binary,
        address      = address,
        port         = port,
        log_level    = log_level,
    );
    std::fs::write(path, content)
        .map_err(|e| format!("Cannot write {}: {}", path, e))
}

fn write_vpn_script(path: &str) -> Result<(), String> {
    let content =
        "#!/bin/bash\n\
         \n\
         echo \"[haven-vpn] Waiting for internet connectivity (up to 90 s)...\"\n\
         DEADLINE=$((SECONDS + 90))\n\
         while [ $SECONDS -lt $DEADLINE ]; do\n\
         \x20\x20\x20\x20if ping -c1 -W1 8.8.8.8 >/dev/null 2>&1; then\n\
         \x20\x20\x20\x20\x20\x20\x20\x20echo \"[haven-vpn] Internet available.\"\n\
         \x20\x20\x20\x20\x20\x20\x20\x20break\n\
         \x20\x20\x20\x20fi\n\
         \x20\x20\x20\x20echo \"[haven-vpn] No internet yet, retrying in 3 s...\"\n\
         \x20\x20\x20\x20sleep 3\n\
         done\n\
         if ! ping -c1 -W1 8.8.8.8 >/dev/null 2>&1; then\n\
         \x20\x20\x20\x20echo \"[haven-vpn] WARNING: internet not available after 90 s, attempting connect anyway.\"\n\
         fi\n\
         \n\
         echo \"[haven-vpn] Connecting to VPN...\"\n\
         adguardvpn-cli connect\n\
         \n\
         echo \"\"\n\
         echo \"[haven-vpn] Press Enter to close...\"\n\
         read\n";
    std::fs::write(path, content)
        .map_err(|e| format!("Cannot write {}: {}", path, e))
}

fn write_desktop_file(
    path: &str,
    name: &str,
    term: &str,
    exec_flag: &str,
    script: &str,
) -> Result<(), String> {
    let content = format!(
        "[Desktop Entry]\n\
         Type=Application\n\
         Name={name}\n\
         Exec=\"{term}\" \"{exec_flag}\" \"{script}\"\n\
         Hidden=false\n\
         X-GNOME-Autostart-enabled=true\n",
        name      = name,
        term      = term,
        exec_flag = exec_flag,
        script    = script,
    );
    std::fs::write(path, content)
        .map_err(|e| format!("Cannot write {}: {}", path, e))
}

fn set_executable(path: &str) -> Result<(), String> {
    use std::os::unix::fs::PermissionsExt;
    let perms = std::fs::Permissions::from_mode(0o755);
    std::fs::set_permissions(path, perms)
        .map_err(|e| format!("Cannot chmod {}: {}", path, e))
}

