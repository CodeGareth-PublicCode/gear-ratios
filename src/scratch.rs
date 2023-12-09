use ndarray::{Array, Dim, Ix};

fn main() {
    println!("Hello, world!");
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
        let mut final_number_store: HashMap<usize, i32> = HashMap::new();

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
                                let result: String =
                                    number_store.iter().map(ToString::to_string).collect();
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
                            for element in &search_row[original_b_x_axis + 1..chunk.len()] {
                                if element.chars().any(|char| char.is_numeric()) {
                                    let stored_number = element.clone().parse::<i32>().unwrap();
                                    number_store.push(stored_number);
                                } else {
                                    break;
                                }
                            }

                            // Now you've finished looking between "." and established the number,
                            // Return the found number
                            let result: String =
                                number_store.iter().map(ToString::to_string).collect();
                            final_number_store.push(result.parse().unwrap());
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        assert_eq!(final_number_store, [467, 35]);
    }

    #[test]
    fn test_sample_case_on_part_1_from_site() {
        let giant_string = "467..114.....*........35..633.......#...617*...........+.58...592...........755....$.*.....664.598..";
        let giant_vec: Vec<String> = giant_string
            .chars()
            .map(|char| String::from(char))
            .collect();
        let giant = giant_vec.iter().map(|element| element.as_str()).collect();

        // Form big multidimensional array
        let binding = Array::from_shape_vec((10, 10), giant).unwrap();
        println!("{}", &binding);
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
                        let (window_position_a, window_position_b) = map_position_to_coordinate(position);

                        // Offset to get back to original array X x X array
                        let original_a_y_axis = anchor_row + window_position_a[0];
                        let original_a_x_axis = chunk_column + window_position_a[1];

                        let original_b_y_axis = anchor_row + window_position_b[0];
                        let original_b_x_axis = chunk_column + window_position_b[1];

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
                                println!("Testing {} and {}", position_a, position_b);

                                // Begin the process of looking across the row to parse where
                                // a solid number is delimited by "."s
                                let mut search_row: Vec<&&str> =
                                    binding.row(original_a_y_axis).into_iter().collect();
                                let mut number_store: Vec<i32> = vec![];
                                let mut coordinate_store: Vec<usize> = vec![];
                                coordinate_store.push(original_a_y_axis);

                                // Search from first up until the number found
                                let search_row_clone = search_row.clone();
                                let reversed_clone: Vec<&&&str> = search_row_clone
                                    [0..original_a_x_axis + 1]
                                    .iter()
                                    .rev()
                                    .collect();
                                let forward_lookup: Vec<&&&str> = search_row_clone
                                [original_a_x_axis + 1..chunk.len()]
                                .iter()
                                .collect();


                                dbg!(&search_row[original_a_x_axis + 1..chunk.len()]);
                                dbg!(&reversed_clone[0..original_a_x_axis + 1]);
                                // If you've pushed anything from here, you must reverse it back to original order
                                // This is a trick to help avoid reading multiple numbers on a line
                                for (position, element) in reversed_clone.iter().enumerate() {
                                    if element.chars().any(|char| char.is_numeric()) {
                                        let stored_number = element.clone().parse::<i32>().unwrap();
                                        let relative_position = original_a_x_axis-position;
                                        coordinate_store.push(relative_position);
                                        println!("Relative position: {}", relative_position);
                                        number_store.push(stored_number);
                                    } else {
                                        break;
                                    }
                                }

                                number_store.reverse();

                                // Continue after the found number to see if it continues, if not break
                                for (position, element) in forward_lookup.iter().enumerate() {
                                    if element.chars().any(|char| char.is_numeric()) {
                                        let stored_number = element.clone().parse::<i32>().unwrap();
                                        let relative_position = original_a_x_axis+position;
                                        number_store.push(stored_number);
                                        coordinate_store.push(relative_position)
                                    } else {
                                        break;
                                    }
                                }

                                // Now you've finished looking between "." and established the number,
                                // Store key value as established coordinates : concatenated value
                                // This helps overlapping windows
                                let coordinate_result: String =
                                    coordinate_store.iter().map(ToString::to_string).collect();
                                println!("This made {}", coordinate_result);

                                let result: String =
                                    number_store.iter().map(ToString::to_string).collect();
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
                            println!("Testing {} and {}", position_a, position_b);


                            let search_row: Vec<&&str> =
                                binding.row(original_b_y_axis).into_iter().collect();

                            let search_row_clone = search_row.clone();
                            let reversed_clone: Vec<&&&str> = search_row_clone
                                [0..original_b_x_axis + 1]
                                .iter()
                                .rev()
                                .collect();

                            let forward_lookup: Vec<&&&str> = search_row_clone
                                [original_b_x_axis + 1..chunk.len()]
                                .iter()
                                .collect();

                            let mut number_store: Vec<i32> = vec![];
                            let mut coordinate_store: Vec<usize> = vec![];
                            coordinate_store.push(original_b_y_axis);

                            // Search from first up until the number found
                            for (position, element) in reversed_clone.iter().enumerate() {
                                if element.chars().any(|char| char.is_numeric()) {
                                    let stored_number = element.clone().parse::<i32>().unwrap();
                                    let relative_position = original_b_x_axis-1;
                                    number_store.push(stored_number);
                                    coordinate_store.push(relative_position);
                                } else {
                                    break;
                                }
                            }

                            number_store.reverse();
                            // coordinate_store.reverse();

                            // Continue after the found number to see if it continues, if not break
                            for (position, element) in forward_lookup.iter().enumerate() {
                                if element.chars().any(|char| char.is_numeric()) {
                                    let stored_number = element.clone().parse::<i32>().unwrap();
                                    let relative_position = original_b_x_axis+1;
                                    number_store.push(stored_number);
                                    coordinate_store.push(relative_position)
                                } else {
                                    break;
                                }
                            }

                            // Now you've finished looking between "." and established the number,
                            // Return the found number
                            let coordinate_result: String =
                                coordinate_store.iter().map(ToString::to_string).collect();
                            println!("This made {}", coordinate_result);
                            // final_number_store.push(result.parse().unwrap());
                            let result: String =
                                number_store.iter().map(ToString::to_string).collect();
                            final_number_store.push(result.parse().unwrap());

                        } else {
                            break;
                        }
                    }
                }
            }
        }

        assert_eq!(final_number_store, [467, 35]);
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
