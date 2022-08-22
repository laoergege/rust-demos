#[allow(dead_code)]
#[warn(unused_doc_comments)]
#[allow(unused_variables)]

/**
 * 作用域界定规则
 * 
 * 生命周期
 */
fn main() {

    // scope();

    // move_var_ownership();

    // ownership_of_fn();

    // borrow();

    test_copy_and_retur();
}

/**
 * 作用域界定变量的生命周期
 * 
 * 超过生命周期的变量会被自动删除，删除变量意味着释放与其关联的所有资源
 */
// fn scope() {
//     {
//         let mascot = String::from("ferris");
//     }
//     println!("{}", mascot);
// }

/**
 * 所有权：rust 的变量对某块内存数据是绑定在一起的，即引用不可变
 * 
 * 一旦变量发生赋值（引用值）给其他变量，就会发生所有权移动
 */
// fn move_var_ownership() {
//     let mascot = String::from("ferris");

//     //移动：所有权一旦转移，旧变量将不再有效
//     let ferris = mascot;
//     println!("{}", mascot);
// }

/**
 * rust 函数传参：值传递
 * 对于原始类型而言则是直接复制
 * 对于引用类型则会“所有权移动”
 */
// fn ownership_of_fn() {
//     fn process(input: String) {}

//     let s = String::from("Hello, world!");

//     process(s); // Ownership of the string in `s` moved into `process`
//     process(s); // Error! ownership already moved.

//     // 显示复制而防止所有权移动
//     // process(s.clone()); // Passing another value, cloned from `s`.
// }

/**
 * 借用：即获取变量自身的引用，而不是把变量对内存的引用传递给其他变量
 * 
 * 通过借用方式可防止复制带来消耗和所有权移动
 * 
 * &T：不可变借用
 * &mut T: 可变借用
 * 
 * 代码只允许
 * 一个或多个不可变借用 (&T)
 * 只有一个可变借用 (&mut T)
 */
// fn borrow() {
//     fn print_greeting(message: &mut String) {
//         message.insert(0, '1');
//         println!("Greeting: {}", message);
//     }

//     let mut greeting = String::from("hello");
//     let _greeting_reference = &greeting; // We borrow `greeting` but the string data is still owned by `greeting`

//     print_greeting(&mut greeting);

//     println!("Greeting: {}", greeting); // We can still use `greeting` 
// }

/**
 * 使用生存期验证引用：当该项被删除且其资源被释放时，我们如何确保没有引用指向这个现已释放且无效的内存？
 * 
 * 生命周期实现
 * 在函数中注释生存期
 */
fn life_of_borrow() {

}

// fn copy_and_return<'a>(vector: &'a mut Vec<String>, value: &'a str) -> &'a str {
//     vector.push(String::from(value.clone()));
//     return value
// }
fn copy_and_return(vector: & mut Vec<String>, value: & str) {
    vector.push(String::from(value.clone()));
}

fn test_copy_and_retur() {
    let name1 = "Joe";
    let name2 = "Chris";
    let name3 = "Anne";

    let mut names = Vec::new();
    copy_and_return(&mut names, &name1);

    println!("{:?}", names);

    // assert_eq!("Joe", copy_and_return(&mut names, &name1));
    // assert_eq!("Chris", copy_and_return(&mut names, &name2));
    // assert_eq!("Anne", copy_and_return(&mut names, &name3));

    assert_eq!(
        names,
        vec!["Joe".to_string(), "Chris".to_string(), "Anne".to_string()]
    )
}

//一个可变引用或多个不可变引用。
// 正确。 Rust 禁止在同一位置同时使用别名操作和变异。