#![feature(asm_const)]
#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]


#[macro_use]
#[cfg(feature = "axstd")]
extern crate axstd as std;


// #[cfg(feature = "axstd")]
// use axstd::print;
// #[cfg(feature = "axstd")]
// use axstd::println;

mod abi;
use abi::{ABI_TABLE,abi_hello,abi_putchar,abi_terminate,register_abi,SYS_HELLO,SYS_PUTCHAR,SYS_TERMINATE};

const PLASH_START: usize = 0x22000000;
// app running aspace
// SBI(0x80000000) -> App <- Kernel(0x80200000)
// 0xffff_ffc0_0000_0000
const RUN_START: usize = 0xffff_ffc0_8010_0000;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let apps_start = PLASH_START as *const u8;
    /*
    let apps_size = 32; // Dangerous!!! We need to get accurate size of apps.
    println!("Load payload ...");
    let code = unsafe { core::slice::from_raw_parts(apps_start, apps_size) };
    println!("content: {:#x}", bytes_to_usize(&code[..8]));
    println!("Load payload ok!");
    */

    register_abi(SYS_HELLO, abi_hello as usize);
    register_abi(SYS_PUTCHAR, abi_putchar as usize);
    register_abi(SYS_TERMINATE, abi_terminate as usize);


    // app 在文件中的偏移
    let mut offset = 0;
    let mut index = 0;
    while let Some(app) = load_app(unsafe { apps_start.offset(offset) }) {
        // 应用长度=2字节魔数 +2字节长度+ 内容长度
        offset += app.len() as isize + 4;
        copy_app(app, RUN_START);
        // run_apps(index);
        run_apps_with_abi(index);
        index += 1;
    }
    println!("Load payload ok!");
}

#[inline]
fn bytes_to_usize(bytes: &[u8]) -> usize {
    usize::from_be_bytes(bytes.try_into().unwrap())
}

fn bytes_to_u16(bytes: &[u8]) -> u16 {
    u16::from_be_bytes(bytes.try_into().unwrap())
}

/// APP 生成格式参考 payload/makebin.sh
/// # 文件格式
/// # 字节序大端法
/// # 2字节魔数 ABCD
/// # 2字节长度
/// # 文件内容
fn load_app(start: *const u8) -> Option<&'static [u8]> {
    println!("=============================");
    //1. 读取魔数 0xABCD
    let magic_bin = unsafe { core::slice::from_raw_parts(start, 2) };
    let magic = bytes_to_u16(&magic_bin[..2]);
    println!("app_magic: {:#x}", magic);

    // 可以判断魔数是否正确
    if magic != 0xABCD {
        println!("no more apps find !!! ");
        return None;
    }
    //2. 读取app size
    let size_bin = unsafe { core::slice::from_raw_parts(start.offset(2), 2) };
    let size = bytes_to_u16(&size_bin[..2]) as usize;
    println!("app_size: {:#x}", size);
    //3. 读取app 内容
    let code = unsafe { core::slice::from_raw_parts(start.offset(4), size) };
    // 十六进制表示
    println!("app_content:");
    for &byte in code {
        print!("{:02X} ", byte);
    }
    println!();
    println!("load code {:?}; address [{:?}]", code, code.as_ptr());
    Some(code)
}

/// 拷贝app 到目的地址
fn copy_app(app_bytes: &[u8], to_addr: usize) {
    let run_code = unsafe { core::slice::from_raw_parts_mut(to_addr as *mut u8, app_bytes.len()) };
    run_code.copy_from_slice(app_bytes);
    println!(
        "run  code {:?}; address [{:?}]",
        run_code,
        run_code.as_ptr()
    );
}

fn run_apps(index: isize) {
    println!("Execute app {} ...", index);
    unsafe {
        core::arch::asm!("
        li      t2, {run_start}
        jalr    ra, t2,  0 
        # j       .  
        # ret      ",
            run_start = const RUN_START,
        )
    }

    /* 
    match index {
        0=>unsafe {
            core::arch::asm!("
            li      t2, {run_start}
            jalr    ra, t2,  0 
            # j       .  
            # ret      ",
                run_start = const RUN_START,
            )
        },
        1=>unsafe {
            core::arch::asm!("
            li      t2, {run_start}
            jalr    t2
            j       .",
            run_start = const RUN_START,
            )
        },
        _=>()
    }
    */
}



fn run_apps_with_abi(index: isize) {
    println!("Execute app {} ...", index);
    let arg0: u8 = b'A';
    unsafe { core::arch::asm!("
        li      t0, {abi_num}
        slli    t0, t0, 3
        la      t1, {abi_table}
        add     t1, t1, t0
        ld      t1, (t1)
        jalr    t1
        li      t2, {run_start}
        jalr    t2
        j       .",
        run_start = const RUN_START,
        abi_table = sym ABI_TABLE,
        // abi_num = const SYS_HELLO,
        // abi_num = const SYS_PUTCHAR,
        abi_num= const SYS_TERMINATE,
        in("a0") arg0,
    )}
}