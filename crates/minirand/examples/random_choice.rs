use minirand::{
    RandomU8, RandomI32,
    RandomF32, RandomF64,
};

fn main() {
    // --- Random choice from integers ---
    let u_numbers = [1u8, 2, 3, 4, 5];
    println!("Random u8 choices:");
    for _ in 0..10 {
        let choice = RandomU8::random_choice(&u_numbers);
        print!("{} ", choice);
    }
    println!("\n");

    let i_numbers = [-10i32, -5, 0, 5, 10];
    println!("Random i32 choices:");
    for _ in 0..10 {
        let choice = RandomI32::random_choice(&i_numbers);
        print!("{} ", choice);
    }
    println!("\n");

    // --- Random choice from floats ---
    let f_numbers = [0.1f32, 0.5, 0.9, 1.5, 2.0];
    println!("Random f32 choices:");
    for _ in 0..10 {
        let choice = RandomF32::random_choice(&f_numbers);
        print!("{:.2} ", choice);
    }
    println!("\n");

    let f64_numbers = [10.0f64, 20.0, 30.0, 40.0, 50.0];
    println!("Random f64 choices:");
    for _ in 0..10 {
        let choice = RandomF64::random_choice(&f64_numbers);
        print!("{:.1} ", choice);
    }
    println!("\n");

    // --- Random choice from strings ---
    let words = ["apple", "banana", "cherry", "date"];
    println!("Random word choices:");
    for _ in 0..10 {
        let choice = RandomU8::random_choice(&words);
        print!("{} ", choice);
    }
    println!();
}
