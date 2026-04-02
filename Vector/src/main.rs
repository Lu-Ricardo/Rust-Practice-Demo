use std::collections::HashMap;

fn main() {
    /*
     * 宏创建
     * 函数创建
     * 迭代器创建
     */

    let a = vec![1, 2, 3];
    let mut b: Vec<i32> = Vec::new(); //因为后期添加元素，所以需要声明为可变的向量
    b.push(4);
    b.push(5);
    b.push(6);
    let c: Vec<i32> = [7, 8, 9].to_vec();

    //vec中元素值的获取
    let first = &a[0];
    let second = a.get(1); //得到的是Option<&i32>类型，需要解构才能使用

    /*
     * 使用索引获取元素值时，如果索引超出范围会报错。例如 first = a[3];
     * 使用get方法可以避免这种错误，它超出边界时会返回None
     */

    if let Some(i) = second {
        println!("first is {first}, second is {i}");
    }

    for i in &c {
        println!("c element is {i}");
    }

    // 如果想要在vector中存放不同类型的数据，可以配合枚举进行
    enum Data {
        Int(i32),
        Str(String),
        Id(i64),
    }

    let d = vec![
        Data::Int(1),
        Data::Str(String::from("Test")),
        Data::Id(123456),
    ];
    // 使用match 匹配模式或者if let 语句单独提取某个值
    for it in d {
        match it {
            Data::Int(i) => println!("int is {i}"),
            Data::Str(str) => println!("String is {str}"),
            Data::Id(id) => println!("Id is {id}"),
        }
    }

    //字符串的引用
    let str = String::from("This");
    let sc1 = &str[0..=2];
    /*
    *以下引用是错误的
    let sc1 = &str[0];
    因为这样的索引指定的类型不确定，Rust 不允许这种索引方式。
    */
    println!("{sc1}");

    //字符串的遍历

    // 1. 作为字符遍历
    for s in str.chars() {
        println!("{}", s);
    }

    // 2. 作为字节遍历
    for s in str.bytes() {
        println!("{s}");
    }

    let mut vec = vec![4, 5, 2, 5, 0, 2, 5, 9, 1, 6, 32, 4, 1, 5, 7, 5, 4, 7];
    sort_and_statistics(&mut vec);
    str_to_piglatin("This is a Message fot test");
}

// 练习1：对于给定的整数数组，查找中位数和众数
fn sort_and_statistics(vec: &mut Vec<i32>) {
    vec.sort();
    let len = vec.len();
    if len % 2 == 0 {
        println!("中位数: {}", (vec[len / 2 - 1] + vec[len / 2]) / 2);
    } else {
        println!("中位数：{}", vec[(len - 1) / 2]);
    }
    let mut map: HashMap<i32, i32> = HashMap::new();
    for v in vec.iter() {
        let count = map.entry(*v).or_insert(0);
        *count += 1;
    }

    let mut max_count = 0;
    let mut max_value = 0;

    for (m, n) in map.iter() {
        //因为m,n实际上是引用类型，其值为迭代器中对应数值的地址。而max_count是i32类型，无法直接进行运算所以需要*解引用
        if *n > max_count {
            max_count = *n;
            max_value = *m;
        }
    }
    println!("众数: {}", max_value);
}

fn str_to_piglatin(s: &str) {
    for word in s.split_whitespace() {
        if word.starts_with('A')
            || word.starts_with('E')
            || word.starts_with('I')
            || word.starts_with('O')
            || word.starts_with('U')
            || word.starts_with('a')
            || word.starts_with('e')
            || word.starts_with('i')
            || word.starts_with('o')
            || word.starts_with('u')
        {
            println!("{}{}ay", &word[1..=word.len() - 1], &word[0..1]);
        } else {
            println!("{}hay", &word);
        }
    }
}
