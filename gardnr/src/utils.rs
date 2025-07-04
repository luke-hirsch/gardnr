use std::process::Command;

pub fn is_installed(cmds: &[&str]) -> Option<String> {
    for cmd in cmds {
        // On Unix-like systems, use "which" to check if command exists
        #[cfg(unix)]
        let status = Command::new("which")
            .arg(cmd)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false);

        if status {
            return Some(cmd.to_string());
        }
    }
    None
}

/// Language-specific executable names
pub struct TechExecutables<'a> {
    pub python: &'a [&'a str],
    pub node: &'a [&'a str],
    pub java: &'a [&'a str],
    pub ruby: &'a [&'a str],
    pub go: &'a [&'a str],
    pub php: &'a [&'a str],
    pub swift: &'a [&'a str],
    pub rust: &'a [&'a str],
}

/// Default executable names for various tech stacks
pub const DEFAULT_EXECUTABLES: TechExecutables<'static> = TechExecutables {
    python: &["python3", "python"],
    node: &["node"],
    java: &["javac"],
    ruby: &["ruby"],
    go: &["go"],
    php: &["php"],
    swift: &["swiftc"],
    rust: &["rustc"],
};
