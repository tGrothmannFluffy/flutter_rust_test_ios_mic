use crate::audio_test::debug_log::{debug_log_string, dequeue_log_message};

#[flutter_rust_bridge::frb(sync)]
pub fn greet(name: String) -> String {
    debug_log_string(name.clone());
    format!("Hello, {name}!")
}

#[flutter_rust_bridge::frb(sync)]
pub fn dequeue_debug_message() -> Option<String> {
    dequeue_log_message()
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
