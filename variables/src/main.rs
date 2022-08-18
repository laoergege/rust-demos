#[allow(dead_code)]
#[allow(unused_variables)]
#[warn(unused_doc_comments)]

fn main() {
    /*
    变量声明：let，默认不可变
    可变变量声明：let mut
    常量声明：const，不可与 mut 配合，且必须注明值的类型
    */
    // let 声明默认不可变
    let x = 5;
    // x = 4; // 报错

    // mut 声明可变
    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is: {}", y);

    // const 不可变且不可使用 mut，必须注明值的类型
    const Y: i32 = 7;

    /*
    隐藏变量：实际上创建了一个新变量，我们可以改变值的类型，并且复用这个名字
    */
    let s = "123";
    let s = 123;

    /*
    类型声明。
    rust 编译器会自动隐式类型推导，但特殊情况也需要表明数据类型，如 const 变量声明
    */
    // 类型推导 number，默认 i32
    let a = 10;
    // 显示类型声明
    let b: i32 = 20;
    let c = 30_i32;
    let d = 30i32;
    println!("a + b = {}", a + b);

    /*
    字符串类型：
    str 类型也称为“字符串切片”
    可以将 &str 视为指向不可变字符串数据的指针。 字符串字面量的类型都是 &str;
    动态场景：Rust 具有另一个名为 String 的字符串类型。 此类型在堆上分配
    */
    // Specify the data type "char"
    let character_1: char = 'S';
    let character_2: char = 'f';

    let smiley_face = '😃';

    let string_1 = "miley ";

    let string_2: &str = "ace";

    println!(
        "{} is a {}{}{}{}.",
        smiley_face, character_1, string_1, character_2, string_2
    );
}
