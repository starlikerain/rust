fn main() {
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    //
    // println!("{} {} {}", tup.0, tup.1, tup.2);
    // println!("{}", tup.3);
    //
    // let a: [i32; 5] = [1, 2, 3, 4, 5];


    // let x = 5;

    // let y = {
    //     let x = 1;
    //     x + 4
    // };
    //
    // println!("The y value is {}", y);

    // let number = 7;
    //
    // // 不会做隐式转换，必须用 bool
    // if number < 5 {
    //     println!("condition true")
    // } else {
    //     println!("condition false")
    // }

    // for number in (1..=4).rev() {
    //     println!("{}!", number)
    // }
    //
    // println!("LIFTOFF!")

    /**
    String 类型 和 字符串字面值
    */
    // let mut s = String::from("我是string2"); // String 类型
    // let mut s2 = "我是s2"; // 字符串字面值
    //
    //
    // s.push_str("尾巴"); // success
    // s2.push_str("啊"); // method not found in `&str`
    //
    // println!("str2 {}", s);
    // println!("sr {}", s2);


    /**
    只复制了 stack 的指针，没有复制到 heap 的数据，
    如果s1退出了 scope，那么会引发 drop，
    同理 s2 退出 scope 也有同样的问题，会引发二次释放(double free)的 bug
     */
    // let s1 = String::from("hello");
    // let s2 = s1;


}




