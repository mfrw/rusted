#![feature(asm)]

const SSIZE: isize = 48;

#[derive(Debug, Default)]
#[repr(C)]
struct ThreadContext {
    rsp: u64,
}

fn hello() -> ! {
    println!("I Love waking up on a new stack!");
    loop {}
}

unsafe fn gt_switch(new: *const ThreadContext) {
    asm!("
    mov 0x00($0), %rsp
    ret
    "
    :
    : "r"(new)
    :
    : "alignstack"
    );
}

pub fn main() {
    let mut ctx = ThreadContext::default();
    let mut stack = vec![0_u8; SSIZE as usize];
    let stack_ptr = stack.as_mut_ptr();

    unsafe {
        std::ptr::write(stack_ptr.offset(SSIZE - 16) as *mut u64, hello as u64);
        ctx.rsp = stack_ptr.offset(SSIZE - 16) as u64;
        gt_switch(&mut ctx);
    }
}
