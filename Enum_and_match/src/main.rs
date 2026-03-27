//普通枚举类型
enum Gender {
    Mael,
    Female,
}

//带数据的枚举类型
enum IpAddrKind {
    V4(i32, i32, i32, i32),
    V6(String),
}

//枚举同样可以有方法语法
impl IpAddrKind {
    fn text(&self) -> String {
        //带有数据的Match处理
        match self {
            IpAddrKind::V4(a, b, c, d) => format!("{}.{}.{}.{}", a, b, c, d),
            IpAddrKind::V6(ip) => String::clone(ip),
        }
    }
}

// Option枚举类型,用于表示一个值可能存在也可能不存在。强制开发者在编写代码时显式处理边界情况。这个枚举类型在标准库中定义如下：

/*
enum Option<T> {
    None,
    Some(T),
}
*/

// 使用代码示例
fn safe_divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None // 除数为零，返回 None 表示失败
    } else {
        Some(numerator / denominator) // 返回 Some 包裹的计算结果
    }
}

struct Student {
    name: String,
    age: u8,
    gender: Gender,
    id: i64,
}

fn main() {
    let student1 = Student {
        name: String::from("Alice"),
        age: 18,
        gender: Gender::Female,
        id: 123456,
    };

    /*对枚举的Macth匹配必须包含所有可能的情况，否则会报错
     *例如Gender中有Mael与Female两个字段，在Match中必须包含对这两个情况的处理
     */
    match student1.gender {
        Gender::Female => println!("Hello, Ms.{}", student1.name),
        Gender::Mael => println!("Hello, Mr.{}", student1.name),
    }

    let ip1 = IpAddrKind::V4(192, 168, 1, 1);
    let ip2 = IpAddrKind::V6(String::from("::1"));

    println!("ip1 is {}\nip2 is [{}]", ip1.text(), ip2.text());

    //Option枚举声明
    let op = Some(5);
    let n: Option<i8> = None;

    let result1 = safe_divide(2.0, 0.5);
    let result2 = safe_divide(10.0, 0.0);

    match result2 {
        None => println!("Error! Division by zero"),
        Some(i) => println!("The rusult is {}", i),
    }

    // if let 语句，只匹配Some时， None时什么也不做。如下代码只有在result不为None时输出结果，r为result的值。
    if let Some(r) = result1 {
        println!("The result is {r}");
    }

    /*let ... else 语法，如果匹配则跳过else后的语句块，不匹配则执行else后的语句块并结束当前语句块。
     * 例如下面的代码，如果性别不是Mael，则输出信息并结束程序
     */
    let Gender::Mael = student1.gender else {
        println!("You're not a male!");
        return;
    };
    println!("Hello, {}!", student1.name);
}
