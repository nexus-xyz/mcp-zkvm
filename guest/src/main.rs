
#![cfg_attr(target_arch = "riscv32", no_std, no_main)]
#[nexus_rt::main]
fn main() {
let base: u128 = 3252;
let result: u128 = base * base * base * base;
assert_eq!(result, 111_841_284_854_016);
}