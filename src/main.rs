use ndarray::{Array, ArrayBase, ArrayView, Dim, Ix, OwnedRepr};
use std::collections::HashSet;
use std::slice::Chunks;

fn main() {
    println!("Hello");
}

pub fn parse_string_to_array(input: &str) -> Vec<String> {
    input
        .chars()
        .into_iter()
        .map(|char| String::from(char))
        .collect()
}

pub fn form_multidimensional_array(
    input: &Vec<String>,
    shape: (usize, usize),
) -> Array<String, Dim<[Ix; 2]>> {
    Array::from_shape_vec(shape, input.to_vec()).unwrap()
}

pub fn form_window_array(
    input: &ArrayBase<OwnedRepr<String>, Dim<[Ix; 2]>>,
) -> Vec<ArrayView<String, Dim<[Ix; 2]>>> {
    let windows: Vec<ArrayView<String, Dim<[Ix; 2]>>> =
        input.windows(Dim((2, 2))).into_iter().collect();
    windows
}

pub fn form_boolean_mask(
    input: &Vec<ArrayView<String, Dim<[Ix; 2]>>>,
) -> Vec<Array<bool, Dim<[Ix; 2]>>> {
    input
        .iter()
        .map(|window| window.map(|element| !(element.to_owned() == ".")))
        .collect()
}

pub fn chunk_window_array(
    input: &Vec<Array<bool, Dim<[Ix; 2]>>>,
    chunk_size: usize,
) -> Chunks<Array<bool, Dim<[Ix; 2]>>> {
    input.chunks(chunk_size)
}

pub fn map_position_to_coordinate(position: usize) -> ([usize; 2], [usize; 2]) {
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

pub fn offset_coordinates_to_trace_back_to_original_array(
    anchor_row: usize,
    chunk_column: usize,
    position_a: [usize; 2],
    position_b: [usize; 2],
) -> ((usize, usize), (usize, usize)) {
    let offset_position_a_y_axis = anchor_row + position_a[0];
    let offset_position_a_x_axis = chunk_column + position_a[1];

    let offset_original_b_y_axis = anchor_row + position_b[0];
    let offset_original_b_x_axis = chunk_column + position_b[1];

    (
        (offset_position_a_y_axis, offset_position_a_x_axis),
        (offset_original_b_y_axis, offset_original_b_x_axis),
    )
}

pub fn extract_coordinates_for_all_non_dot_pairings(
    input: &Chunks<Array<bool, Dim<[Ix; 2]>>>,
) -> HashSet<((usize, usize), (usize, usize))> {
    let mut non_dot_pairing_coordinates: HashSet<((usize, usize), (usize, usize))> = HashSet::new();

    // For chunks of windows that are working horizontal across array
    // and move 1 row down at end of every chunk row
    for (anchor_row, chunk) in input.clone().enumerate() {
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
                    let (window_position_a, window_position_b) =
                        map_position_to_coordinate(position);

                    // Knowing the window coordinates, offset to trace
                    // coordinates in the original array
                    let (original_position_a, original_position_b) =
                        offset_coordinates_to_trace_back_to_original_array(
                            anchor_row,
                            chunk_column,
                            window_position_a,
                            window_position_b,
                        );

                    non_dot_pairing_coordinates.insert((original_position_a, original_position_b));
                }
            }
        }
    }

    non_dot_pairing_coordinates
}

pub fn extract_non_boolean_pair_values_from_original_multidimensional_array(
    input_array: &Array<String, Dim<[Ix; 2]>>,
    coordinates: &((usize, usize), (usize, usize)),
) -> (String, String) {
    let position_a: String = String::from(input_array.get(coordinates.0).unwrap());
    let position_b: String = String::from(input_array.get(coordinates.1).unwrap());
    (position_a, position_b)
}

