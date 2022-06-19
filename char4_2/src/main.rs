fn fn1() {
    println!("【fn1】：引用与借用");

    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    println!("\n");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn fn2() {
    println!("【fn2】：借用");

    let s = String::from("hello");

    change(&s);

    println!("\n");
}
fn change(some_string: &String) {
    // some_string.push_str(",world");this is error
}

fn fn3() {
    println!("【fn3】：只能有一个对某一特定数据的可变引用");

    //  在一个作用域同一时间，只能有一个【可变】引用
    //  不可变引用可以多个。
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    //first borrow later used here
    //println!("{},{}", r1, r2);

    println!("\n");
}

fn fn4() {
    println!("【fn4】：作用域");

    // 变量的作用域： 从声明的地方到最后一次调用
    // 可变引用的作用域 不能和 不可变引用的作用域重叠
    // 编译器在作用域结束之前判断不再使用的引用的能力
    //     被称为 非词法作用域生命周期（Non-Lexical Lifetimes，简称 NLL）
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{},{}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);

    println!("\n");
}

fn fn5() {
    println!("【fn5】：悬垂指针");

    let reference_to_nothing = no_dangle(); //dangle();

    println!("{}\n", reference_to_nothing);
}

/*
fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
*/

fn no_dangle() -> String {
    let s = String::from("hello");

    // &s
    s // 应当把所有权都移动出去而不是引用（使用权 ）
}

fn fn0() {
    println!("【fn】：");

    //1. 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
    //2. 引用必须总是有效的。(被引用的对象不能被销毁)

    println!("\n");
}

fn main() {
    fn1();
    fn2();
    fn3();
    fn4();
    fn5();
}
