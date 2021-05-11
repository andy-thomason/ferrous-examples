
fn fizzbuzz0(i: u32) -> String {
    if i % 3 == 0 && i % 5 == 0 {
        "FizzBuzz".to_owned()
    } else if i % 3 == 0 {
        "Fizz".to_owned(),
    } else if i % 5 == 0 {
        "FizzBuzz".to_owned()
    } else {
        format!("{}", i)
    }
}

fn fizzbuzz1(i: u32) -> String {
    match (i % 3, i % 5) {
        (0, 0) => "FizzBuzz".to_owned(),
        (0, _) => "Fizz".to_owned(),
        (_, 0) => "Buzz".to_owned(),
        (_, _) => format!("{}", i),
    }
}

fn fizzbuzz2(i: u32) -> String {
    match i % 15 {
        0 => "FizzBuzz".to_owned(),
        3 | 6 | 9 | 12 => "Fizz".to_owned(),
        5 | 10 => "Buzz".to_owned(),
        _ => format!("{}", i),
    }
}

fn main() {
    for i in 0..31 {
        println!("{:10} {:10}", fizzbuzz1(i), fizzbuzz2(i));
    }
}
