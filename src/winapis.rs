use std::ffi::CStr;
use std::os::raw::c_char;
use std::time::Duration;
use winapi::um::{
    winuser::{GetAsyncKeyState, GetForegroundWindow, GetWindowTextA, INPUT, INPUT_MOUSE, MOUSEEVENTF_MOVE, SendInput},
    shellapi::ShellExecuteW
};
use encoding_rs::GBK;
use widestring::U16CString;
use winput::{Button, press, release};

use super::Config;

pub fn sleep(millis: u64) {
    std::thread::sleep(Duration::from_millis(millis))
}

// Function to move the mouse cursor
pub fn move_mouse(config: &Config) {
    let mut input = INPUT {
        type_: INPUT_MOUSE,
        u: unsafe { std::mem::zeroed() },
    };
    unsafe {
        input.u.mi_mut().dx = config.mouse_dx;
        input.u.mi_mut().dy = config.mouse_dy;
        input.u.mi_mut().dwFlags = MOUSEEVENTF_MOVE;
        SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32); // Send the mouse input to the system
    }
}

// Function to check if the current process has admin privilege
#[inline(always)]
pub fn has_admin_privilege() -> bool {
    std::fs::metadata("C:\\Windows\\System32\\config\\systemprofile").is_ok()
}

// Function to launch a new process with admin privilege
pub fn launch_new_process_with_admin_privilege() {
    let exe_path = std::env::current_exe().unwrap(); // Get the current executable path
    let exe_path = exe_path.to_str().unwrap(); // Convert it to a string
    let exe_path = U16CString::from_str(exe_path).unwrap(); // Convert it to a wide string
    let result = unsafe {
        ShellExecuteW(
            std::ptr::null_mut(), // No parent window
            U16CString::from_str("runas").unwrap().as_ptr(), // Use the "runas" verb to request admin privilege
            exe_path.as_ptr(), // The executable path
            std::ptr::null(), // No parameters
            std::ptr::null(), // No working directory
            1, // SW_SHOWNORMAL Show the new process normally
        )
    };
    if result as usize <=32 {
        // Failure, handle the error
        eprintln!("Failed to launch a new process with admin privilege");
        std::io::stdin().read_line(&mut String::new()).unwrap(); // Wait for user input
    }
}

// Function to get the window title of the foreground window
pub fn get_window_title(config: &Config) -> String {
    let window_size = config.max_title_length;
    let window_usize = window_size as usize;
    let mut window_title = vec![0 as c_char; window_usize]; // Create a buffer to store the window title
    unsafe {
        GetWindowTextA(GetForegroundWindow(), window_title.as_mut_ptr(), window_size); // Get the window title of the foreground window
    }
    let window_title_bytes = unsafe { CStr::from_ptr(window_title.as_ptr()) }
        .to_bytes(); // Convert the buffer to a byte slice
    
    let mut decoder = GBK.new_decoder(); // Create a decoder for GBK encoding
    let mut window_title = String::with_capacity(window_usize); // Create a string to store the decoded window title
    let _ = decoder.decode_to_string(window_title_bytes, &mut window_title, true); // Decode the window title to the string
    window_title
}

pub fn key_pressed(key_code: i32) -> bool {
    matches!(unsafe { GetAsyncKeyState(key_code) }, -32767 | -32768)
}

pub fn keeps_detect_and_move(config: &Config) {
    let key = config.key;
    let target_window_title = &config.target_window_title;
    let start_time = config.start_time;
    let check_interval = config.check_interval;
    let move_interval = config.move_interval;
    let mut handle:winapi::shared::windef::HWND = std::ptr::null_mut(); // Create a null handle to store the target window handle
    loop {

        let window = unsafe{GetForegroundWindow()}; // Get the handle of the foreground window
        if key.is_down() { // Check if the ` key is pressed
            if handle.is_null() && !window.is_null() { // If we don't have a target window handle and the foreground window is not null
                let current_window_title = get_window_title(config); // Get the window title of the foreground window
    
                if current_window_title == *target_window_title { // If the window title matches the target window title
                    handle = window; // Set the target window handle to the foreground window handle
                }
            }
            if handle == window { // If the target window handle is the same as the foreground window handle
                press(Button::Left);
                sleep(start_time);
                release(Button::Left);
                while key.is_down() {
                    move_mouse(config); // Move the mouse cursor to the left
                    sleep(move_interval); // Sleep for 10 milliseconds
                }
            }
        }
        sleep(check_interval); // Sleep for 100 milliseconds
    }
}