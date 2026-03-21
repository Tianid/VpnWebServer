pub struct Terminal {
    pub name:       String,
    pub exec_flag:  String,
}

pub fn detect() -> Option<Terminal> {
    let candidates = [
        ("x-terminal-emulator", "-e"),
        ("lxterminal",          "-e"),
        ("xfce4-terminal",      "-e"),
        ("gnome-terminal",      "--"),
        ("mate-terminal",       "-e"),
        ("konsole",             "-e"),
        ("xterm",               "-e"),
    ];

    for (name, flag) in &candidates {
        if which_exists(name) {
            return Some(Terminal {
                name:      name.to_string(),
                exec_flag: flag.to_string(),
            });
        }
    }
    None
}

fn which_exists(name: &str) -> bool {
    std::process::Command::new("which")
        .arg(name)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}
