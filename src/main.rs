// use ferris_says::say;
// use std::io::{stdout, BufWriter};
//
// fn main(){
//     let stdout = stdout();
//     let message = String::from("Hello fellow Rustaceans!\
//     我");
//     let width = message.chars().count();
//
//     let mut writer = BufWriter::new(stdout.lock());
//     say(message.as_bytes(), width, &mut writer).unwrap();
//
// }
//

// ------------------------分割线------------------------
use std::io;

fn main() {
    println!("guess a number game");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("输入错误");

    println!("the number u guess is {}", guess);
}