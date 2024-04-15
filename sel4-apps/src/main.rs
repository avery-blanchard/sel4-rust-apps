#![no_std]
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
	let syscall_id: SeL4Word = 3;
        let out_badge: SeL4Word;
        let out_info: SeL4Word;
        let mut mr0 = 0;
        let mut mr1 = 0;
        let mut mr2 = 0;
        let mut mr3 = 0;


	asm!(
		// Save current stack pointer (rsp) in r14
		"mov r14, rsp",
		// Perform system call
		"syscall",
		// Restore stack pointer (rsp) from r14
		"mov rsp, r14",
		// Input syscall_id into rdx, output rdi after syscall to out_badge
		in("rdx") syscall_id,
		inout("rdi") 0 => out_badge,
		// Output rsi, r10, r8, r9, r15 after syscall
		out("rsi") out_info,
		out("r10") mr0,
		out("r8") mr1,
		out("r9") mr2,
		 out("r15") mr3,
		// Input 0 into r12 for syscall to denote reply args are unused (const UNUSED_REPLY_ARG: seL4_Word = 0;)
		in("r12") 0,
		// Output rcx, r11, r14 after syscall
		lateout("rcx") _,
		lateout("r11") _,
		lateout("r14") _,
	);
    }

}
