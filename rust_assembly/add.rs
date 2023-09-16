use std::arch::asm;

fn add_asm(a: i32, b: i32) -> i32 {
    let result: i32;
    unsafe {
        asm!(
            "add {0}, {1}", // Sum operators
            inout(reg) a => result, // Entry and exit operators
            in(reg) b // Entry operator
        );
    }
    result
}

fn main() {
    let result = add_asm(5, 7);
    println!("Result: {}", result);
}
