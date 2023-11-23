// #[global_allocator]
// static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::ffi::CStr;
use std::os::raw::c_char;
use std::time::Duration;
use std::thread::sleep;
use winapi::um::{
    winuser::{GetAsyncKeyState, GetForegroundWindow, GetWindowTextA, INPUT, INPUT_MOUSE, MOUSEEVENTF_MOVE, SendInput},
    shellapi::ShellExecuteW
};
use encoding_rs::GBK;
use widestring::U16CString;
use winput::{Button, press, release};

// Constants for window title, key code, mouse movement and window size
const TARGET_WINDOW_TITLE: &str = "原神"; // The name of the game we want to target
const KEY_CODE: i32 = 192; // ` The key code of the ` key, which will trigger the mouse movement
const MOUSE_DX: i32 = -2000; // move 2000 pixels to the left The horizontal distance to move the mouse cursor
const MOUSE_DY: i32 = 0; // The vertical distance to move the mouse cursor (zero means no movement)
const WINDOW_SIZE: i32 = 256; // The maximum size of the window title in bytes

// Function to move the mouse cursor
#[inline(always)]
fn move_mouse_left() {
    let mut input = INPUT {
        type_: INPUT_MOUSE,
        u: unsafe { std::mem::zeroed() },
    };
    unsafe {
        input.u.mi_mut().dx = MOUSE_DX;
        input.u.mi_mut().dy = MOUSE_DY;
        input.u.mi_mut().dwFlags = MOUSEEVENTF_MOVE;
        input.u.mi_mut().time = 0;
        input.u.mi_mut().dwExtraInfo = 0;
        SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32); // Send the mouse input to the system
    }
}

// Function to check if the current process has admin privilege
fn has_admin_privilege() -> bool {
    std::fs::metadata("C:\\Windows\\System32\\config\\systemprofile").is_ok()
}

// Function to launch a new process with admin privilege
fn launch_new_process_with_admin_privilege() {
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
#[inline(always)]
fn get_window_title() -> String {
    let mut window_title = [0 as c_char; WINDOW_SIZE as usize]; // Create a buffer to store the window title
    unsafe {
        GetWindowTextA(GetForegroundWindow(), window_title.as_mut_ptr(), WINDOW_SIZE); // Get the window title of the foreground window
    }
    let window_title_bytes = unsafe { CStr::from_ptr(window_title.as_ptr()) }
        .to_bytes(); // Convert the buffer to a byte slice
    
    let mut decoder = GBK.new_decoder(); // Create a decoder for GBK encoding
    let mut window_title = String::with_capacity(WINDOW_SIZE as usize); // Create a string to store the decoded window title
    let _ = decoder.decode_to_string(window_title_bytes, &mut window_title, true); // Decode the window title to the string
    window_title
}

fn main() {

    // Check if the current process has admin privilege
    if !has_admin_privilege() {
        // If not, launch a new process with admin privilege
        launch_new_process_with_admin_privilege();
        return;
    }

    println!("When the window title is \"{}\" and the ` key is pressed, the cursor will keep moving to the left.", TARGET_WINDOW_TITLE);

    let mut handle:winapi::shared::windef::HWND = std::ptr::null_mut(); // Create a null handle to store the target window handle
    
    loop {

        let window = unsafe{GetForegroundWindow()}; // Get the handle of the foreground window
        if unsafe { GetAsyncKeyState(KEY_CODE) == -32767 } { // Check if the ` key is pressed
            if handle.is_null() && !window.is_null() { // If we don't have a target window handle and the foreground window is not null
                let current_window_title = get_window_title(); // Get the window title of the foreground window
    
                if current_window_title == TARGET_WINDOW_TITLE { // If the window title matches the target window title
                    handle = window; // Set the target window handle to the foreground window handle
                }
            }
            if handle == window { // If the target window handle is the same as the foreground window handle
                press(Button::Left);
                sleep(Duration::from_millis(300));
                release(Button::Left);
                while matches!(unsafe { GetAsyncKeyState(KEY_CODE) }, -32767 | -32768) {
                    move_mouse_left(); // Move the mouse cursor to the left
                    sleep(Duration::from_millis(10)); // Sleep for 10 milliseconds
                }
            }
        }
        sleep(Duration::from_millis(100)); // Sleep for 100 milliseconds
    }
}
