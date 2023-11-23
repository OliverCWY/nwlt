pub mod config;

pub use config::*;

pub mod winapis;
pub use winapis::*;
use winput::Vk;

const CONFIG_THRESHOLD_MS: u64 = 2000;
const CONFIG_GENERATE_DETECT_MS: u64 = 100;

const MAX_KEY_CODE: usize = 256;

pub fn generate_config() -> Config {
    let mut key_map = [0u64; MAX_KEY_CODE];
    let interval = CONFIG_GENERATE_DETECT_MS;
    let threshold = CONFIG_THRESHOLD_MS;
    eprintln!("切换到游戏后按住触发键{}ms以上，听到哔声后完成配置", threshold);
    loop {
        for (key, time) in key_map.iter_mut().enumerate() {
            if key_pressed(key as i32) {
                *time += interval;
                if *time >= threshold {
                    let mut config = Config::default();
                    config.target_window_title = get_window_title(&config);
                    config.key = unsafe {
                        Vk::from_u8(key as u8)
                    };
                    unsafe {
                        winapi::um::utilapiset::Beep(1000, 400);
                    }
                    return config;
                }
            } else {
                *time = 0;
            }
        }
        sleep(interval)
    }
}