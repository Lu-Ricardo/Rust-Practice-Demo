/*
-------------------------------------------------
Trait: 共同方法，可以被多个类型实现。
优势：降低代码的耦合度，提高代码的复用性。
-------------------------------------------------
 */

struct Man;

struct SuperMan;

// Trait 声明
trait Fly {
    fn Fly(&self);
}

trait Run {
    //纯虚函数，不需要实现。需要实现者自定义的方法
    fn slow_run(&self);
    fn fast_run(&self);

    //实函数，有默认实现方法
    fn walk(&self) {
        println!("Man is walking");
    }
}

impl Fly for SuperMan {
    fn Fly(&self) {
        println!("SuperMan is flying");
    }
}

/*
impl Run for Man {
    //如果定义了不只一个纯虚函数，需要全部实现
    fn slow_run(&self) {
        println!("Man is running slowly");
    }

    fn fast_run(&self) {
        println!("Man is running fast");
    }

    //walk函数因为有默认实现，所以可以不必重写
}

impl Run for SuperMan {
    fn slow_run(&self) {
        println!("Man is running slowly");
    }

    fn fast_run(&self) {
        println!("Man is running fast");
    }
}

上面的代码可以使用宏进行简化，因为run和fly接口中的函数名和参数列表都完全一致
*/
macro_rules! impl_run {
    ($type:ty, $name:expr) => {
        impl Run for $type {
            fn slow_run(&self) {
                println!("{} is running slowly", $name);
            }
            fn fast_run(&self) {
                println!("{} is runingg fast", $name);
            }
        }
    };
}

impl_run!(SuperMan, "SuperMan");
impl_run!(Man, "Man");

fn main() {
    println!("Hello, world!");
    SuperMan.Fly();
    Man.slow_run();
    Man.fast_run();
    Man.walk();
    let hum = Man;
    notify(&hum);
}

fn notify(t: &impl Run) {
    println!("Get status...");
    t.fast_run();
}
