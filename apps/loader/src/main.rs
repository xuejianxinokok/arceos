#![feature(asm_const)]
#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::println;
#[cfg(feature = "axstd")]
use axstd::print;

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

    let mut offset=0;
    // let mut run_off_set=0;
    while let Some(app)  =  load_app( unsafe {apps_start.offset( offset)})  {
        // 应用长度=2字节魔数 +2字节长度+ 内容长度
        offset+=app.len() as isize +4;
        //拷贝app 到目的地址
        // println!("run_off_set:{}",run_off_set);
        // copy_app(app,RUN_START+run_off_set);
        copy_app(app,RUN_START);
        run_apps();
        // run_off_set+=app.len();
        
    } 

    // run_apps();
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
fn load_app(start :* const u8)-> Option< &'static [u8]> {
    println!("=============================");
    // 读取魔数 0xABCD
    let magic_bin = unsafe { core::slice::from_raw_parts(start, 2) };
    let magic=bytes_to_u16(&magic_bin[..2]);
    println!("app_magic: {:#x}", magic);

    if magic!=0xABCD {
        println!("no more apps find !!! ");
        return  None;
    }

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
    // unsafe{start.offset(4 +size as isize) }
    println!("load code {:?}; address [{:?}]", code, code.as_ptr());
    Some(code)
}

/// 拷贝app 到目的地址
fn copy_app(app_bytes: &[u8],to_addr:usize){
    let run_code = unsafe {
        core::slice::from_raw_parts_mut(to_addr as *mut u8, app_bytes.len())
    };
    run_code.copy_from_slice(app_bytes);
    println!("run  code {:?}; address [{:?}]", run_code, run_code.as_ptr());
}


fn run_apps(){

    println!("Execute app ...");
    // execute app
    unsafe { core::arch::asm!("
        li      t2, {run_start}
        jalr    t2
        # j      .
        ret ",
        run_start = const RUN_START,
    )}
}