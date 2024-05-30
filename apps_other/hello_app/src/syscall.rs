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
         # 保存当前栈指针
        addi    s0, sp, 16     # 将当前栈指针保存到s0寄存器中

        li      t0, {abi_num}  # 加载abi_num 到t0
        slli    t0, t0, 3      # 左移3位,t0*8,形成ABI_OFFSET
        add     t1, a7, t0     # ABI_ADDR=ABI_TABLE + ABI_OFFSET
        ld      t1, (t1)       # t1=ABI_ADDR
        jalr    t1             # 跳转到t1 所指定的位置,返回地址保存在 x1(ra) 中
        
        # 恢复当前栈指针
        addi    sp, s0, 0       # 将保存在s0中的栈指针恢复到sp寄存器
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
    
// 在这里我尝试保存了所有能保存的，也是这个问题
pub fn putchar1(c: char)->isize {
    // syscall(SYS_PUTCHAR, [0, c as usize , 0]);
    let mut ret:isize=0;

    unsafe {
        core::arch::asm!("
        # 保存被调用者保存寄存器的状态
        addi    sp, sp, -224    # 分配空间保存所有必要的寄存器
        sw      ra, 0  (sp)     # 保存返回地址到栈上
        sw      s0, 8  (sp)     # 保存s0到栈上
        sw      s1, 16 (sp)     # 保存s1到栈上
        sw      s2, 24 (sp)     # 保存s2到栈上
        sw      s3, 32 (sp)     # 保存s3到栈上
        sw      s4, 40 (sp)     # 保存s4到栈上
        sw      s5, 48 (sp)     # 保存s5到栈上
        sw      s6, 56 (sp)     # 保存s6到栈上
        sw      s7, 64 (sp)     # 保存s7到栈上
        sw      s8, 72 (sp)     # 保存s8到栈上
        sw      s9, 80 (sp)     # 保存s9到栈上
        sw      s10,88 (sp)     # 保存s10到栈上
        sw      s11,96 (sp)     # 保存s11到栈上

        sw      t0 ,104(sp)
        sw      t1 ,112(sp)
        sw      t2 ,120(sp)
        sw      t3 ,128(sp)
        sw      t4 ,136(sp)
        sw      t5 ,144(sp)
        sw      t6 ,152(sp)

        sw      a0 ,160(sp)
        sw      a1 ,168(sp)
        sw      a2 ,176(sp)
        sw      a3 ,184(sp)
        sw      a4 ,192(sp)
        sw      a5 ,200(sp)
        sw      a6 ,208(sp)
        sw      a7 ,216(sp)

        
        li      t0, {abi_num}
        slli    t0, t0, 3
        add     t1, a7, t0
        ld      t1, (t1)
        jalr    t1


        # 恢复函数上下文
        lw      ra, 0  (sp) 
        lw      s0, 8  (sp)    
        lw      s1, 16 (sp)    
        lw      s2, 24 (sp)    
        lw      s3, 32 (sp)    
        lw      s4, 40 (sp)    
        lw      s5, 48 (sp)    
        lw      s6, 56 (sp)    
        lw      s7, 64 (sp)    
        lw      s8, 72 (sp)    
        lw      s9, 80 (sp)    
        lw      s10,88 (sp)    
        lw      s11,96 (sp)    

        lw      t0 ,104(sp)
        lw      t1 ,112(sp)
        lw      t2 ,120(sp)
        lw      t3 ,128(sp)
        lw      t4 ,136(sp)
        lw      t5 ,144(sp)
        lw      t6 ,152(sp)

        lw      a0 ,160(sp)
        lw      a1 ,168(sp)
        lw      a2 ,176(sp)
        lw      a3 ,184(sp)
        lw      a4 ,192(sp)
        lw      a5 ,200(sp)
        lw      a6 ,208(sp)
        lw      a7 ,216(sp)
        addi    sp, sp, 224  # 恢复栈指针
        ",
            abi_num = const SYS_PUTCHAR,
            inlateout("a0") c as usize => ret,
            // clobber_abi("C")  // 指定 clobber_abi 属性为 "C"
            // options(noreturn),  //如果设置 options(noreturn) 那么不会返回
        );

        // core::arch::asm!(
        //     " wfi
        //       wfi
        //     ",
        //      // options(noreturn)
        //  );
        
    };

    ret
}


pub fn putchar(c: char)->isize {
    // syscall(SYS_PUTCHAR, [0, c as usize , 0]);
    let mut ret:isize=0;

    unsafe {
        core::arch::asm!("
        addi sp, sp, -16*8
        sd ra, 120(sp)
        sd t0, 112(sp)
        sd t1, 104(sp)
        sd t2, 96(sp)
        sd t3, 88(sp)
        sd t4, 80(sp)
        sd t5, 72(sp)
        sd t6, 64(sp)
        sd a0, 56(sp)
        sd a1, 48(sp)
        sd a2, 40(sp)
        sd a3, 32(sp)
        sd a4, 24(sp)
        sd a5, 16(sp)
        sd a6, 8(sp)
        sd a7, 0(sp)


        #addi sp, sp, -32
	    #sw t0, 0(sp)
	    #sw t1, 8(sp)
	    #sw a7, 16(sp)
	    #sw ra, 24(sp)
        # 将当前栈指针保存到s0寄存器中
        # mv s0 ,sp    
        
        li      t0, {abi_num}
        slli    t0, t0, 3
        add     t1, a7, t0
        ld      t1, (t1)
        jalr    t1
        //jalr是自动把pc+4保存到ra吗，嗯

        # 将保存在s0中的栈指针恢复到sp寄存器
        # mv sp, s0 
        //最后执行的指令是什么
        //稍等

        #lw t0, 0(sp)
	    #lw t1, 8(sp)
	    #lw a7, 16(sp)
	    #lw ra, 24(sp)
	    #addi sp, sp, 32


        ld ra, 120(sp)
        ld t0, 112(sp)
        ld t1, 104(sp)
        ld t2, 96(sp)
        ld t3, 88(sp)
        ld t4, 80(sp)
        ld t5, 72(sp)
        ld t6, 64(sp)
        ld a0, 56(sp)
        ld a1, 48(sp)
        ld a2, 40(sp)
        ld a3, 32(sp)
        ld a4, 24(sp)
        ld a5, 16(sp)
        ld a6, 8(sp)
        ld a7, 0(sp)
        addi sp, sp, 16*8

        ",
            abi_num = const SYS_PUTCHAR,
            inlateout("a0") c as usize => ret,
            clobber_abi("C")  // 指定 clobber_abi 属性为 "C"
            // options(noreturn),  //如果设置 options(noreturn) 那么不会返回
        );

        // core::arch::asm!(
        //     " wfi
        //       wfi
        //     ",
        //      // options(noreturn)
        //  );
        
    };

    ret
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
