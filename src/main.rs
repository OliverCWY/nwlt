// #[global_allocator]
// static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use nwlt::*;
use std::fs::File;
use std::io::{Read, stdin, Write};
use std::io::ErrorKind::NotFound;
use std::process;

const DEFAULT_CONFIG_FILE_PATH: &str = "./nwlt.toml";

fn get_config() -> Config {
    let mut config_file = match File::open(DEFAULT_CONFIG_FILE_PATH) {
        Ok(f) => f,
        Err(e) => {
            if e.kind() == NotFound {
                eprint!("{DEFAULT_CONFIG_FILE_PATH} 文件不存在，是否开始配置(y开始配置(默认)/n使用默认配置):");

                let config = loop {
                    let mut input = String::new();
                    stdin().read_line(&mut input).expect("Failed to read line");
                    let input = input.as_bytes().first();
                    match input {
                        Some(b'n') | Some(b'N') => {
                            break Config::default()
                        },
                        Some(b'y') | Some(b'Y') | None | Some(13) => {
                            break generate_config();
                        },
                        Some(c) => {
                            eprintln!("输入错误！{c}");
                        }
                    };
                };

                
                let config_str = toml::to_string_pretty(&config).unwrap();
                match File::create(DEFAULT_CONFIG_FILE_PATH) {
                    Ok(mut file) => {
                        if file.write_all(config_str.as_bytes()).is_err() {
                            eprintln!("{DEFAULT_CONFIG_FILE_PATH} 写入失败，请检查文件权限！");
                            process::exit(1);
                        };
                    },
                    Err(_) => {
                        eprintln!("{DEFAULT_CONFIG_FILE_PATH} 创建失败，请检查文件权限！");
                        process::exit(1);
                    }
                }
                return config
            } else {
                eprintln!("{DEFAULT_CONFIG_FILE_PATH} 打开失败，请检查文件权限！");
                process::exit(1);
            }
        }
    };
    let mut config_str = String::new(); 
    if config_file.read_to_string(&mut config_str).is_err() {
        eprintln!("{DEFAULT_CONFIG_FILE_PATH} 读取失败，请重试！");
        process::exit(1);
    }
    toml::from_str(&config_str).unwrap()
}

fn main() {

    // Check if the current process has admin privilege
    if !has_admin_privilege() {
        // If not, launch a new process with admin privilege
        launch_new_process_with_admin_privilege();
        return;
    }

    let config = get_config();

    println!("当窗口标题为 \"{}\" 并按住键{:?}时, 鼠标会持续移动.", config.target_window_title, config.key);

    keeps_detect_and_move(&config);

}