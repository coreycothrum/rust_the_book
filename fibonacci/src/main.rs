fn fibonacci_number(n: usize) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci_number(n - 1) + fibonacci_number(n - 2),
    }
}

fn main() {
    for n in 0..=10 {
        let f = fibonacci_number(n);
        println!("{n}: {f}");
    }
}
