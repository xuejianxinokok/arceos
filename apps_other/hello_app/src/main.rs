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
    // core::arch::asm!(
    //     " wfi
    //       nop
    //     ",
    //      // options(noreturn)
    // );
    // syscall::putchar('A');
    // syscall::putchar('B');

    // syscall::puts("lab4 puts test!\n");
    syscall::exit(0);
    

}



use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
