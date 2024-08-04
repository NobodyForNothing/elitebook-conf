use std::fs;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let xdg_runtime_dir = std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| {
        let var = format!("/tmp/{}-runtime-dir", nix::unistd::geteuid());
        std::env::set_var("XDG_RUNTIME_DIR", &var);
        var
    });
    if !PathBuf::from(&xdg_runtime_dir).exists() {
        fs::create_dir(&xdg_runtime_dir).unwrap()
    }

    let ssh_agent_out = Command::new("ssh-agent").arg("-s")
        .output().unwrap().stdout;
    let ssh_agent_out = String::from_utf8(ssh_agent_out).unwrap();
    ssh_agent_out
        .split(";")
        .map(|e| e.trim_matches(|c| c == '\n' || c == ' '))
        .map(|e| e.split_once('='))
        .filter(|e| e.is_some())
        .map(|e| e.unwrap())
        .for_each(|(k, v)| std::env::set_var(k, v));

    Command::new("dbus-daemon")
        .arg("--session")
        .arg(format!("--address=unix:path={xdg_runtime_dir}/bus"))
        .spawn().unwrap();

    // IMPORTANT: needs to exec at the end or this won't work
    Command::new("river").exec();
}
