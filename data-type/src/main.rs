#[allow(unused_variables)]

/**
 * rust 编译器会自动隐式类型推到，但特殊情况也需要表明数据类型
 */
fn main() {
    // number，默认 i32
    let a = 10;
    // 现实类型
    let b: i32 = 20;
    let c = 30_i32;
    let d = 30i32;
    println!("a + b = {}", add(a, b))
}

fn add(i: i32, j: i32) -> i32 {
    i + j
}
