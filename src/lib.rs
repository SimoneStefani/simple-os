// File attributes
#![feature(lang_items)]
#![feature(const_fn)] // Static initializers
#![feature(unique)]
#![feature(const_unique_new)]
#![no_std] // Don't link std lib

extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;

#[macro_use]
mod vga_buffer;

// Disable name mangling to following fn
// since we want to call it from assembly
#[no_mangle]
pub extern fn rust_main(multiboot_information_address: usize) {
    
    vga_buffer::clear_screen();
    println!("Hello World{}", "!");

    let boot_info = unsafe{ multiboot2::load(multiboot_information_address) };
    let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");

    println!("memory areas:");
    for area in memory_map_tag.memory_areas() {
        println!("    start: 0x{:x}, length: 0x{:x}", area.base_addr, area.length);
    }

    loop{}
    
}

// Handle unwinding and panic
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}