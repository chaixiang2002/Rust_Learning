use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the nuber!");

    //1..=100
    // rand::thread_rng 函数提供实际使用的随机数生成器
    // ：它位于当前执行线程的本地环境中，并从操作系统获取 seed
    let secret_number = rand::thread_rng().gen_range(1..101);

    //println!("The secrert number is: {}",secret_number);

    loop {
        println!("Please input your guess. ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("***『You win』***");
                break;
            }
        }
    }
}
