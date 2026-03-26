use std::ops::Add;

fn practice_2() {
    let integer_sum = add_two_numbers(10, 5);
    let float_sum = add_two_numbers(10.3, 5.2);

    println!("{}", integer_sum);
    println!("{}", float_sum);


}

fn add_two_numbers<T: Add<Output = T>>(a: T, b: T) -> T {
     a + b
}
