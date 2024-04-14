#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(decl_macro)]

use core::panic::PanicInfo;
use core::arch::asm;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub type CInt = i32;

pub type SeL4Word = i32;


pub fn main() {
    unsafe {
	let sys = 3; // system call numbers are generated when the kernel is built, hardcoded
        let out_badge: SeL4Word;
        let out_info: SeL4Word;
        let mut mr0 = 0;
        let mut mr1 = 0;
        let mut mr2 = 0;
        let mut mr3 = 0;

        asm!(	
            "mov r14, rsp",
            "syscall",
            "mov rsp, r14",
            in("rdx") sys,
            inout("rdi") 0 => out_badge,
            out("rsi") out_info,
            out("r10") mr0,
            out("r8") mr1,
            out("r9") mr2,
            out("r15") mr3,
            in("r12") 0,
            lateout("rcx") _,
            lateout("r11") _,
            lateout("r14") _,
            );
    }

}
