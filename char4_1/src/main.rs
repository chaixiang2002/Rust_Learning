fn fn1() {
    println!("【fn1】");
    /*  1. Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。
    2. 值在任一时刻有且只有一个所有者。
    3. 当所有者（变量）离开作用域，这个值将被丢弃。 */
    println!("所有权规则：");

    // 变量作用域
    {
        // s 在这里无效, 它尚未声明
        let s = "hello"; // 从此处起，s 是有效的

        // 使用 s
        println!("{}", s);
    } // 此作用域已结束，s 不再有效

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() 在字符串后追加字面值

    println!("{}\n\n", s); // 将打印 `hello, world!`
}

fn fn2() {
    println!("【fn2】:move");
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{},world!", s1);this is error
    println!("{},world!", s2);

    println!("\n");
}

fn fn3() {
    println!("【fn3】:clone");

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1={}, s2={}\n", s1, s2);

    // 只在栈上的数据：是可以直接被拷贝的
    let x = 5;
    let y = x;
    println!("x={},y={}", x, y);

    /*
    任何不需要分配内存或某种形式资源的类型都可以实现 Copy
    如下是一些 Copy 的类型：
    所有整数类型，比如 u32。
    布尔类型，bool，它的值是 true 和 false。
    所有浮点数类型，比如 f64。
    字符类型，char。

    元组，当且仅当其包含的类型也都实现 Copy 的时候。
    比如，(i32, i32) 实现了 Copy，
    但 (i32, String) 就没有。

    println!("\n");
    */
}

fn fn4() {
    println!("【fn4】:所有权与函数");

    let s = String::from("hello"); // s 进入作用域

    takes_ownership(s); // s 的值移动到函数里 ...
                        // ... 所以到这里不再有效

    let x = 5; // x 进入作用域

    makes_copy(x); // x 应该移动函数里，
                   // 但 i32 是 Copy 的，
                   // 所以在后面可继续使用 x

    println!("\n");
} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 没有特殊之处

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。
  // 占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。没有特殊之处

fn fn5() {
    println!("【fn5】:返回值与作用域");

    let s1 = gives_ownership(); // gives_ownership 将返回值
                                // 转移给 s1

    let s2 = String::from("hello"); // s2 进入作用域

    let s3 = takes_and_gives_back(s2); // s2 被移动到
                                       // takes_and_gives_back 中,
                                       // 它也将返回值移给 s3
} // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
  // 所以什么也不会发生。s1 离开作用域并被丢弃

fn gives_ownership() -> String {
    // gives_ownership 会将
    // 返回值移动给
    // 调用它的函数

    let some_string = String::from("yours"); // some_string 进入作用域.

    some_string // 返回 some_string
                // 并移出给调用的函数
                //
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域
    //

    a_string // 返回 a_string 并移出给调用的函数
}

fn fn6() {
    println!("【fn6】:");

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}
fn main() {
    fn1();
    fn2();
    fn3();
    fn4();
    fn5();
    println!("\n");
    fn6();
}
