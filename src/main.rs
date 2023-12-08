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

        // Form big multidimensional array
        let binding = Array::from_shape_vec((3, 10), giant).unwrap();
        let windows: Vec<ArrayView<&str, Dim<[Ix; 2]>>> =
            binding.windows(Dim((2, 2))).into_iter().collect();

        // Convert to T or F whether it is a dot
        let not_a_dot_mask: Vec<Array<bool, Dim<[Ix; 2]>>> = windows
            .iter()
            .map(|window| window.map(|element| !(element.to_owned() == ".")))
            .collect();

        // Chunk size to help reflect rows
        let chunk_length = binding.row(0).len() - 1;
        let mut final_number_store: Vec<i32> = vec![];

        // For chunks of windows that are working horizontal across array, and move 1 row down at end of every chunk row
        for (anchor_row, chunk) in not_a_dot_mask.chunks(chunk_length).enumerate() {

            // Within that chunk, moving across columns 1 at a time
            for (chunk_column, window_element) in chunk.iter().enumerate() {

                // Establish easy grid references
                let a1: bool = window_element.get((0, 0)).unwrap().to_owned();
                let a2: bool = window_element.get((0, 1)).unwrap().to_owned();
                let b1: bool = window_element.get((1, 0)).unwrap().to_owned();
                let b2: bool = window_element.get((1, 1)).unwrap().to_owned();

                // Build out tuples reflecting every possible combo within a 2x2 grid
                let combinations: [(bool, bool); 6] =
                    [(a1, a2), (b1, b2), (a1, b1), (a2, b2), (b1, a2), (a1, b2)];

                // Your looking for a pattern of TT to match that neither are a dot
                // But 1 could be a symbol
                for (position, &combo) in combinations.iter().enumerate() {

                    if combo == (true, true) {
                        // Translate values back to coordinates of 2x2 window grid
                        let (position_a, position_b) = map_position_to_coordinate(position);

                        // Offset to get back to original array X x X array
                        let original_a_y_axis = anchor_row + position_a[0];
                        let original_a_x_axis = chunk_column + position_a[1];

                        let original_b_y_axis = anchor_row + position_b[0];
                        let original_b_x_axis = chunk_column + position_b[1];

                        // Pull original values back from first array
                        let position_a: &&str =
                            binding.get((original_a_y_axis, original_a_x_axis)).unwrap();
                        let position_b: &&str =
                            binding.get((original_b_y_axis, original_b_x_axis)).unwrap();

                        // If position a is numeric, and not a symbol like position b
                        if position_a.chars().any(|char| char.is_numeric())
                            && position_b.chars().any(|char| !char.is_numeric())
                        {
                            {
                                // Begin the process of looking across the row to parse where
                                // a solid number is delimited by "."s
                                let mut search_row: Vec<&&str> =
                                    binding.row(original_a_y_axis).into_iter().collect();
                                let mut number_store: Vec<i32> = vec![];

                                // Search from first up until the number found
                                for element in &search_row[0..original_a_x_axis + 1] {
                                    if element.chars().any(|char| char.is_numeric()) {
                                        let stored_number = element.clone().parse::<i32>().unwrap();
                                        number_store.push(stored_number);
                                    }
                                }

                                // Continue after the found number to see if it continues, if not break
                                for element in &search_row[original_a_x_axis + 1..chunk.len()] {
                                    if element.chars().any(|char| char.is_numeric()) {
                                        let stored_number = element.clone().parse::<i32>().unwrap();
                                        number_store.push(stored_number);
                                    } else {
                                        break;
                                    }
                                }

                                // Now you've finished looking between "." and established the number,
                                // Return the found number
                                let result: String = number_store.iter().map(ToString::to_string).collect();
                                final_number_store.push(result.parse().unwrap());
                            }

                        // else if the position a is a symbol and b is a numeric
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
                            let mut number_store: Vec<i32> = vec![];

                            // Search from first up until the number found
                            for element in &search_row[0..original_b_x_axis + 1] {
                                if element.chars().any(|char| char.is_numeric()) {
                                    let stored_number = element.clone().parse::<i32>().unwrap();
                                    number_store.push(stored_number);
                                }
                            }

                            // Continue after the found number to see if it continues, if not break
                            for element in &search_row[original_b_x_axis+1..chunk.len()] {
                                if element.chars().any(|char| char.is_numeric()) {
                                    let stored_number = element.clone().parse::<i32>().unwrap();
                                    number_store.push(stored_number);
                                } else {
                                    break;
                                }
                            }

                            // Now you've finished looking between "." and established the number,
                            // Return the found number
                            let result: String = number_store.iter().map(ToString::to_string).collect();
                            final_number_store.push(result.parse().unwrap());


                        } else {
                            break
                        }
                    }
                }
            }
        }

        assert_eq!(final_number_store, [467,35]);
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
