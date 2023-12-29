fn main() {
    func(30);
    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1 // kinda like return because no ;
    };

    println!("The value of y is: {y}");

    let f = five();
    println!("The value of f is: {f}");

    let x = plus_one(5);

    println!("Plus one: {x}");

    let condition = true;
    let number = if condition { five() } else { 6 };

    println!("The value of number is: {number}");

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // Acts as return out of the loop
        }
    };

    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("Remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }

    let celsius = fahrenheit_to_celsius(100.0);
    println!("Celsius {celsius}");

    let fib = get_nth_fibonacci(5);
    println!("Fib {fib}");
}

fn fahrenheit_to_celsius(fahrenheits: f32) -> f32 {
    (fahrenheits - 32.0) * 5.0/9.0
}

fn get_nth_fibonacci(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }

    n + get_nth_fibonacci(n - 1)
}

fn func(x: i32) {
    println!("The value of x is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32  {
    x + 1
}
