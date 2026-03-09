struct Person {
    name: String,
    age: i32,
    tel: i64,
}
fn main() {
    let hum = Person {
        name: String::from("Ricardo"),
        age: 18,
        tel: 12345678910,
    };
    //居中对齐输出，宽度为20个字符
    println!("{:^20}\n{:^20}\n{:^20}", hum.name, hum.age, hum.tel);
    //使用 "-" 填充空白处
    println!("{:-^20}\n{:-^20}\n{:-^20}", hum.name, hum.age, hum.tel);
    //左对齐输出
    println!("{:<20}\n{:<20}\n{:<20}", hum.name, hum.age, hum.tel);
    // 使用不同字符填充空白处
    println!("{:*<20}\n{:-<20}\n{:*<20}", hum.name, hum.age, hum.tel);
    //右对齐输出
    println!("{:>20}\n{:>20}\n{:>20}", hum.name, hum.age, hum.tel);
    println!(
        "Hello World! I am {}. Tomorrow is my {} years old brithday party, Welcome for you! My telephone is {}.",
        hum.name, hum.age, hum.tel
    );
}
