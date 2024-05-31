#![feature(asm_const)]
#![no_std]
#![no_main]

mod syscall;



#[no_mangle]
unsafe extern "C" fn _start(){

    const SYS_HELLO: usize = 1;
    const SYS_PUTCHAR: usize = 2;
    const SYS_TERMINATE: usize = 3;
    


    syscall::hello();
    
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

    syscall::puts("LAB4 PUTS TEST! \n");
    syscall::exit(0);
    

}



use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
