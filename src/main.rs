use std::collections::HashMap;

fn main() {
    let text = "Hello World wonderful World";

    let mut map = HashMap::new();

    for world in text.split_whitespace() {
        // 基于现有的 V 来更新 V
        let count = map.entry(world).or_insert(0); // or_insert 返回的是可变引用 &mut i32 类型
        *count += 1;
    }

    println!("{:#?}", map);
}
