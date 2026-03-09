fn main() {
    let n1: i32 = 7;
    let n2: f32 = 3.14159;

    let n3: i32 = 0xAF; //175
    let n4: i32 = 0b110; //6
    let n5: i32 = 0o26; //22

    let r1: f32 = (n1 / 3) as f32;
    let r2: i32 = (n2 / 3.0) as i32;

    println!("{n1} / 3 = {r1}\n {n2} / 3 = {r2}"); // 除法结果为浮点数时向下取整
    println!("n2 = {:.2}", n2); // 保留小数点后两位
    println!("n3 (Hex) = {n3}, n4 (Bin) = {n4}, n5 (Oct) = {n5}"); //输出结果均为十进制数
    println!(
        "n3 (Hex) = {:x}, n4 (Bin) = {:b}, n5 (Oct) = {:o}",
        n3, n4, n5
    ); //按照对应进制(16, 2, 8)输出

    let tuple: (&str, i8, i64) = ("Ricardo", 18, 12345678910);
    //提取元组中对应的值
    let (name, age, tel) = tuple;

    println!("Tuple: {:?}\n name: {name}, age :{age}, tel: {tel}", tuple);

    let a1 = [3; 10]; //初始化一个数组，一共十个元素，每个元素均为3
    let a2 = [1, 2, 3, 4, 5]; //初始化一个数组，有1-5五个元素

    println!("a1 = {:?}\na2 = {:?}", a1, a2);
    // 输出对应索引的值
    println!("a1(0-4) = {:?}, a2(0-2) = {:?}", &a1[0..=4], &a2[0..=2]);
}
