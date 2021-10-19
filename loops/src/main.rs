fn main() {
    
    // Rusts most basic loop

    loop {
        println!("again!");
        break;
    }

    // Returning values from loops
    // one of the uses of a `loop` is to retry an operation you know might fail, 
    // such as checking whether a thread has completed its job
    // you might need to return that result to the rest of your code
    // to do this, you can add the value you want returned after the break expression 
    // you use to stop the loop; that value will be returned out of the loop so 
    // you can use it

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {}", result);

    // Conditional Loops with While

    // Its often useful for a program to evaluate a condition within a loop. 
    // While the condition is true, the loop runs. When the condition ceases to be true, 
    // the program calls `break`, stopping the loop. This loop can be implemented with 
    // `loop` `if` `else` and `break`

    // In rust there is a `while` loop. 

    let mut number = 3;

    while number != 0 {
        println!("{}", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");


    // Looping Through a Collection with `for`

    // You could use the `while` construct to loop over the elements of a collection,
    // such as an array

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }


    // this is not ideal, we can have a more performant and concise version
    // with a `for` loop 

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // heres what a countdown would look like using a `for` loop and another method
    // `rev` to reverse the range

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("LIFTOFF!!!");
    
}
