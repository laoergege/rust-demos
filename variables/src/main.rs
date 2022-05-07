#[allow(dead_code)]
#[allow(unused_variables)]

/**
 * 不可变变量声明：let
 * 可变变量声明：mut
 * 常量声明：const
 * 解构
 */
fn main() {
    // let 声明默认不可变
    // let x = 5;
    // mut 声明可变
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    let x = 5;
    println!("The value of x is: {}", x);
    // 重新声明，隐藏旧变量
    let x = 6;
    println!("The value of x is: {}", x);

    // mut 与隐藏的另一个区别是，当再次使用 let 时，实际上创建了一个新变量，我们可以改变值的类型，并且复用这个名字
    let s = "123";
    let s = 123;


    // const 不可变且不可使用 mut，必须注明值的类型
    const Y: i32 = 7;


    // 变量解构
}
