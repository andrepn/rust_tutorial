use std::io;

fn fibnumcalc(fibnum: u32) -> u32 {
    let mut x = 1;
    let mut y = 0;
    let mut count = 0;

    loop {
        let z = y;
        y = x + y;
        x = z;

        if count == fibnum {
            break x;
        }
        count += 1;
    }
}

fn main() {
    println!("Tell me which fib num to fetch boss!");

    let mut fibnum = String::new();

    io::stdin()
        .read_line(&mut fibnum)
        .expect("Failed to read line boss");

    let fibnum: u32 = match fibnum.trim().parse() {
        Ok(num) => num,
        Err(_) => 1000,
    };

    let finalnum = fibnumcalc(fibnum);
    println!("The fibonacci number is {}", finalnum);
}
