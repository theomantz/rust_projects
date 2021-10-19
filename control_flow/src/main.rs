// Deciding whether or not to run some code depending on if a condition is true
// and deciding to run code repeatedly _while_ a condition is true are the basic
// building blocks in most programming languages.

// The most common constructs that allow control flow in Rust are `if` expressions
// and loops

fn main() {
    // If expressions 
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // All `if` expressions start with the keyword `if` and followed by a condition
    // the block of code we want to execute if the condition is true is placed
    // immediately after the condition inside curly brackets

    // blocks of code associated with the conditions `if` expressions are sometimes
    // called _arms_ just like the arms in `match` expressions 

    // optionally we can include an else expression to give the program an alternative
    // block of code to execute should the condition evaluate to false

    // if you dont provide an `else` expression and the condition is false the program
    // will just skip the `if` block and move on the the next bit of code

    // if we change the above code around and create a function for it

    if_else(5, 7);

    // the condition in an `if` expression must be a bool

    // you can have multiple conditions by combining `if` and `else` in an `else if`
    // expression 

    if_else_else_if(6);

    // because if is an _expression_ we can use it on the right side of a `let`
    // statement 

    if_and_let(true);

    // blocks of code evaluate to the last expression in them, and numbers are also
    // expressions
    // values that have the potential to be results from reach arm of the `if` expression
    // must be of the same type

    // The following function shows that when types are mismatched, we get an error

    // if_mismatched_types(5, "six");

    // Repetition with Loops

    // Rust provides several loops. A loop runs throuigh the code inside the loop body
    // to the end then starts immediately back at the beginning. 
}

fn if_else(x: i32, y: i32) {
    let number = x;

    if number < x {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    println!("the numbers x and y were {} and {}, respectively", x, y);
}

fn if_else_else_if(x: i32) {
    let number = x;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }
}

fn if_and_let(cond: bool) {
    let condition = cond;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}

// fn if_mismatched_types(x: i32, y: String) {
//     let condition = true;

//     let number = if condition { x } else { y };

//     println!("The value of the number is: {}", number);
// }