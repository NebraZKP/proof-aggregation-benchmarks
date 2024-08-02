#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

use nexus_rt::{print, read_private_input};

#[nexus_rt::main]
fn main() {
    let input: u32 = read_private_input().unwrap();
    print!("Hello, world! Input: {}\n", input);
}
