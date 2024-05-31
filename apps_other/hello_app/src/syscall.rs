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
        

        li      t0, {abi_num}  # 加载abi_num 到t0
        slli    t0, t0, 3      # 左移3位,t0*8,形成ABI_OFFSET
        add     t1, a7, t0     # ABI_ADDR=ABI_TABLE + ABI_OFFSET
        ld      t1, (t1)       # t1=ABI_ADDR
        jalr    t1             # 跳转到t1 所指定的位置,返回地址保存在 x1(ra) 中
        
       
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
            abi_num = const SYS_HELLO,
            // options(noreturn),
        );
    };
}

pub fn putchar(c: char) -> isize {
    // syscall(SYS_PUTCHAR, [0, c as usize , 0]);
    let mut ret: isize = 0;

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
	    #sd t0, 0(sp)
	    #sd t1, 8(sp)
	    #sd a7, 16(sp)
	    #sd ra, 24(sp)
        
        
        li      t0, {abi_num}
        slli    t0, t0, 3
        add     t1, a7, t0
        ld      t1, (t1)
        jalr    t1
        
        #ld t0, 0(sp)
	    #ld t1, 8(sp)
	    #ld a7, 16(sp)
	    #ld ra, 24(sp)
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

pub fn puts(str: &str) {
    // for c in str.as_bytes() {
    //     putchar(*c as char);
    // }

    str.as_bytes().into_iter().for_each(|c| {
        putchar(*c as char);
    });
}

pub fn exit(xstate: isize) -> isize {
    //syscall(SYS_TERMINATE, [xstate as usize, 0, 0]);
    //xstate

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

        li      t0, {abi_num}
        slli    t0, t0, 3
        add     t1, a7, t0
        ld      t1, (t1)
        jalr    t1
        
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
            abi_num = const SYS_TERMINATE,
            in("a0") xstate,
            options(noreturn),
        )
    }
}
