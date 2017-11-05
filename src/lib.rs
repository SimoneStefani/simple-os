// File attributes
#![feature(lang_items)]
#![feature(const_fn)] // Static initializers
#![feature(unique)]
#![feature(const_unique_new)]
#![no_std] // Don't link std lib

extern crate rlibc;

#[macro_use]
mod vga_buffer;

// Disable name mangling to following fn
// since we want to call it from assembly
#[no_mangle]
pub extern fn rust_main() {
    // let hello = b"Hello World! Pew!";
    // let color_byte = 0x1f; // white foreground, blue background

    // let mut hello_colored = [color_byte; 34];
    // for (i, char_byte) in hello.into_iter().enumerate() {
    //     hello_colored[i*2] = *char_byte;
    // }

    // // write `Hello World!` to the center of the VGA text buffer
    // let buffer_ptr = (0xb8000 + 1988) as *mut _;
    // unsafe { *buffer_ptr = hello_colored };

    // loop{}
    vga_buffer::print_something();
}

// Handle unwinding and panic
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}