// Rust 的集合类型不能直接进行循环，需要变成迭代器

fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "World, hello";
    let regions = [southern_germany, chinese, english];
    for region in regions {
            println!("{}", region);
    }
}

fn main() {
    greet_world();
}
