extern "C" {
    fn sum(a: i32, b: i32) -> i32;
}

fn main() {
    let a = 10;
    let b = 20;

    unsafe {
        let result = sum(a, b);
        println!("Sum of {} and {} is: {}", a, b, result);
    }
}
