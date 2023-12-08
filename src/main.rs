use ndarray::{Array, Dim, Ix};

fn main() {
    println!("Hello, world!");
}

fn logic_docs() {
    /// Dot inverse structure
    /// a1 a2
    /// b1 b2
    ///
    /// Dot inverse raw
    /// 7 .
    /// . *
    ///
    /// Dot inverse finding processed (any not a ".")
    /// T F
    /// F T
    ///
    /// Iterate through all grid combos to find TF or FT to find number adjacent to symbol
    ///
    /// (0,0)(0,1) = T F
    /// (1,0)(1,1) = F T
    ///
    /// (0,0)(1,0) = T F
    /// (0,1)(1,1) = F T
    ///
    /// (1,0)(0,1) = F, F
    ///
    /// (0,0),(1,1) = T, T <-- HIT
    ///
    /// On hit check against master grid (need to know actual coordinates to trace back
    /// fold windows iterator, chunks of x windows, 1 chunk = anchor row, x windows = columns)
    ///
    /// For all windows ,
    /// chunk by number of columns - 1,
    /// 1 chunk = 1 anchor row,
    /// 1 window = 1 anchor column
    ///
    ///
    println!("brain-dump")
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use ndarray::iter::Windows;
    use ndarray::ArrayView;
    use ndarray::Dim;
    use ndarray::{Array, Ix};
    use std::collections::HashMap;
    use tracing::event;
    use tracing::{span, Level};

    #[test]
    fn test_masking_logic_to_find_all_that_are_not_a_dot() {
        let giant: Vec<&str> = vec![
            "4", "6", "7", ".", ".", "1", "1", "4", ".", ".", ".", ".", ".", "*", ".", ".", ".",
            ".", ".", ".", ".", ".", "3", "5", ".", ".", "6", "3", "3", ".",
        ];

        // Master multidimensional array
        // [[4, 6, 7, ., ., 1, 1, 4, ., .],
        // [., ., ., *, ., ., ., ., ., .],
        // [., ., 3, 5, ., ., 6, 3, 3, .]]

        let binding = Array::from_shape_vec((3, 10), giant).unwrap();

        // Iter of 2x2 windows
        // [[4,6]
        //  [.,.]]
        //
        // [[6,7]
        // [.,.]]
        //
        // [[7,.]
        // [.,*]]

        let windows: Vec<ArrayView<&str, Dim<[Ix; 2]>>> =
            binding.windows(Dim((2, 2))).into_iter().collect();

        // Iter of 2x2 masked windows
        // [[T,T]
        //  [F,F]]
        //
        // [[T,T]
        // [F,F]]
        //
        // [[T,F]
        // [F,T]]

        let not_a_dot_mask: Vec<Array<bool, Dim<[Ix; 2]>>> = windows
            .iter()
            .map(|window| window.map(|element| element.to_owned() != "."))
            .collect();

        // Test you can parse a window as expected
        // [[4,6]
        //  [.,.]]
        //
        // [[6,7]
        //  [.,.]]
        //
        // [[7,.] <---- should be [T,F] referenced as a1, a2
        //  [.,*]] <---- should be [F,T] referenced as b1, b2

        let a1: bool = not_a_dot_mask
            .get(2)
            .unwrap()
            .get((0, 0))
            .unwrap()
            .to_owned();
        let a2: bool = not_a_dot_mask
            .get(2)
            .unwrap()
            .get((0, 1))
            .unwrap()
            .to_owned();
        let b1: bool = not_a_dot_mask
            .get(2)
            .unwrap()
            .get((1, 0))
            .unwrap()
            .to_owned();
        let b2: bool = not_a_dot_mask
            .get(2)
            .unwrap()
            .get((1, 1))
            .unwrap()
            .to_owned();

        assert_eq!(a1, true);
        assert_eq!(a2, false);
        assert_eq!(b1, false);
        assert_eq!(b2, true);

        assert_eq!((a1, b2), (true, true)); // this is the target column

        // chunked to enable enumeration to reflect row anchor
        // In a 2x2 window crawl, you shouldn't expect to see the last column as the window
        // just before should incorporate it
        let chunk_length = binding.len() - 1;
        let chunked_masks = not_a_dot_mask.chunks(chunk_length);

        assert_eq!(chunked_masks.len(), 3)
    }

    #[test]
    fn test_chunking_windows_to_be_able_to_trace_mask_array_back_to_root_array() {
        let giant: Vec<&str> = vec![
            "4", "6", "7", ".", ".", "1", "1", "4", ".", ".", ".", ".", ".", "*", ".", ".", ".",
            ".", ".", ".", ".", ".", "3", "5", ".", ".", "6", "3", "3", ".",
        ];

        // Master multidimensional array
        // [[4, 6, 7, ., ., 1, 1, 4, ., .],
        // [., ., ., *, ., ., ., ., ., .],
        // [., ., 3, 5, ., ., 6, 3, 3, .]]

        let binding = Array::from_shape_vec((3, 10), giant).unwrap();
        println!("{}", &binding);
        // Iter of 2x2 windows
        // [[4,6]
        //  [.,.]]
        //
        // [[6,7]
        // [.,.]]
        //
        // [[7,.]
        // [.,*]]

        // a 3 x 10 grid, converted to 2 x 2s, provides 18 window slices

        // an a x b grid converts to
        // (a-1) x (b-1) = number of windows
        // number of windows x 6 = combos to parse
        // so a 3 x 10 grid converts to
        // (2 * 9) = 18 windows
        // 18 * 6 = 108 combos to parse

        let windows: Vec<ArrayView<&str, Dim<[Ix; 2]>>> =
            binding.windows(Dim((2, 2))).into_iter().collect();

        // Iter of 2x2 masked windows
        // [[T,T]
        //  [F,F]]
        //
        // [[T,T]
        // [F,F]]
        //
        // [[T,F]
        // [F,T]]

        let not_a_dot_mask: Vec<Array<bool, Dim<[Ix; 2]>>> = windows
            .iter()
            .map(|window| window.map(|element| !(element.to_owned() == ".")))
            .collect();

        let chunk_length = binding.row(0).len() - 1;

        let mut window_store: HashMap<usize, &[Array<bool, Dim<[Ix; 2]>>]> = HashMap::new();

        // Chunk per row, store anchor reference,
        // This means when you iterate through hashmap values
        // You can then enumerate the values, and each position = anchor column
        // Knowing the anchor column and anchor row, means you have the opportunity to track position
        // in the source main array

        // let mut numbers_adjacent_to_symbol_store: HashMap<K, V> = HashMap::new();

        for (anchor_row, chunk) in not_a_dot_mask.chunks(chunk_length).enumerate() {
            for (chunk_column, window_element) in chunk.iter().enumerate() {
                let a1: bool = window_element.get((0, 0)).unwrap().to_owned();
                let a2: bool = window_element.get((0, 1)).unwrap().to_owned();
                let b1: bool = window_element.get((1, 0)).unwrap().to_owned();
                let b2: bool = window_element.get((1, 1)).unwrap().to_owned();

                let combinations: [(bool, bool); 6] =
                    [(a1, a2), (b1, b2), (a1, b1), (a2, b2), (b1, a2), (a1, b2)];

                for (position, &combo) in combinations.iter().enumerate() {
                    // Find a position where there's a number number,
                    // or number symbol in any direction
                    if combo == (true, true) {
                        let (position_a, position_b) = map_position_to_coordinate(position);
                        println!("Testing {:?} and {:?}", position_a, position_b);

                        // Offset to get back to original array X x X array
                        let original_a_y_axis = anchor_row + position_a[0];
                        let original_a_x_axis = chunk_column + position_a[1];

                        let original_b_y_axis = anchor_row + position_b[0];
                        let original_b_x_axis = chunk_column + position_b[1];

                        // Establish original value
                        let position_a: &&str =
                            binding.get((original_a_y_axis, original_a_x_axis)).unwrap();
                        let position_b: &&str =
                            binding.get((original_b_y_axis, original_b_x_axis)).unwrap();

                        // Only care if one of the two are a symbol
                        println!("Testing {} and {}", position_a, position_b);

                        if position_a.chars().any(|char| char.is_numeric())
                            && position_b.chars().any(|char| !char.is_numeric())
                        {
                            // now you know that position a is near a symbol you want to
                            // then look across the columns to trace the overall number
                            // but because if a symbol is adjacent to the middle of a number
                            // you'll hit duplicates, it means, you need to store and reference by
                            // coordinate value
                            //
                            // dbg!(position_a);
                            // dbg!(original_a_x_axis);

                            let mut search_row: Vec<&&str> =
                                binding.row(original_a_y_axis).into_iter().collect();
                            let mut number_store: Vec<i32> = vec![];

                            // Search from first up until the number found
                            for element in &search_row[0..original_a_x_axis + 1] {
                                if element.chars().any(|char| char.is_numeric()) {
                                    let stored_number = element.clone().parse::<i32>().unwrap();
                                    number_store.push(stored_number);
                                    (dbg!(element));
                                }
                            }

                            for element in &search_row[original_a_x_axis+1..chunk.len()] {
                                if element.chars().any(|char| char.is_numeric()) {
                                    let stored_number = element.clone().parse::<i32>().unwrap();
                                    number_store.push(stored_number);
                                    (dbg!(element));
                                } else {
                                    break;
                                }
                            }

                            let stuff: String = number_store.iter().map(ToString::to_string).collect();
                            dbg!(&stuff);
                            println!("Bro found a number {}", stuff);


                            // println!("{}{}", first_dot_found_next_to_number, last_dot_found_next_to_number);
                            // dbg!(search_row[first_dot_found_next_to_number..last_dot_found_next_to_number]);
                        } else if position_a.chars().any(|char| !char.is_numeric())
                            && position_b.chars().any(|char| char.is_numeric())
                        {
                            // now you know that position b is near a symbol you want to
                            // then look across the columns to trace the overall number
                            // but because if a symbol is adjacent to the middle of a number
                            // you'll hit duplicates, it means, you need to store and reference by
                            // coordinate value

                            let search_row: Vec<&&str> =
                                binding.row(original_b_y_axis).into_iter().collect();

                            dbg!(&search_row);
                            dbg!(&search_row[0..original_b_x_axis + 1]);
                            dbg!(&search_row[original_b_x_axis..chunk.len()]);
                        } else {
                            println!("Pair are both numeric and therefore not relevant to close to a symbol")
                        }
                    }
                }
            }
        }
    }

    fn map_position_to_coordinate(position: usize) -> ([usize; 2], [usize; 2]) {
        // [(a1, a2),
        //  (b1, b2),
        //  (a1, b1),
        //  (a2, b2),
        //  (b1, a2),
        //  (a1, b2)];

        match position {
            0 => ([0, 0], [0, 1]),
            1 => ([1, 0], [1, 1]),
            2 => ([0, 0], [1, 0]),
            3 => ([0, 1], [1, 1]),
            4 => ([1, 0], [0, 1]),
            5 => ([0, 0], [1, 1]),
            _ => ([9, 9], [9, 9]),
        }
    }
}
