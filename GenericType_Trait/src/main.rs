fn main() {
    let vec = vec![2, 4, 1, 5, 6, 52, 211, 51];
    let ch = ['A', 'B', 'C'];

    println!(
        "Largest in vec is {}\nLargest in ch is {}",
        largest(&vec),
        largest(&ch)
    );
}

//泛型函数的定义，使用尖括号括起来即可，泛型名称可以任意设定
fn largest<U: std::cmp::PartialOrd>(arg: &[U]) -> &U {
    let mut largest = &arg[0];
    for i in arg.iter() {
        if i > largest {
            largest = i;
        }
    }
    largest
}
//甚至可以使用多个泛型
struct Point<T, U> {
    x: T,
    y: U,
}
