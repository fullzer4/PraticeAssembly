use std::arch::asm;

//fix this code

fn main() {
    let result: i32;

    #[cfg(target_arch = "x86_64")]
    {
        unsafe {
            asm!(
                "/* {0} {1} */",

                "add $0, $1",
                inout(reg) 5 => result,
                in(reg) 7
            );
        }
    }

    #[cfg(target_arch = "arm")]
    {
        unsafe {
            asm!(
                "/* {0} {1} */",

                "add {0}, {1}",
                inout(reg) 5 => result,
                in(reg) 7
            );
        }
    }

    println!("Res: {}", result);
}
