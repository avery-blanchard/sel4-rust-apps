#![no_std]


pub type c_int = i32; // hardcoded, this might depend

pub type word_t = int<16>; // again, hardcoded. 

typedef word_t seL4_Word;

#[repr(transparent)]

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Bitfield<T, U> {
    inner: T,
    _phantom: PhantomData<U>,
}

impl<T, U> Bitfield<T, U> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            _phantom: PhantomData,
        }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }

    pub fn inner(&self) -> &T {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

pub(crate) type SeL4Bitfield<T, const N: usize> = Bitfield<[T; N], T>;

impl seL4_MessageInfo {
    pub(crate) fn from_word(word: seL4_Word) -> Self {
        Self(SeL4Bitfield::new([word]))
    }

    pub(crate) fn into_word(self) -> seL4_Word {
        self.0.into_inner()[0]
    }
}

pub fn sys_id_to_word(sys_id: c_int) -> seL4_Word {
    sys_id as seL4_Word
}

pub fn sys_recv(
    sys: c_int,
    src: seL4_Word,
    out_mr0: &mut seL4_Word,
    out_mr1: &mut seL4_Word,
    out_mr2: &mut seL4_Word,
    out_mr3: &mut seL4_Word,
    reply: seL4_Word,
) -> (seL4_MessageInfo, seL4_Word) {
    let out_info: seL4_Word;
    let out_badge: seL4_Word;
    unsafe {
        asm!(
            "mov r14, rsp",
            "syscall",
            "mov rsp, r14",
            in("rdx") sys_id_to_word(sys),
            inout("rdi") src => out_badge,
            out("rsi") out_info,
            out("r10") *out_mr0,
            out("r8") *out_mr1,
            out("r9") *out_mr2,
            out("r15") *out_mr3,
            in("r12") reply,
            lateout("rcx") _,
            lateout("r11") _,
            lateout("r14") _,
        );
    }
    (seL4_MessageInfo::from_word(out_info), out_badge)
}

