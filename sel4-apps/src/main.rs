#![no_std]


pub fn sys_send_null(sys: c_int, src: seL4_Word, info_arg: seL4_MessageInfo) {
    unsafe {
        asm!(
            "mov r14, rsp",
            "syscall",
            "mov rsp, r14",
            in("rdx") sys_id_to_word(sys),
            in("rdi") src,
            in("rsi") info_arg.into_word(),
            lateout("rcx") _,
            lateout("r11") _,
            lateout("r14") _,
        );
    }
}

pub fn seL4_Signal(&mut self, dest: seL4_CPtr) {
      let msg_info = seL4_MessageInfo::new(0, 0, 0, 0);
      sys_send_null(syscall_id::Send, dest, msg_info)
}


