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
    println!(
        "Hello World! I am {}, my age is {}, my telephone is {}",
        hum.name, hum.age, hum.tel
    );
}
