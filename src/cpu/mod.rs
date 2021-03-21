use crate::serial_println;

pub mod gdt;
pub mod idt;
pub mod serial;

pub const STACK_SIZE: usize = 4096;
pub static STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

pub fn init() {
    gdt::init_gdt();
    serial_println!("GDT Initialized!");
    idt::init_idt();
    serial_println!("IDT Initialized!");
    unsafe { idt::PICS.lock().initialize() };
    serial_println!("PICS Initialized!");
    x86_64::instructions::interrupts::enable();
    serial_println!("Interrupts enabled!");
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
