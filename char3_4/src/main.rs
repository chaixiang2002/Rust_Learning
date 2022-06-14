fn main() {
    println!("Hello, world!");

    // hello,world

    // if表达式
    let number = 3;

    if number < 3 {
        println!("condition was true");
    } else if number % 3 == 0 {
        println!("sda");
    } else {
        println!("condition was false");
    }
    let condition = true;
    let numbr = if condition { 5 } else { 6 };
    println!("numbr is {}\n\n", numbr);

    fn2();
    fn3();
    fn4();
    fn5();
    fn6();
}

fn fn2() {
    println!("【fn2】");

    let mut count = 0;
    'counting_up: loop {
        println!("count={}", count);
        let mut remaining = 10;

        loop {
            println!("remaining={}", remaining);
            if remaining == 9 {
                break;
                // break 将只退出内层循环
            }
            if count == 2 {
                break 'counting_up;
                // break 'counting_up; 语句将退出外层循环
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count ={}\n\n", count);
}

fn fn3() {
    println!("【fn3】:从循环返回值");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}\n\n", result);
}

fn fn4() {
    println!("【fn4】：while条件循环");

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }
}

fn fn5() {
    println!("【fn5】:for循环遍历集合的元素");

    let a = [10, 20, 30, 40, 50];

    for t in a {
        println!("the value is:{}\n\n", t);
    }
}

fn fn6() {
    println!("【fn6】:for来倒计时");

    // (1..4) 前开后闭
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!\n\n");
}
