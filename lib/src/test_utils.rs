use log::info;
use std::env;
use std::process::Command;
use std::str;
use std::sync::Once;

pub const FUNNEL_HOST: &str = "http://localhost";
pub const FUNNEL_PORT: u16 = 8000;
pub static INIT: Once = Once::new();

pub fn setup() {
    INIT.call_once(|| {
        env::set_var("RUST_LOG", "debug");
        env_logger::init();
    });
}

pub async fn ensure_funnel_running() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("ps aux | grep '[f]unnel server run'")
        .output()
        .expect("Failed to execute command");

    let output_str = str::from_utf8(&output.stdout).unwrap();

    if output_str.is_empty() {
        panic!("Funnel is not running.");
    }

    let funnel_url = format!("{}:{}", FUNNEL_HOST, FUNNEL_PORT);
    funnel_url
}
