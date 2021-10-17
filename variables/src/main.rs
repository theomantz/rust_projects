fn main() {

    // Example of mutable variable declaration. 
    // With mutable variables you cannot change their type.
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    
    // Constant declaration. Must have its type implied at delaration. 
    // Cannot be reassigned (obviously). Type is static (obviously).
    const MAX_POINTS: u32 = 100_000;
    println!("The max points value is: {}", MAX_POINTS);

    // Variable shadowing is different than mutability. 
    // Requires the 'let' keyword each time. Variable types can be inferred and 
    // it can change
    let y = 5;
    println!("The value of y is: {}", y);
    let y = y + 1;
    println!("The value of y is: {}", y);
    let y = "beans";
    println!("The value of y is: {}", y);

}
