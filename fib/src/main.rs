use std::io;

fn main() {
    println!("Fib Input: ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Non valid readline");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };

    let ans = fib(guess);
    println!("{}", ans);
}

fn fib(n: u32) -> u64 {
    let mut pre: u64 = 0;
    let mut post: u64 = 1;

    for _index in 0..n {
        post = pre + post;
        pre = post - pre;
    }

    pre
}