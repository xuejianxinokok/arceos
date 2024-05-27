#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::println;
use axstd::print;

const PLASH_START: usize = 0x22000000;

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
    let app1= parse_app(apps_start);
    let _app2=parse_app(app1);
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
fn parse_app(start :* const u8)-> * const u8{
    println!("=============================");
    // 读取魔数 0xABCD
    let magic = unsafe { core::slice::from_raw_parts(start, 2) };
    println!("app_magic: {:#x}", bytes_to_u16(&magic[..2]));
    // 可以判断魔数是否正确
    let size_bin = unsafe { core::slice::from_raw_parts(start.offset(2), 2) };
    let size=bytes_to_u16(&size_bin[..2]) as usize;
    println!("app_size: {:#x}", size);

    let code = unsafe { core::slice::from_raw_parts(start.offset(4), size ) };
    // 输出切片的十六进制表示
    println!("app_content:");
    for &byte in code {
        print!("{:02X} ", byte);
    }
    println!();
    // 返回下一个app地址
    unsafe{start.offset(4 +size as isize) }
    
}