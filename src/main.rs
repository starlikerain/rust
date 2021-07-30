#[derive(Debug)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must between 1 and 100, get {}", value)
        }

        Guess { value }
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    loop {
        let guess = "32";
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let guess = Guess::new(guess);
        println!("{:#?}",guess);
    }
}


// use std::io::Read;
// use std::io;
// use std::fs::File;


// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("Hello.txt");
//
//     let mut f = match f {
//         Ok(file) => file,
//         Err(error) => return Err(error),
//     };
//
//     let mut s = String::new();
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut s = String::new();
//     File::open("hello.txt")?.read_to_string(&mut s)?;
//     Ok(s)
// }
//
// use std::error::Error;
// use std::net::IpAddr;
//
// fn main() -> Result<(), Box<dyn Error>> {
//     // let result = read_username_from_file();
//     // println!("{:?}", result);
//     let ip: IpAddr = "127.0.0.1".parse().expect("DIY error");
//     println!("{:?}", ip);
//     Ok(())
// }