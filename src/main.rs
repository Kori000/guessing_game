use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数游戏!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("猜一个数字!");
    println!("秘密数字是 {}", secret_number);

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("读取行失败");

    let guess: u32 = guess.trim().parse().expect("请输入数字!");

    println!("你输入的值是 {}", guess);
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("太小了!"),
        Ordering::Greater => println!("太大了!"),
        Ordering::Equal => println!("猜对了!"),
    };
}
