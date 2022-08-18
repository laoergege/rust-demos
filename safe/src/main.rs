use std::{collections::HashMap, fs::read_to_string};

/**
 * Rust 通过强制执行一些基本规则（尽管很严格 ）来保证内存安全，这些规则涉及到如何传输数据，如何“借用”数据以及谁“拥有”数据。
 */
fn main() {
    let source = read_to_string("./Cargo.toml").unwrap();
    let mut files = HashMap::new();
   
    // 所有权：在代码块中，当把一个数据传给一个函数之后，此调用方代码块(calling code)将无权再访问该数据。
    files.insert("one", source.clone()); // 复制 source 给 insert 函数
    files.insert("two", source);

    // 当借用数据时（即当你引用一个数据），可以做无数次的不可变借用，只能做一次可变借用。通常，在值前加上 & 符号来获取引用
    let mut_files_ref = &mut files;
    // mut_files_ref2 = &mut files; // 不可同时多次可变借用
    needs_mutable_ref(mut_files_ref);

    // 稍后可变借用
    // let mut_files_ref2 = &mut files;

    // 借用顺序：可变借用处理完后面才能再进行借用
    let files_ref = &files;
    let files_ref2 = &files;
    print_borrowed_map(files_ref);
    print_borrowed_map(files_ref2);

}

fn print_borrowed_map(map: &HashMap<&str, String>) {
    println!("{:?}", map)
}

fn needs_mutable_ref(map: &mut HashMap<&str, String>) {
    map.insert("three", "123".to_string());
}