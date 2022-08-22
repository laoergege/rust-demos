
#[allow(dead_code)]
#[allow(unused_variables)]
#[warn(unused_doc_comments)]

use std::io;
use std::{io::{Error, Read}, fs::File};
use std::panic;

/**

- 使用 unwrap 方法直接访问 Option::Some 类型的内部值，但是如果变体是 None，则此方法将会 panic。
- expect 方法的作用与 unwrap 相同，但它提供由第二个参数提供的自定义 panic 消息。
- 可选链 ?. 


Result<T, E>：表示可能处理成功或失败的结果
enum Result<T, E> {
    Ok(T):  // A value T was obtained.
    Err(E): // An error of type E was encountered instead.
}
- 使用 unwrap 方法直接访问 Result::Ok 类型的内部值，但是如果变体是 None，则此方法将会 panic。
- expect 方法的作用与 unwrap 相同，但它提供由第二个参数提供的自定义 panic 消息。
- ？：传播错误
 */
fn main() {
    // vec 索引越界访问会造成 panic
    let v = vec![0, 1, 2, 3];
    // println!("{}", v[6]); // this will cause a panic!

    // 手动抛出错误，并捕获
    if let Err(error) = panic::catch_unwind(error) {
        println!("panic captured: {:#?}", error)
    };



    /*
     以下使用模式匹配方式去解构 Option、Resutl
     */


    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];  

    for index in [0, 2, 99] {
        match fruits.get(index) {
            Some(&"coconut") => println!("Coconuts are awesome!!!"),
            Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
            // None => println!("There is no fruit! :("),
            //  _（下划线）通配符模式，以匹配任何其他项。^ 必须有，因为 `match` 需要覆盖全部情况。
            _ => println!("There is no fruit! :(") 
        }
    }

    /*
    if let 运算符可将模式与表达式进行比较。 如果表达式与模式匹配，则会执行 if 块。 
    if let 表达式的好处是，当你关注的是要匹配的单个模式时，你不需要 match 表达式的所有样板代码。
     */
    for index in [0, 2, 99] {
        if let Some(fruit_name) = fruits.get(index) {
            println!("It's a delicious {}!", fruit_name)
        } else {
            println!("There is no fruit! :(") // 解构失败。切换到失败情形。
        }
    }

    let stdin = io::stdin();
    let mut input = String::new();
    while let Ok(n) = stdin.read_line(&mut input) {
        println!("{} bytes read", n);
        println!("You input: {}", input);
        input.clear();
    }
}

fn read_file_contents(path: &str) -> Result<String, Error> {
    let mut content = String::new();

    File::open(path)?.read_to_string(&mut content)?;

    Ok(content)
}

fn error() {
    panic!("发生错误")
}