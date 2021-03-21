#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(asm)]

use core::panic::PanicInfo;
use cpu::serial;
use stivale::{HeaderFramebufferTag, StivaleHeader};

mod cpu;

static FRAMEBUFFER_TAG: HeaderFramebufferTag = HeaderFramebufferTag::new().bpp(32);

#[link_section = ".stivale2hdr"]
#[no_mangle]
#[used]
pub static STIVALE_HDR: StivaleHeader =
    StivaleHeader::new(&cpu::STACK[cpu::STACK_SIZE - 1] as *const u8)
        .tags((&FRAMEBUFFER_TAG as *const HeaderFramebufferTag).cast());

#[no_mangle]
pub extern "C" fn _start(stivale_struct_ptr: usize) {
    cpu::init();

    let stivale_struct = unsafe { stivale::load(stivale_struct_ptr) };

    let memory_map = stivale_struct.memory_map().expect("No Memory map!");

    for map in memory_map.iter() {
        serial_println!(
            "Address: {:#x} - {:#x} | Length: {:#x} | Type {:#x}",
            map.start_address(),
            map.end_address(),
            map.size(),
            map.entry_type() as u32
        );
    }

    cpu::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("Paninc {}", info);
    loop {}
}
