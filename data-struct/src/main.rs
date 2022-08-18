#[allow(dead_code)]
#[allow(unused_variables)]
#[warn(unused_doc_comments)]

fn main() {
    /*
    数组。 
    数组的每个元素都具有相同的数据类型。 数据类型永远不会更改。
     数组大小是固定的。 长度永远不会更改。
     */
    // Declare array, initialize all values, compiler infers length = 7
    let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
    
    // Declare array, initialize all values to 0, length = 5
    let bytes = [0; 5];

    /*
    矢量
    与数组一样，矢量存储数据类型相同的多个值。 
    与数组不同之处在于，向量的大小或长度是动态的。
    */
    // 声明和初始化向量的常用方法是使用 vec! 宏，该宏还接受与数组构造函数相同的语法。
    let three_nums = vec![1, 2, 3];
    println!("Initial vector: {:?}", three_nums); 
    let three_nums = vec![0, 5];
    println!("Initial vector: {:?}", three_nums); 
    // Vec 构造器
    let mut three_nums = Vec::new();
    // 动态添加数据，需要将变量声明为可变
    three_nums.push(123);

    /*
     索引访问。
     与数组一样，不允许索引越界。
     
     对于数组，这种类型的表达式会导致编译器返回错误。 
     对于向量，编译通过，但程序在表达式位置进入不可恢复的死机状态并停止程序执行。
     */
    let one = three_nums[0];

    /*
    哈希
    HashMap<K, V> 类型通过映射每个键 K 及其值 V 来存储数据。
     */
    use std::collections::HashMap;
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(String::from("123"), String::from("123"));
}
