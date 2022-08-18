#[allow(dead_code)]
#[allow(unused_variables)]
#[warn(unused_doc_comments)]

/**
 * 在 rust 中，一切皆表达式
 */
fn main() {
    greet_world();

    if_else();
}

fn if_else() {
    /*
    在 Rust 中的 if 块也可充当表达式。
    但条件分支中的所有执行块都必须为要编译的代码返回相同的类型。
    */
    let formal = true;
    let greeting = if formal {
        // if used here as an expression
        "Good day to you." // return a String
    } else {
        "Hey!" // return a String
    };
    println!("{}", greeting) // prints "Good day to you."
}

/**
 * 循环：`loop / while(条件) / for...in(迭代器)` + `break / continue`
 * for...in(迭代器) 循环实际上只是一个语法糖，编译器会将其展开使用 loop 循环对迭代器进行循环访问，直至返回 None
 *
 * 匹配： `match expr {}`、`if let / while let`
 */
fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "World, hello";
    let regions = [southern_germany, chinese, english];

    // loop
    println!("loop:");
    let mut i = 0;
    loop {
        println!("{}", regions[i]);
        i += 1;
        if i == regions.len() {
            break;
        }
    }

    // while 条件循环
    println!("while");
    let mut i = 0;
    while i <= regions.len() {
        println!("{}", regions[i]);
        i += 1;
    }

    // for 迭代器
    println!("for...in");
    for region in regions {
        println!("{}", region);
    }
}
