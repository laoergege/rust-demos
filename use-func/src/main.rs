fn main() {
    goodbye("hello rust");
}

/**
 * 可定义在 main 函数后
 * Rust 不在意文件中函数的定义位置，只要在文件中的某处定义了函数即可。
 */
fn goodbye(msg: &str) -> i32 {
    println!("\n{}", msg);
    1
}