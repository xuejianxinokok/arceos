#![feature(asm_const)]
#![no_std]
#![no_main]

mod syscall;

#[no_mangle]
unsafe extern "C" fn _start() {
    const SYS_HELLO: usize = 1;
    const SYS_PUTCHAR: usize = 2;
    const SYS_TERMINATE: usize = 3;

    // test
    // syscall::putchar('L');
    // syscall::putchar('A');
    // syscall::putchar('B');
    // syscall::putchar('4');
    // syscall::putchar(' ');
    // syscall::putchar('P');
    // syscall::putchar('U');
    // syscall::putchar('T');
    // syscall::putchar('S');
    // syscall::putchar(' ');
    // syscall::putchar('T');
    // syscall::putchar('E');
    // syscall::putchar('S');
    // syscall::putchar('T');
    // syscall::putchar('!');
    // syscall::putchar('\n');

    // lab4
    // syscall::hello();
    // syscall::puts("LAB4 PUTS TEST! \n");
    // syscall::exit(0);

    // lab5
    // LAB5 APP1
    // syscall::putchar('C');
    // LAB5 APP2
    syscall::putchar('D');
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
