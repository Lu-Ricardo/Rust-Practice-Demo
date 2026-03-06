fn main() {
    println!("猜大小！");
    let ans = rand::random_range(1..=100);
    loop {
        println!("请输入一个数字: ");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("未检测到输入...");
        let input: i32 = match input.trim().parse() {
            Ok(input) => input,
            Err(_) => continue,
        };
        if ans > input {
            println!("输入过小！");
        } else if ans < input {
            println!("输入过大！");
        } else {
            println!("恭喜你猜对了！");
            break;
        }
    }
}
