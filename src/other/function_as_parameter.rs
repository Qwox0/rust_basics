fn fun_test(value: i32, f: &dyn Fn(i32) -> i32) -> i32 {
    println!("f({}) = {}", value, f(value));
    // println!("{:?}", f);
    value
}

fn times2(value: i32) -> i32 {
    2 * value
}

#[allow(unused)]
pub fn run() {
    fun_test(5, &times2);
}
