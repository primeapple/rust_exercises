const NTH_FIBONACCI_NUMBER: u32 = 10;

fn main() {
    println!("Generating the {}. Fibonacci Number", NTH_FIBONACCI_NUMBER);
    // println!("Using a loop");
    // println!("The {}. Fibonacci Number is {}",
    //     NTH_FIBONACCI_NUMBER,
    //     fibonacci_loop(NTH_FIBONACCI_NUMBER)
    // );
    println!("Using recursion");
    println!("The {}. Fibonacci Number is {}",
        NTH_FIBONACCI_NUMBER,
        fibonacci_recursion(NTH_FIBONACCI_NUMBER)
    );
}

// TODO: use a vector instead of an array
// fn fibonacci_loop(n: u32) -> u32 {
//     let numbers = [0; n+1];
//     numbers[1] = 1;
//     for index in (2..=n) {
//         numbers[index] = numbers[index-1] + numbers[index-2];
//     }
//     numbers[n]
// }

fn fibonacci_recursion(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursion(n-1) + fibonacci_recursion(n-2)
    }
}
