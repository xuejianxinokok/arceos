#![feature(asm_const)]
#![no_std]
#![no_main]

mod syscall;

use core::panic::PanicInfo;

#[no_mangle]
unsafe extern "C" fn _start(){
    // "nop"
    // "wfi",
    // "ebreak",
    // core::arch::asm!(
    //     "wfi
    //      ret ",
    //     options(noreturn)
    // )

    // let arg0: u8 = b'C';
    // core::arch::asm!("
    //     li      t0, {abi_num}
    //     slli    t0, t0, 3        #左移3位即乘以8
    //     add     t1, a7, t0
    //     ld      t1, (t1)
    //     jalr    t1
    //     wfi",
    //     abi_num = const SYS_PUTCHAR,
    //     in("a0") arg0,
    //     options(noreturn),
    // )

    syscall::hello();
    syscall::puts("lab4 puts test!");
    syscall::exit(0);
    
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
