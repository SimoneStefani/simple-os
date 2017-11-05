// File attributes
#![feature(lang_items)]
#![feature(const_fn)] // Static initializers
#![feature(unique)]
#![feature(const_unique_new)]
#![no_std] // Don't link std lib

extern crate rlibc;
extern crate volatile;
extern crate spin;

#[macro_use]
mod vga_buffer;

// Disable name mangling to following fn
// since we want to call it from assembly
#[no_mangle]
pub extern fn rust_main() {
    
    vga_buffer::clear_screen();
    println!("Hello World{}", "!");

    loop{}
    
}

// Handle unwinding and panic
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}