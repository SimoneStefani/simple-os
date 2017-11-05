// File attributes
#![feature(lang_items)]
#![no_std] // Don't link std lib

// Disable name mangling to following fn
// since we want to call it from assembly
#[no_mangle]
pub extern fn rust_main() {}

// Handle unwinding and panic
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}