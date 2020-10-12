fn another_function() {
    println!("Inside another function");
}

fn another_function_withparam(a: i32) {
    println!("Inside another function with parameter {}", a);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    println!("Hello, world!");

    another_function();
    another_function_withparam(99);

    println!("Outside function with return {}", plus_one(100));
}
