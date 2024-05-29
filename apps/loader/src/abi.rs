use std::process::exit;
use std::io::{Write,stdout};
pub  const SYS_HELLO: usize = 1;
pub  const SYS_PUTCHAR: usize = 2;
pub  const SYS_TERMINATE: usize = 3;

pub static mut ABI_TABLE: [usize; 16] = [0; 16];

fn register_abi(num: usize, handle: usize) {
    unsafe {
        ABI_TABLE[num] = handle;
    }
}

///   注册所有abi
pub fn register_all_abi() {
 
    register_abi(SYS_HELLO, abi_hello as usize);
    register_abi(SYS_PUTCHAR, abi_putchar as usize);
    register_abi(SYS_TERMINATE, abi_terminate as usize);
}


pub fn abi_hello() {
    stdout().write_all(b"[ABI:Hello] Hello, Apps!");
    // println!("[ABI:Hello] Hello, Apps!");
}

pub fn abi_putchar(c: char) {
    // print!("{}", c);
    // UTF-8 编码最多需要4个字节
    let mut bytes = [0; 4]; 
    let encoded_len = c.encode_utf8(&mut bytes).len();
    let slice: &[u8] = &bytes[..encoded_len];
    stdout().write( slice );
}

pub fn abi_terminate(code: i32) {
    stdout().write_all(b"[ABI:Terminate]!");
    exit(code);
}