pub fn trace_whole_number_from_array_following_coordinates(
    input_array: &Array<String, Dim<[Ix; 2]>>,
    coordinates: &(usize, usize),
) -> (String, String) {
    let search_row: Vec<_> = input_array.row(coordinates.0).into_iter().collect();
    let mut number_store: Vec<i32> = vec![];
    let mut coordinate_store: Vec<i16> = vec![];

    let reversed_search_slice: Vec<_> = search_row[0..coordinates.1].iter().rev().collect();
    let forward_search_slice: Vec<_> = search_row[coordinates.1..search_row.len()].iter().collect();

    // println!("\n");
    // dbg!(&coordinates);
    // dbg!(&reversed_search_slice);
    // dbg!(&forward_search_slice);
    // println!("\n");

    // Search from first up until the number found
    for (position, element) in reversed_search_slice.iter().enumerate() {
        if element.chars().any(|char| char.is_numeric()) {
            let stored_number = element.parse::<i32>().unwrap();
            number_store.push(stored_number);

            // println!("Reverse lookup: {}-{}={} from {:?}", coordinates.1, position, coordinates.1 - position, &reversed_search_slice);
            coordinate_store.push((coordinates.1 - position) as i16);
        } else {
            break;
        }
    }

    number_store.reverse();
    coordinate_store.reverse();

    for (position, element) in forward_search_slice.iter().enumerate() {
        if element.chars().any(|char| char.is_numeric()) {
            let stored_number = element.parse::<i32>().unwrap();
            number_store.push(stored_number);

            // println!("Forward lookup: {}+{}={} {:?}", coordinates.1, position, coordinates.1 + position + 1, &forward_search_slice);
            coordinate_store.push((coordinates.1 + position + 1) as i16);
        } else {
            break;
        }
    }

    coordinate_store.push(coordinates.0 as i16);

    let result: String = number_store.iter().map(ToString::to_string).collect();
    let coordinate_result: String = coordinate_store.iter().map(ToString::to_string).collect();

    // println!("{} from {:?}", &coordinate_result, &coordinate_store);

    (result, coordinate_result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::Array2;
    use std::collections::HashMap;

    #[test]
    fn test_parse_string_to_array() {
        let giant_string = "467..114.....*";
        let array = parse_string_to_array(giant_string);
        let expected_array = vec![
            "4", "6", "7", ".", ".", "1", "1", "4", ".", ".", ".", ".", ".", "*",
        ];
        assert_eq!(array, expected_array);
    }

    #[test]
    fn test_multidimensional_array() {
        let giant_string = "467..114.....*";
        let array = parse_string_to_array(giant_string);
        let expected_array = vec![
            "4", "6", "7", ".", ".", "1", "1", "4", ".", ".", ".", ".", ".", "*",
        ];

        let multi_array = form_multidimensional_array(&array, (7, 2));
        let expected = Array2::from_shape_vec((7, 2), expected_array).unwrap();
        assert_eq!(multi_array, expected);
    }

    #[test]
    fn test_window_array() {
        let giant_string = "467..114.....*";
        let array = parse_string_to_array(giant_string);
        let multi_array = form_multidimensional_array(&array, (7, 2));
        let window_array = form_window_array(&multi_array);

        let expected = Array2::from_shape_vec((2, 2), vec!["4", "6", "7", "."]).unwrap();
        assert_eq!(expected, window_array.get(0).unwrap())
    }

    #[test]
    fn test_form_boolean_mask() {
        let giant_string = "467..114.....*";
        let array = parse_string_to_array(giant_string);
        let multi_array = form_multidimensional_array(&array, (2, 7));
        let window_array = form_window_array(&multi_array);

        let boolean_mask = form_boolean_mask(&window_array);
        let expected = Array2::from_shape_vec((2, 2), vec![true, true, true, false]).unwrap();

        assert_eq!(boolean_mask.get(0).unwrap(), expected);
    }

    #[test]
    fn test_form_chunk_boolean_window_array() {
        let giant_string = "467..114.....*";
        let array = parse_string_to_array(giant_string);
        let multi_array = form_multidimensional_array(&array, (2, 7));

        let window_array = form_window_array(&multi_array);
        let boolean_mask = form_boolean_mask(&window_array);
        let chunked_window_array = chunk_window_array(&boolean_mask, 6);

        // Given a max row of 7, you will never iterate on to just the last column
        // You will stop short in order to satisfy the 2 x 2 window
        assert_eq!(chunked_window_array.len(), 1);
    }

    #[test]
    fn test_extract_non_dot_pairings_from_array() {
        let test_string = "467..114.....*........35..633.";
        let array = parse_string_to_array(test_string);
        let multi_array = form_multidimensional_array(&array, (3, 10));

        let window_array = form_window_array(&multi_array);
        let boolean_mask = form_boolean_mask(&window_array);
        let chunked_window_array = chunk_window_array(&boolean_mask, 9);

        let coordinate_pairings: HashSet<((usize, usize), (usize, usize))> =
            extract_coordinates_for_all_non_dot_pairings(&chunked_window_array);

        // Coordinates reflect not-dot coordinate pairings reading from left to right
        // [[4, 6, 7, ., ., 1, 1, 4, ., .],
        //  [., ., ., *, ., ., ., ., ., .],
        //  [., ., 3, 5, ., ., 6, 3, 3, .]]

        let expected: HashSet<((usize, usize), (usize, usize))> = HashSet::from([
            ((0, 0), (0, 1)),
            ((0, 1), (0, 2)),
            ((0, 2), (1, 3)),
            ((0, 5), (0, 6)),
            ((0, 6), (0, 7)),
            ((1, 3), (2, 3)),
            ((2, 2), (1, 3)),
            ((2, 2), (2, 3)),
            ((2, 6), (2, 7)),
            ((2, 7), (2, 8)),
        ]);

        assert_eq!(coordinate_pairings, expected);
    }

    #[test]
    fn test_extract_non_boolean_value_from_array() {
        let test_string = "467..114.....*........35..633.";
        let array = parse_string_to_array(test_string);
        let multi_array = form_multidimensional_array(&array, (3, 10));

        let test_coordinate_pair: ((usize, usize), (usize, usize)) = ((0, 0), (0, 1));

        let (non_boolean_a, non_boolean_b) =
            extract_non_boolean_pair_values_from_original_multidimensional_array(
                &multi_array,
                &test_coordinate_pair,
            );

        assert_eq!(non_boolean_a, "4");
        assert_eq!(non_boolean_b, "6");
    }

    #[test]
    fn test_extract_numbers_adjacent_to_symbols_and_track_total_coordinates() {
        let test_string = "467..114.....*........35..633.";
        let array = parse_string_to_array(test_string);
        let multi_array = form_multidimensional_array(&array, (3, 10));

        let window_array = form_window_array(&multi_array);
        let boolean_mask = form_boolean_mask(&window_array);
        let chunked_window_array = chunk_window_array(&boolean_mask, 9);

        let coordinate_pairings: HashSet<((usize, usize), (usize, usize))> =
            extract_coordinates_for_all_non_dot_pairings(&chunked_window_array);

        // print!("{}", &multi_array);
        // println!("");
        // println!("{:?}", &coordinate_pairings);

        let filtered_pairing: Vec<_> = coordinate_pairings
            .iter()
            .filter(|pairing| {
                let non_boolean_values =
                    extract_non_boolean_pair_values_from_original_multidimensional_array(
                        &multi_array,
                        &pairing,
                    );

                let position_a_is_numeric =
                    non_boolean_values.0.chars().any(|char| char.is_numeric())
                        && non_boolean_values.1.chars().any(|char| !char.is_numeric());

                let position_b_is_numeric =
                    non_boolean_values.0.chars().any(|char| !char.is_numeric())
                        && non_boolean_values.1.chars().any(|char| char.is_numeric());

                if position_a_is_numeric || position_b_is_numeric {
                    true
                } else {
                    false
                }
            })
            .collect();

        // println!("\n");
        // println!("{:?}", &filtered_pairing);
        // println!("\n");

        let mut coordinates_and_value_map: HashMap<String, String> = HashMap::new();

        for filtered_pair in filtered_pairing.iter() {
            let target_number: &&((usize, usize), (usize, usize)) = filtered_pair;
            let non_boolean_pair =
                extract_non_boolean_pair_values_from_original_multidimensional_array(
                    &multi_array,
                    &target_number,
                );

            let position_a_is_numeric = non_boolean_pair.0.chars().any(|char| char.is_numeric())
                && non_boolean_pair.1.chars().any(|char| !char.is_numeric());

            let position_b_is_numeric = non_boolean_pair.0.chars().any(|char| !char.is_numeric())
                && non_boolean_pair.1.chars().any(|char| char.is_numeric());

            if position_a_is_numeric {
                let traced_number = trace_whole_number_from_array_following_coordinates(
                    &multi_array,
                    &target_number.0,
                );
                coordinates_and_value_map
                    .entry(traced_number.1)
                    .or_insert(traced_number.0);
            } else if position_b_is_numeric {
                let traced_number = trace_whole_number_from_array_following_coordinates(
                    &multi_array,
                    &target_number.1,
                );
                coordinates_and_value_map
                    .entry(traced_number.1)
                    .or_insert(traced_number.0);
            } else {
                ()
            }
        }

        assert_eq!(
            coordinates_and_value_map
                .values()
                .into_iter()
                .collect::<Vec<_>>(),
            vec!["35", "467"]
        )
    }


    #[test]
    fn test_extract_numbers_adjacent_to_symbols_based_on_part_1_test_case() {

        let giant_string = "467..114.....*........35..633.......#...617*...........+.58...592...........755....$.*.....664.598..";

        let array = parse_string_to_array(giant_string);
        let multi_array = form_multidimensional_array(&array, (10, 10));

        let window_array = form_window_array(&multi_array);
        let boolean_mask = form_boolean_mask(&window_array);
        let chunked_window_array = chunk_window_array(&boolean_mask, 9);

        let coordinate_pairings: HashSet<((usize, usize), (usize, usize))> =
            extract_coordinates_for_all_non_dot_pairings(&chunked_window_array);

        // print!("{}", &multi_array);
        // println!("");
        // println!("{:?}", &coordinate_pairings);

        let filtered_pairing: Vec<_> = coordinate_pairings
            .iter()
            .filter(|pairing| {
                let non_boolean_values =
                    extract_non_boolean_pair_values_from_original_multidimensional_array(
                        &multi_array,
                        &pairing,
                    );

                let position_a_is_numeric =
                    non_boolean_values.0.chars().any(|char| char.is_numeric())
                        && non_boolean_values.1.chars().any(|char| !char.is_numeric());

                let position_b_is_numeric =
                    non_boolean_values.0.chars().any(|char| !char.is_numeric())
                        && non_boolean_values.1.chars().any(|char| char.is_numeric());

                if position_a_is_numeric || position_b_is_numeric {
                    true
                } else {
                    false
                }
            })
            .collect();

        // println!("\n");
        // println!("{:?}", &filtered_pairing);
        // println!("\n");

        let mut coordinates_and_value_map: HashMap<String, String> = HashMap::new();

        for filtered_pair in filtered_pairing.iter() {
            let target_number: &&((usize, usize), (usize, usize)) = filtered_pair;
            let non_boolean_pair =
                extract_non_boolean_pair_values_from_original_multidimensional_array(
                    &multi_array,
                    &target_number,
                );

            let position_a_is_numeric = non_boolean_pair.0.chars().any(|char| char.is_numeric())
                && non_boolean_pair.1.chars().any(|char| !char.is_numeric());

            let position_b_is_numeric = non_boolean_pair.0.chars().any(|char| !char.is_numeric())
                && non_boolean_pair.1.chars().any(|char| char.is_numeric());

            if position_a_is_numeric {
                let traced_number = trace_whole_number_from_array_following_coordinates(
                    &multi_array,
                    &target_number.0,
                );
                coordinates_and_value_map
                    .entry(traced_number.1)
                    .or_insert(traced_number.0);
            } else if position_b_is_numeric {
                let traced_number = trace_whole_number_from_array_following_coordinates(
                    &multi_array,
                    &target_number.1,
                );
                coordinates_and_value_map
                    .entry(traced_number.1)
                    .or_insert(traced_number.0);
            } else {
                ()
            }
        }

        let established_values_adjacent_to_symbol: i32 = coordinates_and_value_map
            .values()
            .into_iter().map(|element| element.parse::<i32>().unwrap()).sum();

        assert_eq!(established_values_adjacent_to_symbol, 4361);
    }

}
