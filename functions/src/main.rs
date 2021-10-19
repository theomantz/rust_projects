fn main() {
    another_function(5);
    another_another_function(5, 6);

    // function bodies can contain statements and expressions

    // Creating a variable and assigning a value to it with the `let` keyword
    // is a statement

    let y = 5;

    // function definitions are also statements, the entirity of fn main() 
    // is a statement

    // Statements do not return a value. Therefore you cannot assign a `let` 
    // statement to another variable, the following code will produce an error

    // let x = (let y = 6);

    // The `let y = 6` statement does not return a value, so there is nothing
    // for x to bind to. This is different from other languages such as C and Ruby
    // where the assignment returns the value of the assignment. 

    // Expressions evaluate to something and make up most of the rest of the code
    // that you will write in Rust. Consider a simple math operation, such as `5 + 6`
    // which evaluates to 11. Expressions can be part of statements: in Listing 3-1
    // the 6 in the statement `let y = 6` is an expression that evaluates to the 
    // value 6. Calling a function is an expression. Calling a macro is an expression.
    // The block that we use to create new scopes `{}`, is an expression. For example:

    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };
    
    println!("The value of y is: {}", y);

    // Functions can return values to the code that calls them. We don't name return
    // values but we do declare their type after an arrow (->). In Rust, the return 
    // value of the function is synonymous with the value of the final expression in 
    // the block of the body of a function. You can return early from a function 
    // by using the `return` keyword and specifying a value, but most functions 
    // return the last expression implicity. Here's an example of a function that 
    // returns a value

    let x = five();

    println!("The return value of the function is: {}", x);

    // lets look at a second example
    // Running this code will print `The value of x is: 6` but if we place a semicolon
    // at the end of the line containing `x + 1`, changing it from an expression to 
    // a statement, we'll get an error.

    let x = plus_one(5);

    println!("The value of x is {}", x);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// For all function parameters, the type must be declared
fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

// For functions with multiple parameters, separate the paramter declarations
// with commas, like this

fn another_another_function(x: i32, y: i32) {
    println!("The value of x and y are {} and {} respectively", x, y);
}
