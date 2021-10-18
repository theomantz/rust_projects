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

    println!("FLOATING POINTS");
    println!("--------");
    println!("The values of x and y are {} and {}, respectively", x, y);
    println!("");
    // Numeric optionations
    println!("NUMERIC OPERATORS");
    println!("--------");

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
    println!("");
    println!("BOOLEANS");
    println!("--------");
    let t = true;
    let f: bool = false; // explicit type annotation
    print!("The two possible values for a boolean are {} and {}", t, f);


    println!("");
    println!("CHAR");
    println!("--------");

    // The char type is four bytes and Rust's most primitive alphabetic type
    // Char literals are specified with single quotes as opposed to string
    // Literals which use double quotes

    let c = '';
    let z = 'Ø';
    let heart_eyed_cat = '😻';
    println!("Some example chars: {}, {}, {}", c, z, heart_eyed_cat);

    println!("");
    println!("COMPOUND TYPES");
    println!("--------");

    // Compound types canm group multiple values into one type. Rust has two 
    // primitive compound types: tuples and arrays

    // Tuples are declared by writing a comma-seoparated list of values inside parenths
    // each position in the tuple has a type and the types of the different values
    // in the tuple dont have to be the same

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("the values in the tuple are {}, {}, and {}", x, y, z);

    // In addition to destructuring we can access a tuple element directly
    // by using a period (.) followed by an index

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("The values of the tuple are {}, {}, and {}", five_hundred, six_point_four, one);

    // Another way to have a collection of multiple values is to have an array
    // unlike a tuple, an array must have values of the same type
    // Arrays are written as a comma-separated list within square brackets

    let a = [1, 2, 3, 4, 5];

    // An array is static in size while a vector can grow and shrink as needed
    // if you are unsure of which type to use, use a vector

    
}