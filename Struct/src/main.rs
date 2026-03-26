#[derive(Debug)] //如果需要在终端输出结构体，需要在结构体定义之前添加这个宏，加上外部参数
//一般结构体
struct Student {
    name: String, // 字符串类型不能为&str，因为结构体要保证拥有所有数据的所有权
    age: u8,
    id: u32,
    gpa: f32,
    major: String,
    class: u16,
}

impl Student {
    /*
     *如果需要使用实例中的字段，那么在声明关联函数时第一个参数必须为&self，表示调用该方法的实例。
     *&self等价于self: &Self
     *调用实例中的字段必须在前面加入self.，如self.name
     */
    fn introduct(&self) {
        println!(
            "Hello! My name is {}, I am {} years old, I learn {}, my id is {}, and I am in class {}, my gpa is {}",
            self.name, self.age, self.major, self.id, self.class, self.gpa
        );
    }
}

#[derive(Debug)]
//元组结构体，只指定字段类型
struct Position(i32, i32, i32);

//类单元结构体，不包含任何字段。因此不占用内存，只为实现派生方法
struct Printer;

//Printer的方法语法实现，为其实现print方法
impl Printer {
    /*
     *关联函数，不使用实例中的字段值
     */

    fn println(s: &str) {
        println!("{s}");
    }
}

fn main() {
    //结构体的实例化
    let std1 = Student {
        //声明顺序可以与定义顺序不同
        name: String::from("Ricardo"),
        id: 123456,
        age: 18,
        gpa: 4.0,
        major: String::from("Computer Science"),
        class: 231,
    };

    let std2 = Student {
        name: String::from("John"),
        id: 789012,
        //从std1中复制其他属性的值，此时std1的所有权被转移
        ..std1
    };

    let mut std3 = Student {
        name: String::from("Alice"),
        id: 345678,
        age: 20,
        gpa: 3.1,
        major: String::from("Mathematics"),
        class: 231,
    };

    let posi_std2 = Position(0, 0, 0);

    //从元组结构体中取出各个字段的值
    let Position(x, y, z) = posi_std2;

    //如果要修改结构体的某个值，必须整个结构体都是可变的。
    std3.name = String::from("Phbe");

    println!("Student 2: {std2:?}\nPosition = {posi_std2:?}");

    //分别输出
    std2.introduct();
    std3.introduct();

    //类单元结构体的声明
    let p1 = Printer;
    //调用无Self的关联函数需要使用::语法。
    Printer::println("This is also from Printer");
}
