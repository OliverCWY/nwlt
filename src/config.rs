use serde::{Serialize, Deserialize};
use serde_default::DefaultFromSerde;
use winput::Vk;

#[derive(Serialize, Deserialize, DefaultFromSerde)]
pub struct Config {
    #[serde(default = "default_title")]
    pub target_window_title: String,
    #[serde(default = "default_key")]
    pub key: Vk,
    #[serde(default = "default_dx")]
    pub mouse_dx: i32,
    #[serde(default = "default_dy")]
    pub mouse_dy: i32,
    #[serde(default = "default_max_title_length")]
    pub max_title_length: i32,
    #[serde(default = "default_check_interval")]
    pub check_interval: u64,
    #[serde(default = "default_move_interval")]
    pub move_interval: u64,
    #[serde(default = "default_start_time")]
    pub start_time: u64,
}

fn default_title() -> String {
    "原神".to_string()
}

fn default_key() -> Vk {
    Vk::Oem3
}

fn default_dx() -> i32 {
    -2000
}

fn default_dy() -> i32 {
    0
}

fn default_max_title_length() -> i32 {
    256
}

fn default_check_interval() -> u64 {
    100
}

fn default_move_interval() -> u64 {
    10
}

fn default_start_time() -> u64 {
    300
}