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

use std::cmp::Ordering;
// ------------------------分割线------------------------
use std::io;

// prelude  // trait
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("secret_number: {}", secret_number);


    loop {
        println!("Guess a number:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("输入错误");

        // 这个写法虽然捕获了但是也退出（崩溃）了
        // let guess:u32 = guess.trim().parse().expect("please enter a number");

        // This way is better
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!!!");
                break;
            }
        }
    }
}



