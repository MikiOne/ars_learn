use core::arch::asm;

fn main() {
    let mut x: u32 = 5;
    unsafe {
        asm!(
        "add {0}, {1}",
        inout(reg) x,
        in(reg) 3
        );
    }
    println!("Result: {}", x);
}