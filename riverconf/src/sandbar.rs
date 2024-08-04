use std::io::{Write};
use std::process::{Command, Stdio};
use std::{fs, thread};
use std::time::{Duration, Instant};
use chrono::{Local};

const UPDATE_INTERVAL: Duration = Duration::from_millis(250);

const FONT: &'static str = "Inconsolata";
const ACCENT: &'static str = "ee9900";
const UNUSED_BG: &'static str = "000000";

/// Launch sandbar in a new thread.
pub fn launch_sandbar() -> thread::JoinHandle<()> {
    thread::spawn(|| {
        let mut last_instant = Instant::now();
        let mut status = Status::new();

        // TODO: style https://github.com/kolunmi/sandbar?tab=readme-ov-file#example-setup
        let mut sandbar = Command::new("sandbar")
            .arg("-font").arg(FONT)
            .arg("-active-bg-color").arg(ACCENT)
            .arg("-inactive-bg-color").arg(UNUSED_BG)
            .arg("-title-bg-color").arg(UNUSED_BG)
            .stdin(Stdio::piped())
            .spawn().unwrap();
        let mut stdin = sandbar.stdin.take().unwrap();

        while !sandbar.try_wait().is_ok_and(|s| s.is_some()) {
            let x = status.compute();

            let elapsed = last_instant.elapsed();
            if elapsed < UPDATE_INTERVAL {
                thread::sleep(UPDATE_INTERVAL - elapsed);
            }

            stdin.write(&x).unwrap();

            last_instant = Instant::now();
        }
    })
}

struct Status {
    total_ram: u64,
    last_status: String,
}

impl Status {
    fn new() -> Self {
        Status {
            total_ram: meminfo_key("MemTotal").unwrap_or(0),
            last_status: String::new(),
        }
    }
    fn compute(&mut self) -> &[u8] {
        let time = time();
        let cpu = cpu();
        let ram = self.total_ram - meminfo_key("MemAvailable").unwrap_or(0);
        let battery = fs::read_to_string("/sys/class/power_supply/BAT0/capacity").unwrap_or(String::from("?"));
        let bat_status = battery_status().unwrap_or(String::from("BAT"));
        self.last_status = format!("all status {time} - {cpu}CPU {ram}MEM - {battery}% {bat_status}");
        self.last_status.as_bytes()
    }
}

fn time() -> String {
    Local::now().format("%H:%M:%S").to_string()
}

fn cpu() -> String {
    match fs::read_to_string("/proc/loadavg") {
        Err(_) => String::from("?"),
        Ok(loadavg) => match loadavg.split_once(" ") {
            None => String::from("?"),
            Some((cpu, _)) => cpu.to_string()
        }
    }
}

/// Read the value of a key in `/proc/meminfo`.
fn meminfo_key(key: &str) -> Option<u64> {
    let meminfo = fs::read_to_string("/proc/meminfo").ok()?;
    for stat in meminfo.lines() {
        if let Some((k, v)) = stat.split_once(":") {
            if k == key {
                let v = v.trim_end_matches(" kB");
                let v = v.parse::<u64>().ok()?;
                return Some(v);
            }
        }
    }

    None
}

fn battery_status() -> Option<String> {
    let status = fs::read_to_string("/sys/class/power_supply/BAT0/status").ok()?;
    match status.as_str() {
        "Charging\n" => Some(String::from("⚡")),
        "Discharging\n" => Some(String::from("🔋")),
        "Not charging\n" | "Full\n" => Some(String::from("🔌")),
        "Unknown\n" | _ => None,
    }
}
