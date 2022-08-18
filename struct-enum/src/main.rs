#[allow(dead_code)]
#[allow(unused_variables)]
#[warn(unused_doc_comments)]

fn main() {
    /**
    结构体是多个其他类型的组合体。结构中的元素称为字段。
    结构中的字段可以具有不同的数据类型，并且可以命名每个字段，以便清楚展示相应值的含义。

    Rust 支持三种结构类型：经典结构、元组结构和单元结构。
    */

    // 经典结构，🤔类似 js 中的对象
    struct Student {
        name: String,
        level: u8,
        remote: bool,
    }
    let stu = Student {
        name: String::from("lys"),
        level: 1,
        remote: false
    };

    // 元组结构：字段没有名称，通过索引。🤔类似 js 中的数组
    struct Grades(char, char, char, char, f32);
    let mark_1 = Grades('A', 'A', 'B', 'A', 3.75);

    // “单元结构”最常用作标记
    struct Unit;

    /**
     * 枚举，即联合标签
     * 与结构一样，枚举变体可以具有命名字段，但也可以具有没有名称的字段或根本没有字段
     */
    #[derive(Debug)]
    enum WebEvent {
        WELoad,
        WEKeys(String, char),
        WEClick { x: i64, y: i64 }
    }

    // 使用结构定义枚举
    struct KeyPress(String, char);
    struct MouseClick { x: i64, y: i64 }

    // Redefine the enum variants to use the data from the new structs
    // Update the page Load variant to have the boolean type
    enum WebEvent2 { WELoad(bool), WEClick(MouseClick), WEKeys(KeyPress) }
    let we_load = WebEvent2::WELoad(true); 
    let we_load = WebEvent2::WEClick(MouseClick {
        x: 1,
        y: 2
    });
    let we_load = WebEvent2::WEKeys(KeyPress (String::from("k"), '1'));
    
}
