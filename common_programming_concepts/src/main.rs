fn main() {
    // If mut wasn't here the code wouldn't compile
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    // const must have a type defined, https://doc.rust-lang.org/reference/const_eval.html
    const SECONDS_IN_HOUR: u32 = 60 * 60;
    println!("The value of const is: {SECONDS_IN_HOUR}");

    // Shadowing
    let y = 10;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y is: {y}");

    // Why shadowing is useful
    let spaces = "    ";
    println!("Spaces: {spaces}");
    let spaces = spaces.len();
    println!("Spaces: {spaces}");
    // this gives a compiler erro because spaces can't be assigned an int later
    // let mut spaces = "    ";
    // spaces = spaces.len();

    // Data types
    // without expect we get an error because parse returns a result and not a u32
    // without u32 we get an error because rust can't determine what type we want
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Guess: {guess}");

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // Destructuring a tuple
    let (_x, y, _z) = tup;

    println!("The value of y is: {y}");

    // Arrays -> fixed length
    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    // array of size 5 with init value 3 for all entries
    let _b = [3; 5];

    // This will crash -> different from ex. C++
    // let bla = b[6];
}
