// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
// }
//
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }
//
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         // 绑定值的模式匹配
//         Coin::Quarter(state) => {
//             println!("SAtate quarter from {:?}!", state);
//             25
//         }
//     }
// }
//
// fn main() {
//     let c = Coin::Quarter(UsState::Alabama);
//     println!("{}", value_in_cents(c))
// }
use std::collections::HashMap;

fn main() {
    // let mut v = vec![1, 2, 3, 4, 5];
    // v.push(12);
    //
    // let third: &mut i32 = &mut v[2];
    //
    // println!("{}", third);
    //
    // match v.get(2) {
    //     Some(third) => println!("The third element is {}", third),
    //     None => println!("啥也不是"),
    // }
    //
    // println!("{:?}", v)


    // let mut v = vec![1, 2, 3, 4, 5];
    // for i in &mut v {
    //     *i += 50;
    // }
    //
    // println!("{:?}", v);


    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);

    let scores_blue_entry = scores.entry(String::from("Bluse"));
    println!("啊啊啊啊 {:?}", scores_blue_entry);


    // let steam_name = String::from("Blue");
    // let score = scores.get(&steam_name); // &

    // match score {
    //     Some(10) => println!("数值是 10啊啊啊啊"),
    //
    //
    //     Some(_) => println!("数值是 {}", _),
    //     None => println!("啥也不是"),
    // }


}

