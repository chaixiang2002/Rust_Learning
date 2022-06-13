fn main() {
    // -- 数据类型 --

    let guess: u32 = "42".parse().expect("Not a number!");

    let x = 2.0;
    let y: f32 = 3.0;

    let t = true;
    let f: bool = false;

    let _c = 'z';
    let z = 'Z';

    let dog = '😻';

    /*

     我们用单引号声明 char 字面量，而与之相反的是
     ，使用双引号声明字符串字面量

    */

    // 元组类型
    // 一旦声明，其长度不会增大或缩小
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup2 = (500, 6.4, 1);
    let (_, y, _) = tup2;
    println!("The value of y is {}", y);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    let snow = ();
    // 没有任何值的元组 () 是一种特殊的类型，只有一个值，也写成 () 。
    // 该类型被称为 单元类型（unit type），而该值被称为 单元值（unit value）。
    // 如果表达式不返回任何其他值，则会隐式返回单元值。

    // 数组类型
    // 与元组不同，数组中的每个元素的类型必须相同。
    // Rust中的数组长度是固定的。
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5]; // let a = [3, 3, 3, 3, 3];
    let first = a[0];
    let second = a[1];
    // 当你想要在栈（stack）而不是在堆（heap）上为数据分配空间
    //（第四章将讨论栈与堆的更多内容），或者是想要确保总是有固定数量的元素时
    // ，数组非常有用。

    println!("Hello, world!");
}
