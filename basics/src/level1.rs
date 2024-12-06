pub fn first_test_function() {
    println!("Let's try this.");
}

pub fn variable_test_fix() {
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {}", number);

    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    // Added "let" below
    let number = 3;
    println!("Number plus two is: {}", number + 2);
}

pub fn add_numbers(num1: i32, num2: i32) -> i32 {
    let result = num1 + num2;
    println!("Adding number {a} to {b}: {result}", a=num1, b=num2, result=result);
    result
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    #[test]
    fn ten_plus_ten_is_twenty() {
        assert_eq!(20, crate::level1::add_numbers(10, 10));
    }
}
