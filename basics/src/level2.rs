pub fn bigger(a: i32, b: i32) -> i32 {
    // TODO: Complete this function to return the bigger number!
    if a == b {
        a
    } else if a > b {
        a
    } else {
        b
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, crate::level2::bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, crate::level2::bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, crate::level2::bigger(42, 42));
    }

    #[test]
    fn indexing_tuple() {
        let numbers = (1, 27, 3);

        // TODO: Use a tuple index to access the second element of `numbers`
        // and assign it to a variable called `second`.
        let second = numbers.1;

        assert_eq!(second, 27, "This is not the 2nd number in the tuple!");
    }
}