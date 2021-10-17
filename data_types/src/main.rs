fn main() {
    // An integer is a number without a fractional component. 
    // Integer types can be between 8 - 128 bits and signed or unsigned 
    // Signed meaning that the integer can be negative
    // Ex. u32 => unsigned 32 bit integer vs. i32 => signed 32 bit integer

    // Arch or usize/isize is dependent on the computer architecture you are using
    // Ex 64-bit arch means a 64 bit integer

    // Unsigned can be between 0 and 2 ** ( n - 1 ) - 1 while signed
    // can be between - ( 2 ** ( n - 1 ) - 1 ) and ( 2 ** ( n - 1) - 1)

    // When in doubt, use Rust defaults

    // Floating point types

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // f64 is double precision while f32 is single precision

    println!("The values of x and y are {} and {}, respectively", x, y);

    // Numeric optionations

    // Addition
    let sum = 5 + 10;
    println!("The sum is {}", sum);

    // Subtraction
    let difference = 95.5 - 4.3;
    println!("The difference is {}", difference);

    // Multiplication
    let product = 4 * 30;
    println!("The product is {}", product);

    // Division
    let quotient = 56.7 / 32.2;
    println!("The quotient is {}", quotient);

    // Remainder
    let remainder = 43 % 5;
    println!("The remainder is {}", remainder);

    // Boolean
    let t = true;
    let f: bool = false; // explicit type annotation

    println!("The two possible values for a boolean are {} and {}", t, f);
}