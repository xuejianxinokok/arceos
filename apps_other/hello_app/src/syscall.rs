const SYS_HELLO: usize = 1;
const SYS_PUTCHAR: usize = 2;
const SYS_TERMINATE: usize = 3;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret;
    unsafe {
        // ld      t0, {abi_num}
        core::arch::asm!(
           "slli    t0, t0, 3      # 左移3位,t0*8,形成ABI_OFFSET 
            add     t1, a7, t0     # ABI_ADDR=ABI_TABLE + ABI_OFFSET
            ld      t1, (t1)       # t1=ABI_ADDR
            jalr    t1             # 跳转到t1 所指定的位置,返回地址保存在 x1(ra)
            ret 
            ",
            inlateout("a0") args[0] => ret,
            in("a1") args[1],
            in("a2") args[2],
            in("t0") id,
        );
    }
    ret
}

pub fn hello() {
    // syscall(SYS_HELLO, [0, 0, 0]);
    
    unsafe {
        core::arch::asm!(" 
        addi sp, sp, -16
	    sw t0, 0(sp)
	    sw t1, 4(sp)
	    sw a7, 8(sp)
	    sw ra, 12(sp)

        li      t0, {abi_num}  # 加载abi_num 到t0
        slli    t0, t0, 3      # 左移3位,t0*8,形成ABI_OFFSET
        add     t1, a7, t0     # ABI_ADDR=ABI_TABLE + ABI_OFFSET
        ld      t1, (t1)       # t1=ABI_ADDR
        jalr    t1             # 跳转到t1 所指定的位置,返回地址保存在 x1(ra) 中
        
        lw t0, 0(sp)
	    lw t1, 4(sp)
	    lw a7, 8(sp)
	    lw ra, 12(sp)
	    addi sp, sp, 16
        ",
            abi_num = const SYS_HELLO,
            // options(noreturn),
        );
    };
}   
    

pub fn putchar(c: char)->() {
    // syscall(SYS_PUTCHAR, [0, c as usize , 0]);
    unsafe {
        core::arch::asm!("
        
        addi sp, sp, -16
	    sw t0, 0(sp)
	    sw t1, 4(sp)
	    sw a7, 8(sp)
	    sw ra, 12(sp)
         # 保存当前栈指针
        addi    s0, sp, 16     # 将当前栈指针保存到s0寄存器中

        li      t0, {abi_num}
        slli    t0, t0, 3
        add     t1, a7, t0
        ld      t1, (t1)
        jalr    t1
         # 恢复当前栈指针
        addi    sp, s0, 0       # 将保存在s0中的栈指针恢复到sp寄存器
        lw t0, 0(sp)
	    lw t1, 4(sp)
	    lw a7, 8(sp)
	    lw ra, 12(sp)
	    addi sp, sp, 16
        ",
            abi_num = const SYS_PUTCHAR,
            in("a0") c as usize,
            // options(noreturn),  //如果设置 options(noreturn) 那么不会返回
        );

        // core::arch::asm!(
        //     " wfi
        //       wfi
        //     ",
        //      // options(noreturn)
        //  );
        
    };
    ()
}

pub fn puts(s: &str) {
    for c in s.chars() {
        putchar(c);
    }
}

pub fn exit(xstate: isize) -> isize {
    //syscall(SYS_TERMINATE, [xstate as usize, 0, 0]);
    //xstate
    
    unsafe {
        core::arch::asm!("
        li      t0, {abi_num}
        slli    t0, t0, 3
        add     t1, a7, t0
        ld      t1, (t1)
        jalr    t1 ",
            abi_num = const SYS_TERMINATE,
            in("a0") xstate,
            options(noreturn),
        )
    }
    
}
