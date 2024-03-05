#![no_std]


pub fn sys_send_null(sys: c_int, src: word, info_arg: msg_info) {
    unsafe {
        asm!("ecall",
            in("a7") sys_id_to_word(sys),
            in("a0") src,
            in("a1") ,
        );
    }
}

pub fn seL4_Signal(&mut self, dest: ptr) {
      let msg_info = sys::new(0, 0, 0, 0);

      sys_send_null(syscall_id::Send, dest, msg_info)
}


