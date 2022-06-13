fn main() {
    // --- 3.1变量和可变性


    let mut x=5;
    println!("The value of x is {}",x);
    x=6;
    println!("The value of x is {}",x);

    const THREE_HOURS_IN_SECONDS: u32=60*60*3;
    println!("The value of x is {}",THREE_HOURS_IN_SECONDS);

    /* 隐藏 和 被标记为mut 有区别 */
    // 当再次使用 let 时，实际上创建了一个新变量
    // ，我们可以改变值的类型，并且复用这个名字。
    let spaces= "   ";
    let spaces= spaces.len();

    let mut space ="   ";
    space=space.len();
}
