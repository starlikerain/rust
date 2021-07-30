// use std::collections::HashMap;
//
// fn main() {
//     let text = "Hello World wonderful World";
//
//     let mut map = HashMap::new();
//
//     for world in text.split_whitespace() {
//         // 基于现有的 V 来更新 V
//         let count = map.entry(world).or_insert(0); // or_insert 返回的是可变引用 &mut i32 类型
//         *count += 1;
//     }
//
//     println!("{:#?}", map);
// }

// panic 产生错误的信息回溯
// fn main(){
//     let v = vec![1,2,3,4];
//
//     v[99];
// }



use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.text");

    // 用 match 简单实现如果出错的时候创建文件而不是直接简单的 panic
    // let _f = match f {
    //     Ok(file) => file,
    //     // Err(error) => {
    //     //     panic!("Error opening file {:?}", error)
    //     // }
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(error) => panic!("Error creating file {:?}", error)
    //         },
    //         oe => panic!("Error opening the file {:?}", oe)
    //     }
    // };

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Error creating file: {:?}", error)
            })
        } else {
            panic!("Error opening file {:?}", error)
        }
    });






}