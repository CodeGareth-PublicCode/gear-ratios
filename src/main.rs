fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use itertools::{interleave, peek_nth, Itertools};
    use ndarray::Array;
    use std::collections::HashMap;

    #[test]
    fn basic_coordinates_navigation() {
        use itertools::Itertools;

        let first_row: Vec<&str> = vec!["4", "6", "7", ".", ".", "1", "1", "4", ".", "."];
        let second_row: Vec<&str> = vec![".", ".", ".", "*", ".", ".", ".", ".", ".", "."];
        let third_row: Vec<&str> = vec![".", ".", "3", "5", ".", ".", "6", "3", "3", "."];

        let giant: Vec<&str> = vec![
            "4", "6", "7", ".", ".", "1", "1", "4", ".", ".", ".", ".", ".", "*", ".", ".", ".",
            ".", ".", ".", ".", ".", "3", "5", ".", ".", "6", "3", "3", ".",
        ];

        let _array = Array::from_shape_vec((3, 10), giant);
        dbg!(_array);
    }
}
