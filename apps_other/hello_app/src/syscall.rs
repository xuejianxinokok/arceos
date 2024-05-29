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
        
        # addi sp, sp, -16
	    # sw t0, 0(sp)
	    # sw t1, 4(sp)
	    # sw a7, 8(sp)
	    # sw ra, 12(sp)

        # 保存被调用者保存寄存器的状态
        #addi    sp, sp, -104    # 分配空间保存所有必要的寄存器，包括sp
        #sw      ra, 96(sp)      # 保存返回地址到栈上
        #sw      s0, 88(sp)      # 保存s0到栈上
        #sw      s1, 80(sp)      # 保存s1到栈上
        #sw      s2, 72(sp)      # 保存s2到栈上
        #sw      s3, 64(sp)      # 保存s3到栈上
        #sw      s4, 56(sp)      # 保存s4到栈上
        #sw      s5, 48(sp)      # 保存s5到栈上
        #sw      s6, 40(sp)      # 保存s6到栈上
        #sw      s7, 32(sp)      # 保存s7到栈上
        #sw      s8, 24(sp)      # 保存s8到栈上
        #sw      s9, 16(sp)      # 保存s9到栈上
        #sw      s10, 8(sp)      # 保存s10到栈上
        ## 保存sp寄存器
        #sw      sp, 0(sp)       # 保存sp到栈上


        li      t0, {abi_num}
        slli    t0, t0, 3
        add     t1, a7, t0
        ld      t1, (t1)
        jar    t1


        

        # lw t0, 0(sp)
	    # lw t1, 4(sp)
	    # lw a7, 8(sp)
	    # lw ra, 12(sp)
	    # addi sp, sp, 16


        # 恢复函数上下文
        #lw      sp, 0(sp)       # 恢复sp寄存器
        #lw      ra, 96(sp)      # 恢复返回地址
        #lw      s0, 88(sp)      # 恢复s0
        #lw      s1, 80(sp)      # 恢复s1
        #lw      s2, 72(sp)      # 恢复s2
        #lw      s3, 64(sp)      # 恢复s3
        #lw      s4, 56(sp)      # 恢复s4
        #lw      s5, 48(sp)      # 恢复s5
        #lw      s6, 40(sp)      # 恢复s6
        #lw      s7, 32(sp)      # 恢复s7
        #lw      s8, 24(sp)      # 恢复s8
        #lw      s9, 16(sp)      # 恢复s9
        #lw      s10, 8(sp)      # 恢复s10
        #addi    sp, sp, 104     # 恢复栈指针

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
