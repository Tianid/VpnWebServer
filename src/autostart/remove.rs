pub fn remove() -> Result<(), String> {
    let home = super::resolve_home()?;

    let paths = [
        format!("{}/.local/bin/haven-autostart.sh",          home),
        format!("{}/.local/bin/haven-vpn-connect.sh",        home),
        format!("{}/.config/autostart/haven-server.desktop", home),
        format!("{}/.config/autostart/haven-vpn.desktop",    home),
    ];

    let mut removed = 0usize;
    for path in &paths {
        match std::fs::remove_file(path) {
            Ok(()) => {
                println!("Removed: {}", path);
                removed += 1;
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(format!("Failed to remove {}: {}", path, e)),
        }
    }

    if removed == 0 {
        println!("Autostart is not configured.");
    }

    Ok(())
}
