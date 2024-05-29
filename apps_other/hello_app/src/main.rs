#![feature(asm_const)]
#![no_std]
#![no_main]

mod syscall;



#[no_mangle]
unsafe extern "C" fn _start(){

    const SYS_HELLO: usize = 1;
    const SYS_PUTCHAR: usize = 2;
    const SYS_TERMINATE: usize = 3;
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
    //     addi sp, sp, -4
    //     sw   ra, 0(sp) 

    //     li      t0, {abi_num}
    //     slli    t0, t0, 3        #左移3位即乘以8
    //     add     t1, a7, t0
    //     ld      t1, (t1)
    //     jalr    t1
        
    //     lw ra, 0(sp)
	//     addi sp, sp, 4

    //     ",
    //     abi_num = const SYS_PUTCHAR,
    //     in("a0") arg0,
    //     // options(noreturn),
    // );


   
    syscall::putchar('A');
    syscall::putchar('A');

    // core::arch::asm!(
    //     " nop
    //       nop
    //     ",
    //      // options(noreturn)
    // );


 
   


    // syscall::hello();
    // core::arch::asm!(
    //     " wfi
    //       nop
    //     ",
    //      // options(noreturn)
    // );
    // syscall::putchar('A');

    // syscall::puts("lab4 puts test!\0");
    // syscall::exit(0);
    

}



use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
