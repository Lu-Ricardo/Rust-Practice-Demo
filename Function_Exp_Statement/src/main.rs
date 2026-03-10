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
    //无限循环
    let ten = loop {
        sum += 1;
        if sum == 10 {
            // 跟在break后可作为返回值
            break sum;
        }
    };
    println!("{ten}");

    loop_fun();
}

fn num_five() -> i32 {
    //同理，也可作为函数的返回值，此时无需return
    5
}

//带参数的函数
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn loop_fun() {
    let mut id = 0;
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // loop 无限循环
    loop {
        println!("[{}] In loop", arr[id]);
        id += 1;
        if id == 10 {
            break;
        }
    }

    // 有条件循环
    id = 0;
    while id < 10 {
        println!("[{}] In while", arr[id]);
        id += 1;
    }

    // 迭代循环
    for i in 0..10 {
        println!("[{}] In for", arr[i]);
    }
}
