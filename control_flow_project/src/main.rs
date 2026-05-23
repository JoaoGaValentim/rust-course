fn main() {
    /*
    Define a `color_to_number` function that accepts a 'color'
    parameter (a string). Use if, else if, and else
    statements to return a corresponding numeric value based
    on the following rules:
    1. If the color is "red", return 1.
    2. If the color is "green", return 2.
    3. If the color is "blue", return 3.
    4. If the color is any other string, return 0.

    Refactor the function above to use the `match` statement
    instead of if, else if, and else.

    Define a `factorial` function that calculates the
    factorial of a number. The factorial is the product
    of multiplying a number by every incremental
    number leading up to it, starting from 1.

    Examples:
    The factorial of 5 is 5 * 4 * 3 * 2 * 1 = 120
    factorial(5) should return 120.

    The factorial of 4 is 4 * 3 * 2 * 1 = 24
    factorial(4) should return 24.

    Implement two solutions/functions for the problem.
    The first solution should not use recursion.
    The second solution should use recursion.
    */

    // 1) if/else if/else 2) match
    let color = color_to_number("red");
    println!("Red code is {color}.");
    let color = color_to_number("green");
    println!("Green code is {color}.");
    let color = color_to_number("blue");
    println!("Blue code is {color}.");
    let color = color_to_number("yellow");
    println!("Any other color code is {color}.");

    // 2) factorial with loop
    let factorial_result = factorial(5);
    println!("5 != {factorial_result}");
    let factorial_result = factorial(4);
    println!("5 != {factorial_result}");
    // 3) factorial recursive
    let factorial_result = factorial_recursive(5);
    println!("5 != {factorial_result}");
    let factorial_result = factorial_recursive(4);
    println!("5 != {factorial_result}");
}

/*
    Define a `color_to_number` function that accepts a 'color'
    parameter (a string). Use if, else if, and else
    statements to return a corresponding numeric value based
    on the following rules:
    1. If the color is "red", return 1.
    2. If the color is "green", return 2.
    3. If the color is "blue", return 3.
    4. If the color is any other string, return 0.
*/
fn color_to_number(color: &str) -> i32 {
    // if color == "red" {
    //     return 1;
    // } else if color == "green" {
    //     return 2;
    // } else if color == "blue" {
    //     return 3;
    // } else {
    //     0
    // }

    let switched_color: i32 = match color {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    };

    switched_color
}

/*
    Define a `factorial` function that calculates the
    factorial of a number. The factorial is the product
    of multiplying a number by every incremental
    number leading up to it, starting from 1.

    Examples:
    The factorial of 5 is 5 * 4 * 3 * 2 * 1 = 120
    factorial(5) should return 120.

    The factorial of 4 is 4 * 3 * 2 * 1 = 24
    factorial(4) should return 24.

    Implement two solutions/functions for the problem.
    The first solution should not use recursion.
    The second solution should use recursion.
*/
fn factorial(n: i32) -> i32 {
    let mut factorial_result = 1;

    for i in 1..=n {
        factorial_result *= i;
    }

    factorial_result
}

fn factorial_recursive(n: i32) -> i32 {
    if n <= 0 {
        return 1;
    }

    n * factorial_recursive(n - 1)
}
