fn main() {
    let a = {
        //语句与表达式
        let b = 10; //语句，带有分号无返回值
        b + 5 //表达式，有返回值，不带分号，放在语句块结尾可以作为返回值
    };
    println!("a = {a}");
    println!("num_five = {}", num_five());
    println!("a + num_five = {}", add(a, num_five()));

    let mut sum = 0;
    let ten = loop {
        sum += 1;
        if sum == 10 {
            break sum;
        }
    };
    println!("{ten}");
}

fn num_five() -> i32 {
    //同理，也可作为函数的返回值，此时无需return
    5
}

//带参数的函数
fn add(a: i32, b: i32) -> i32 {
    a + b
}
