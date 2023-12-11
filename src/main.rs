use ndarray::{Array, ArrayBase, ArrayView, Dim, Ix, OwnedRepr};
use std::collections::{HashMap};
use std::slice::Chunks;

// ---- PART 2 START ----

use std::collections::HashSet;

fn main() {
    let file_path: &str = "./src/input.txt";
    let content: String = std::fs::read_to_string(file_path).expect("should read from file");
    let content_as_line = content.lines().collect::<String>();

    let array = parse_string_to_array(&content_as_line);
    let multi_array = form_multidimensional_array(&array, (140, 140));

    let window_array = form_window_array(&multi_array);
    let boolean_mask = form_boolean_mask(&window_array);
    let chunked_window_array = chunk_window_array(&boolean_mask, 139);

    let coordinate_pairings: HashSet<((usize, usize), (usize, usize))> =
        extract_coordinates_for_all_non_dot_pairings(&chunked_window_array);

    let filtered_pairing: Vec<_> = coordinate_pairings
        .iter()
        .filter(|pairing| {
            let non_boolean_values =
                extract_non_boolean_pair_values_from_original_multidimensional_array(
                    &multi_array,
                    &pairing,
                );

            let position_a_is_numeric = non_boolean_values.0.chars().any(|char| char.is_numeric())
                && non_boolean_values.1.chars().any(|char| !char.is_numeric());

            let position_b_is_numeric = non_boolean_values.0.chars().any(|char| !char.is_numeric())
                && non_boolean_values.1.chars().any(|char| char.is_numeric());

            let position_a_is_star = non_boolean_values.0 == "*";
            let position_b_is_star = non_boolean_values.1 == "*";

            if position_a_is_numeric && position_b_is_star {
                true
            } else if position_b_is_numeric && position_a_is_star {
                true
            } else {
                false
            }
        })
        .collect();

    let mut coordinates_and_value_map: HashMap<((usize, usize)), Vec<String>> = HashMap::new();

    for filtered_pair in filtered_pairing.iter() {
        let target_number: &&((usize, usize), (usize, usize)) = filtered_pair;
        let non_boolean_pair = extract_non_boolean_pair_values_from_original_multidimensional_array(
            &multi_array,
            &target_number,
        );

        let position_a_is_numeric = non_boolean_pair.0.chars().any(|char| char.is_numeric())
            && non_boolean_pair.1.chars().any(|char| !char.is_numeric());

        let position_b_is_numeric = non_boolean_pair.0.chars().any(|char| !char.is_numeric())
            && non_boolean_pair.1.chars().any(|char| char.is_numeric());

        let position_a_is_star = non_boolean_pair.0 == "*";
        let position_b_is_star = non_boolean_pair.1 == "*";

        if position_a_is_numeric && position_b_is_star {
            let traced_number =
                trace_whole_number_from_array_following_coordinates(&multi_array, &target_number.0);
            coordinates_and_value_map
                .entry(target_number.1)
                .or_insert(vec![]).push(traced_number.0);
        } else if position_b_is_numeric && position_a_is_star {
            let traced_number =
                trace_whole_number_from_array_following_coordinates(&multi_array, &target_number.1);
            coordinates_and_value_map
                .entry(target_number.1)
                .or_insert(vec![]).push(traced_number.0);
        } else {
            ()
        }

    }

    dbg!(coordinates_and_value_map);
    // let established_values_adjacent_to_symbol: i32 = coordinates_and_value_map
    //     .values()
    //     .into_iter()
    //     .map(|element| element.parse::<i32>().unwrap())
    //     .sum();
}




fn dudd_main() {
    let file_path: &str = "./src/input.txt";
    let content: String = std::fs::read_to_string(file_path).expect("should read from file");
    let content_as_line = content.lines().collect::<String>();

    let array = parse_string_to_array(&content_as_line);
    let multi_array = form_multidimensional_array(&array, (140, 140));

    let skinny_window_array = form_three_by_three_window_array(&multi_array);
    let boolean_mask = form_boolean_mask(&skinny_window_array);

    // Because you now are chunking 3x3 grids, reduce total size by 2 to reflect impossible anchor columns
    let chunked_window_array = chunk_window_array(&boolean_mask, 138);

    let coordinate_trios: HashSet<((usize, usize), (usize, usize), (usize, usize))> =
        extract_coordinates_for_all_non_dot_trios(&chunked_window_array);

    let filtered_trios: Vec<_> = coordinate_trios
        .iter()
        .filter(|trio| {
            let non_boolean_values =
                extract_non_boolean_trio_values_from_original_multidimensional_array(
                    &multi_array,
                    &trio,
                );

            let position_a_is_numeric =
                non_boolean_values.0.chars().any(|char| char.is_numeric());
            let position_b_is_star = non_boolean_values.1 == "*";
            let position_c_is_numeric =
                non_boolean_values.2.chars().any(|char| char.is_numeric());

            if position_a_is_numeric && position_b_is_star && position_c_is_numeric {
                println!("Keeping {:?}", non_boolean_values)
            }
            return position_a_is_numeric && position_b_is_star && position_c_is_numeric
        })
        .collect();

    let mut coordinates_and_value_map: HashMap<i64, i64> = HashMap::new();
    for filtered_trio in filtered_trios.iter() {
        let target_number: &&((usize, usize), (usize, usize), (usize, usize)) = filtered_trio;

        let traced_number_a =
            trace_whole_number_from_array_following_coordinates(&multi_array, &target_number.0);
        let traced_number_c =
            trace_whole_number_from_array_following_coordinates(&multi_array, &target_number.2);

        let traced_coordinates_total: i64 =
            traced_number_a.1.parse::<i64>().unwrap() + traced_number_c.1.parse::<i64>().unwrap();
        let traced_values_product: i64 =
            traced_number_a.0.parse::<i64>().unwrap() * traced_number_c.0.parse::<i64>().unwrap();

        coordinates_and_value_map
            .entry(traced_coordinates_total)
            .or_insert(traced_values_product);
    }

    let result: i64 = coordinates_and_value_map.values().into_iter().sum();

    println!("Final result: {}", result);
    assert_ne!(result, 46371710);
    assert_ne!(result, 55985365);
}

// ---- PART 1 START ----

fn part_1_main() {
    let file_path: &str = "./src/input.txt";
    let content: String = std::fs::read_to_string(file_path).expect("should read from file");
    let content_as_line = content.lines().collect::<String>();

    let array = parse_string_to_array(&content_as_line);
    let multi_array = form_multidimensional_array(&array, (140, 140));

    let window_array = form_window_array(&multi_array);
    let boolean_mask = form_boolean_mask(&window_array);
    let chunked_window_array = chunk_window_array(&boolean_mask, 139);

    let coordinate_pairings: HashSet<((usize, usize), (usize, usize))> =
        extract_coordinates_for_all_non_dot_pairings(&chunked_window_array);

    let filtered_pairing: Vec<_> = coordinate_pairings
        .iter()
        .filter(|pairing| {
            let non_boolean_values =
                extract_non_boolean_pair_values_from_original_multidimensional_array(
                    &multi_array,
                    &pairing,
                );

            let position_a_is_numeric = non_boolean_values.0.chars().any(|char| char.is_numeric())
                && non_boolean_values.1.chars().any(|char| !char.is_numeric());

            let position_b_is_numeric = non_boolean_values.0.chars().any(|char| !char.is_numeric())
                && non_boolean_values.1.chars().any(|char| char.is_numeric());

            if position_a_is_numeric || position_b_is_numeric {
                true
            } else {
                false
            }
        })
        .collect();

    let mut coordinates_and_value_map: HashMap<String, String> = HashMap::new();

    for filtered_pair in filtered_pairing.iter() {
        let target_number: &&((usize, usize), (usize, usize)) = filtered_pair;
        let non_boolean_pair = extract_non_boolean_pair_values_from_original_multidimensional_array(
            &multi_array,
            &target_number,
        );

        let position_a_is_numeric = non_boolean_pair.0.chars().any(|char| char.is_numeric())
            && non_boolean_pair.1.chars().any(|char| !char.is_numeric());

        let position_b_is_numeric = non_boolean_pair.0.chars().any(|char| !char.is_numeric())
            && non_boolean_pair.1.chars().any(|char| char.is_numeric());

        if position_a_is_numeric {
            let traced_number =
                trace_whole_number_from_array_following_coordinates(&multi_array, &target_number.0);
            coordinates_and_value_map
                .entry(traced_number.1)
                .or_insert(traced_number.0);
        } else if position_b_is_numeric {
            let traced_number =
                trace_whole_number_from_array_following_coordinates(&multi_array, &target_number.1);
            coordinates_and_value_map
                .entry(traced_number.1)
                .or_insert(traced_number.0);
        } else {
            ()
        }
    }

    let established_values_adjacent_to_symbol: i32 = coordinates_and_value_map
        .values()
        .into_iter()
        .map(|element| element.parse::<i32>().unwrap())
        .sum();

    println!("{:?}", established_values_adjacent_to_symbol);
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

pub fn form_three_by_three_window_array(
    input: &ArrayBase<OwnedRepr<String>, Dim<[Ix; 2]>>,
) -> Vec<ArrayView<String, Dim<[Ix; 2]>>> {
    let windows: Vec<ArrayView<String, Dim<[Ix; 2]>>> =
        input.windows(Dim((3, 3))).into_iter().collect();
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

pub fn map_trio_position_to_coordinate(position: usize) -> ([usize; 2], [usize; 2], [usize; 2]) {
    // [(a1, b1, c1),
    //  (a2, b2, c2),
    //  (a3, b3, c3),
    //  (a1, a2, a3)
    //  (b1, b2, b3)
    //  (c1, c2, c3)
    //  (c1, b2, a3)
    //  (a1, b2, c3),
    //  (a1, b2, c1),
    //  (a3, b2, c3)
    //  (a1, b2, c2),
    //  (a3, b2, c2),
    //  (b1, b2, a3),
    //  (b1, b2, c3)
    //  (b1, b2, a2),
    //  (b3, b2, a2),
    //  (b1, b2, c2),
    //  (b3, b2, c2),];

    match position {
        0 => ([0, 0], [1, 0], [2, 0]),
        1 => ([0, 1], [1, 1], [2, 1]),
        2 => ([0, 2], [1, 2], [2, 2]),
        3 => ([0, 0], [0, 1], [0, 2]),
        4 => ([1, 0], [1, 1], [1, 1]),
        5 => ([2, 0], [2, 1], [2, 2]),
        6 => ([2, 0], [1, 1], [0, 2]),
        7 => ([0, 1], [1, 0], [2, 2]),
        8 => ([0, 0], [1, 1], [2, 0]),
        9 => ([0, 2], [1, 1], [2, 2]),
        10 => ([0, 1], [1, 1], [2, 1]),
        11 => ([0, 2], [1, 1], [2, 1]),
        12 => ([1, 0], [1, 1], [0, 2]),
        13 => ([1, 0], [1, 1], [2, 2]),
        14 => ([1, 0], [1, 1], [0, 1]),
        15 => ([1, 2], [1, 1], [0, 2]),
        16 => ([1, 0], [1, 1], [3, 1]),
        17 => ([1, 2], [1, 1], [2, 1]),
        _ => ([9, 9], [9, 9], [9, 9]),
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

    // dbg!((
    //     (&offset_position_a_y_axis, &offset_position_a_x_axis),
    //     (&offset_original_b_y_axis, &offset_original_b_x_axis),
    // ));

    (
        (offset_position_a_y_axis, offset_position_a_x_axis),
        (offset_original_b_y_axis, offset_original_b_x_axis),
    )
}

pub fn offset_trio_coordinates_to_trace_back_to_original_array(
    anchor_row: usize,
    chunk_column: usize,
    position_a: [usize; 2],
    position_b: [usize; 2],
    position_c: [usize; 2],
) -> ((usize, usize), (usize, usize), (usize, usize)) {
    let offset_position_a_y_axis = anchor_row + position_a[0];
    let offset_position_a_x_axis = chunk_column + position_a[1];
    // assert_eq!(&offset_position_a_x_axis< &10, true);


    let offset_original_b_y_axis = anchor_row + position_b[0];
    let offset_original_b_x_axis = chunk_column + position_b[1];
    // assert_eq!(&offset_original_b_x_axis< &10, true);

    let offset_original_c_y_axis = anchor_row + position_c[0];
    let offset_original_c_x_axis = chunk_column + position_c[1];
    // dbg!(&position_c);
    // dbg!(&anchor_row);
    // dbg!(&chunk_column);

    // assert_eq!(&offset_original_c_x_axis< &10, true);

    (
        (offset_position_a_y_axis, offset_position_a_x_axis),
        (offset_original_b_y_axis, offset_original_b_x_axis),
        (offset_original_c_y_axis, offset_original_c_x_axis),
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

pub fn extract_coordinates_for_all_non_dot_trios(
    input: &Chunks<Array<bool, Dim<[Ix; 2]>>>,
) -> HashSet<((usize, usize), (usize, usize), (usize, usize))> {
    let mut non_dot_trio_coordinates: HashSet<((usize, usize), (usize, usize), (usize, usize))> =
        HashSet::new();

    // For chunks of windows that are working horizontal across array
    // and move 1 row down at end of every chunk row
    for (anchor_row, chunk) in input.clone().enumerate() {
        // Within that chunk, moving across columns 1 at a time
        for (chunk_column, window_element) in chunk.iter().enumerate() {
            // Establish easy grid references
            // println!("{}", window_element);

            let a1: bool = window_element.get((0, 0)).unwrap().to_owned();
            let a2: bool = window_element.get((0, 1)).unwrap().to_owned();
            let a3: bool = window_element.get((0, 2)).unwrap().to_owned();
            let b1: bool = window_element.get((1, 0)).unwrap().to_owned();
            let b2: bool = window_element.get((1, 1)).unwrap().to_owned();
            let b3: bool = window_element.get((1, 2)).unwrap().to_owned();
            let c1: bool = window_element.get((2, 0)).unwrap().to_owned();
            let c2: bool = window_element.get((2, 1)).unwrap().to_owned();
            let c3: bool = window_element.get((2, 2)).unwrap().to_owned();

            // Build out tuples reflecting every possible combo within a 3x3 grid
            // Avoid dumb hard coding every combo
            let variables = [a1,a2,a3,b1,b2,b3,c1,c2,c3];
            let mut combinations = Vec::new();

            // Check all combinations of three variables
            for i in 0..variables.len() {
                for j in i + 1..variables.len() {
                    for k in j + 1..variables.len() {
                        combinations.push((variables[i], variables[j], variables[k]));
                    }
                }
            }

            // Avoid dumb hard coding every combo
            let coordinate_variables = [[0,0], [0,1], [0,2], [1,0], [1,1], [1,2], [2,0], [2,1], [2,2]];
            let mut coordinate_combinations = Vec::new();

            // Check all combinations of three variables
            for i in 0..coordinate_variables.len() {
                for j in i + 1..coordinate_variables.len() {
                    for k in j + 1..coordinate_variables.len() {
                        coordinate_combinations.push((coordinate_variables[i], coordinate_variables[j], coordinate_variables[k]));
                    }
                }
            }

            // println!("Found success at {:?}", &combinations.get(9));

            // Your looking for a pattern of TTT to match that neither are a dot
            // But 1 could be a symbol
            for (position, &combo) in combinations.iter().enumerate() {
                if combo == (true, true, true) {
                    // Translate values back to coordinates of 3x3 window grid
                    let (window_position_a, window_position_b, window_position_c) =
                        coordinate_combinations.get(position).unwrap();

                    // Knowing the window coordinates, offset to trace
                    // coordinates in the original array
                    let (original_position_a, original_position_b, original_position_c) =
                        offset_trio_coordinates_to_trace_back_to_original_array(
                            anchor_row,
                            chunk_column,
                            *window_position_a,
                            *window_position_b,
                            *window_position_c,
                        );

                    non_dot_trio_coordinates.insert((
                        original_position_a,
                        original_position_b,
                        original_position_c,
                    ));
                }
            }
        }
    }

    non_dot_trio_coordinates
}

pub fn extract_non_boolean_pair_values_from_original_multidimensional_array(
    input_array: &Array<String, Dim<[Ix; 2]>>,
    coordinates: &((usize, usize), (usize, usize)),
) -> (String, String) {
    let position_a: String = String::from(input_array.get(coordinates.0).unwrap());
    let position_b: String = String::from(input_array.get(coordinates.1).unwrap());
    (position_a, position_b)
}

pub fn extract_non_boolean_trio_values_from_original_multidimensional_array(
    input_array: &Array<String, Dim<[Ix; 2]>>,
    coordinates: &((usize, usize), (usize, usize), (usize, usize)),
) -> (String, String, String) {
    let position_a: String = String::from(input_array.get(coordinates.0).unwrap());
    let position_b: String = String::from(input_array.get(coordinates.1).unwrap());
    let position_c: String = String::from(input_array.get(coordinates.2).unwrap());
    println!("{}{}{}",&position_a, &position_b, &position_c);
    (position_a, position_b, position_c)
}

pub fn trace_whole_number_from_array_following_coordinates(
    input_array: &Array<String, Dim<[Ix; 2]>>,
    coordinates: &(usize, usize),
) -> (String, String) {
    // println!("Tracing: {:?}", &input_array);

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
        let window_array =form_window_array(&multi_array);
        println!("{:?}", window_array);

        let expected = Array2::from_shape_vec((2, 2), vec!["4", "6", "7", "."]).unwrap();
        assert_eq!(expected, window_array.get(0).unwrap())
    }

    #[test]
    fn test_skinny_window_array() {
        let giant_string = "467..114.....*........35..633.......#...617*...........+.58...592...........755....$.*.....664.598..";
        let array = parse_string_to_array(giant_string);
        let multi_array = form_multidimensional_array(&array, (10, 10));
        let window_array = form_three_by_three_window_array(&multi_array);

        dbg!(&window_array);


        let expected = Array2::from_shape_vec((3, 3), vec!["4", "6", "7", ".", ".", ".",".",".","3"]).unwrap();
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
    fn test_extract_non_dot_trios_from_array() {
        let test_string = "467..114.....*........35..633.......#...617*...........+.58...592...........755....$.*.....664.598..";
        let array = parse_string_to_array(test_string);
        let multi_array = form_multidimensional_array(&array, (10, 10));
        println!("{}", multi_array);

        // [[4, 6, 7, ., ., 1, 1, 4, ., .],
        //  [., ., ., *, ., ., ., ., ., .],
        //  [., ., 3, 5, ., ., 6, 3, 3, .],
        //  [., ., ., ., ., ., #, ., ., .],
        //  [6, 1, 7, *, ., ., ., ., ., .],
        //  [., ., ., ., ., +, ., 5, 8, .],
        //  [., ., 5, 9, 2, ., ., ., ., .],
        //  [., ., ., ., ., ., 7, 5, 5, .],
        //  [., ., ., $, ., *, ., ., ., .],
        //  [., 6, 6, 4, ., 5, 9, 8, ., .]]

        let window_array = form_three_by_three_window_array(&multi_array);

        let test_chunks: &Chunks<ArrayView<String, Dim<[Ix; 2]>>> = &window_array.chunks(8);
        // dbg!(test_chunks);

        let boolean_mask = form_boolean_mask(&window_array);
        let chunked_window_array = chunk_window_array(&boolean_mask, 8);

        let coordinate_trios: HashSet<((usize, usize), (usize, usize), (usize, usize))> =
            extract_coordinates_for_all_non_dot_trios(&chunked_window_array);

        let filtered_trios: Vec<_> = coordinate_trios
            .iter()
            .filter(|trio| {
                let non_boolean_values =
                    extract_non_boolean_trio_values_from_original_multidimensional_array(
                        &multi_array,
                        &trio,
                    );

                let position_a_is_numeric = non_boolean_values.0.chars().any(|char| char.is_numeric());
                let position_b_is_star = non_boolean_values.1 == "*";
                let position_c_is_numeric = non_boolean_values.2.chars().any(|char| char.is_numeric());

                if position_a_is_numeric && position_b_is_star && position_c_is_numeric {
                    println!("Keeping {:?}", non_boolean_values)
                }
                return position_a_is_numeric && position_b_is_star && position_c_is_numeric
            })
            .collect();


        assert_eq!(filtered_trios.len(),2);




        // assert_eq!(coordinate_trios, expected);
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
            vec!["467","35"]
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
            .into_iter()
            .map(|element| element.parse::<i32>().unwrap())
            .sum();

        assert_eq!(established_values_adjacent_to_symbol, 4361);
    }

    #[test]
    fn test_extract_numbers_adjacent_to_symbols_based_on_part_2_test_case() {
        let giant_string = "467..114.....*........35..633.......#...617*...........+.58...592...........755....$.*.....664.598..";

        let array = parse_string_to_array(giant_string);
        let multi_array = form_multidimensional_array(&array, (10, 10));

        let skinny_window_array = form_three_by_three_window_array(&multi_array);
        let boolean_mask = form_boolean_mask(&skinny_window_array);
        let chunked_window_array = chunk_window_array(&boolean_mask, 8);

        let coordinate_trios: HashSet<((usize, usize), (usize, usize), (usize, usize))> =
            extract_coordinates_for_all_non_dot_trios(&chunked_window_array);

        let filtered_trios: Vec<_> = coordinate_trios
            .iter()
            .filter(|trio| {
                let non_boolean_values =
                    extract_non_boolean_trio_values_from_original_multidimensional_array(
                        &multi_array,
                        &trio,
                    );

                let position_a_is_numeric =
                    non_boolean_values.0.chars().any(|char| char.is_numeric());
                let position_b_is_star = non_boolean_values.1 == "*";
                let position_c_is_numeric =
                    non_boolean_values.2.chars().any(|char| char.is_numeric());

                if position_a_is_numeric && position_b_is_star && position_c_is_numeric {
                    println!("Keeping {:?}", non_boolean_values)
                }
                return position_a_is_numeric && position_b_is_star && position_c_is_numeric
            })
            .collect();

        let mut coordinates_and_value_map: HashMap<i32, i32> = HashMap::new();

        for filtered_trio in filtered_trios.iter() {
            let target_number: &&((usize, usize), (usize, usize), (usize, usize)) = filtered_trio;
            println!("{:?}", &target_number);
            let traced_number_a =
                trace_whole_number_from_array_following_coordinates(&multi_array, &target_number.0);
            let traced_number_c =
                trace_whole_number_from_array_following_coordinates(&multi_array, &target_number.2);

            let traced_coordinates_total: i32 = &traced_number_a.1.parse::<i32>().unwrap()
                + &traced_number_c.1.parse::<i32>().unwrap();
            let traced_values_product: i32 = &traced_number_a.0.parse::<i32>().unwrap()
                * &traced_number_c.0.parse::<i32>().unwrap();

            coordinates_and_value_map.entry(traced_coordinates_total).or_insert(traced_values_product);

        }

        let established_part_2_value_pairs_adjacent_to_star: i32 =
            coordinates_and_value_map.values().into_iter().sum();

        assert_eq!(established_part_2_value_pairs_adjacent_to_star, 467835)
    }

    #[test]
    fn test_combo_maker() {

        // Avoid dumb hard coding every combo
        let variables = [(0,0), (0,1), (0,2), (1,0), (1,1), (1,2), (2,0), (2,1), (2,2)];
        let mut combinations = Vec::new();

        // Check all combinations of three variables
        for i in 0..variables.len() {
            for j in i + 1..variables.len() {
                for k in j + 1..variables.len() {
                    combinations.push((variables[i], variables[j], variables[k]));
                }
            }
        }

        // Pull each combo from array
        // Print the generated combinations
        for (idx, combo) in combinations.iter().enumerate() {
            println!("Combination {}: {:?}", idx, combo);
        }

        println!("{:?}", combinations.get(5).unwrap());




    }

    #[test]
    fn test_part_2_test_case_with_star_solar_system_idea() {

        let giant_string = "467..114.....*........35..633.......#...617*...........+.58...592...........755....$.*.....664.598..";

        let array = parse_string_to_array(giant_string);
        let multi_array = form_multidimensional_array(&array, (10, 10));

        let window_array = form_window_array(&multi_array);
        let boolean_mask = form_boolean_mask(&window_array);
        let chunked_window_array = chunk_window_array(&boolean_mask, 9);

        let coordinate_pairings: HashSet<((usize, usize), (usize, usize))> =
            extract_coordinates_for_all_non_dot_pairings(&chunked_window_array);

        let filtered_pairing: Vec<_> = coordinate_pairings
            .iter()
            .filter(|pairing| {
                let non_boolean_values =
                    extract_non_boolean_pair_values_from_original_multidimensional_array(
                        &multi_array,
                        &pairing,
                    );

                let position_a_is_numeric = non_boolean_values.0.chars().any(|char| char.is_numeric())
                    && non_boolean_values.1.chars().any(|char| !char.is_numeric());

                let position_b_is_numeric = non_boolean_values.0.chars().any(|char| !char.is_numeric())
                    && non_boolean_values.1.chars().any(|char| char.is_numeric());

                let position_a_is_star = non_boolean_values.0 == "*";
                let position_b_is_star = non_boolean_values.1 == "*";

                if position_a_is_numeric && position_b_is_star {
                    true
                } else if position_b_is_numeric && position_a_is_star {
                    true
                } else {
                    false
                }
            })
            .collect();

        let mut coordinates_and_value_map: HashMap<((usize, usize)), Vec<String>> = HashMap::new();
        let mut numbers_already_traced: HashSet<String> = HashSet::new();

        for filtered_pair in filtered_pairing.iter() {
            let target_number: &&((usize, usize), (usize, usize)) = filtered_pair;
            // dbg!(&target_number);
            let non_boolean_pair = extract_non_boolean_pair_values_from_original_multidimensional_array(
                &multi_array,
                &target_number,
            );

            let position_a_is_numeric = non_boolean_pair.0.chars().any(|char| char.is_numeric())
                && non_boolean_pair.1.chars().any(|char| !char.is_numeric());

            let position_b_is_numeric = non_boolean_pair.0.chars().any(|char| !char.is_numeric())
                && non_boolean_pair.1.chars().any(|char| char.is_numeric());

            let position_a_is_star = non_boolean_pair.0 == "*";
            let position_b_is_star = non_boolean_pair.1 == "*";

            if position_a_is_numeric && position_b_is_star {
                let traced_number =
                    trace_whole_number_from_array_following_coordinates(&multi_array, &target_number.0);

                if !numbers_already_traced.contains(&traced_number.1) {
                    coordinates_and_value_map
                        .entry(target_number.1)
                        .or_insert(vec![]).push(traced_number.0);
                }

                numbers_already_traced.insert(traced_number.1);

            } else if position_b_is_numeric && position_a_is_star {
                let traced_number =
                    trace_whole_number_from_array_following_coordinates(&multi_array, &target_number.1);

                if !numbers_already_traced.contains(&traced_number.1) {
                    coordinates_and_value_map
                        .entry(target_number.0)
                        .or_insert(vec![]).push(traced_number.0);
                }

                numbers_already_traced.insert(traced_number.1);
            } else {
                ()
            }

        }

        let mut total_counter = 0;

        for store in coordinates_and_value_map.values() {

            if store.len() > 1 {
                let store_product: i32 = store.iter().map(|traced_number| traced_number.parse::<i32>().unwrap()).product();
                total_counter += store_product
            }

        }
        //
        // dbg!(coordinates_and_value_map);
        //
        // println!("{}", total_counter);
        assert_eq!(total_counter, 467835)
    }

    #[test]
    fn test_part_2_real_case_with_star_solar_system_idea() {

        let file_path: &str = "./src/input.txt";
        let content: String = std::fs::read_to_string(file_path).expect("should read from file");
        let content_as_line = content.lines().collect::<String>();

        let array = parse_string_to_array(&content_as_line);
        let multi_array = form_multidimensional_array(&array, (140, 140));

        let window_array = form_window_array(&multi_array);
        let boolean_mask = form_boolean_mask(&window_array);
        let chunked_window_array = chunk_window_array(&boolean_mask, 139);

        let coordinate_pairings: HashSet<((usize, usize), (usize, usize))> =
            extract_coordinates_for_all_non_dot_pairings(&chunked_window_array);

        let filtered_pairing: Vec<_> = coordinate_pairings
            .iter()
            .filter(|pairing| {
                let non_boolean_values =
                    extract_non_boolean_pair_values_from_original_multidimensional_array(
                        &multi_array,
                        &pairing,
                    );

                let position_a_is_numeric = non_boolean_values.0.chars().any(|char| char.is_numeric())
                    && non_boolean_values.1.chars().any(|char| !char.is_numeric());

                let position_b_is_numeric = non_boolean_values.0.chars().any(|char| !char.is_numeric())
                    && non_boolean_values.1.chars().any(|char| char.is_numeric());

                let position_a_is_star = non_boolean_values.0 == "*";
                let position_b_is_star = non_boolean_values.1 == "*";

                if position_a_is_numeric && position_b_is_star {
                    true
                } else if position_b_is_numeric && position_a_is_star {
                    true
                } else {
                    false
                }
            })
            .collect();

        let mut coordinates_and_value_map: HashMap<((usize, usize)), Vec<String>> = HashMap::new();
        let mut numbers_already_traced: HashSet<String> = HashSet::new();

        for filtered_pair in filtered_pairing.iter() {
            let target_number: &&((usize, usize), (usize, usize)) = filtered_pair;
            // dbg!(&target_number);
            let non_boolean_pair = extract_non_boolean_pair_values_from_original_multidimensional_array(
                &multi_array,
                &target_number,
            );

            let position_a_is_numeric = non_boolean_pair.0.chars().any(|char| char.is_numeric())
                && non_boolean_pair.1.chars().any(|char| !char.is_numeric());

            let position_b_is_numeric = non_boolean_pair.0.chars().any(|char| !char.is_numeric())
                && non_boolean_pair.1.chars().any(|char| char.is_numeric());

            let position_a_is_star = non_boolean_pair.0 == "*";
            let position_b_is_star = non_boolean_pair.1 == "*";

            if position_a_is_numeric && position_b_is_star {
                let traced_number =
                    trace_whole_number_from_array_following_coordinates(&multi_array, &target_number.0);

                if !numbers_already_traced.contains(&traced_number.1) {
                    coordinates_and_value_map
                        .entry(target_number.1)
                        .or_insert(vec![]).push(traced_number.0);
                }

                numbers_already_traced.insert(traced_number.1);

            } else if position_b_is_numeric && position_a_is_star {
                let traced_number =
                    trace_whole_number_from_array_following_coordinates(&multi_array, &target_number.1);

                if !numbers_already_traced.contains(&traced_number.1) {
                    coordinates_and_value_map
                        .entry(target_number.0)
                        .or_insert(vec![]).push(traced_number.0);
                }

                numbers_already_traced.insert(traced_number.1);
            } else {
                ()
            }

        }

        let mut total_counter = 0;

        for store in coordinates_and_value_map.values() {

            if store.len() > 1 {
                let store_product: i32 = store.iter().map(|traced_number| traced_number.parse::<i32>().unwrap()).product();
                total_counter += store_product
            }

        }
        //
        // dbg!(coordinates_and_value_map);
        //
        println!("{}", total_counter);
        assert_ne!(total_counter, 467835);
        assert_ne!(total_counter, 46371710);
        assert_ne!(total_counter, 55985365);
    }

    #[test]
    fn test_fake_3_by_3_grids() {
        let giant_string = "..467..231*245..158..";

        let array = parse_string_to_array(giant_string);
        let multi_array = form_multidimensional_array(&array, (3, 7));

        let skinny_window_array = form_three_by_three_window_array(&multi_array);
        let boolean_mask = form_boolean_mask(&skinny_window_array);
        let chunked_window_array = chunk_window_array(&boolean_mask, 5);

        let coordinate_trios: HashSet<((usize, usize), (usize, usize), (usize, usize))> =
            extract_coordinates_for_all_non_dot_trios(&chunked_window_array);

        let filtered_trios: Vec<_> = coordinate_trios
            .iter()
            .filter(|trio| {
                let non_boolean_values =
                    extract_non_boolean_trio_values_from_original_multidimensional_array(
                        &multi_array,
                        &trio,
                    );

                let position_a_is_numeric =
                    non_boolean_values.0.chars().any(|char| char.is_numeric());
                let position_b_is_star = non_boolean_values.1 == "*";
                let position_c_is_numeric =
                    non_boolean_values.2.chars().any(|char| char.is_numeric());

                if position_a_is_numeric && position_b_is_star && position_c_is_numeric {
                    println!("Keeping {:?}", non_boolean_values)
                }
                return position_a_is_numeric && position_b_is_star && position_c_is_numeric
            })
            .collect();

        let mut coordinates_and_value_map: HashMap<i32, i32> = HashMap::new();

        for filtered_trio in filtered_trios.iter() {
            let target_number: &&((usize, usize), (usize, usize), (usize, usize)) = filtered_trio;
            println!("{:?}", &target_number);
            let traced_number_a =
                trace_whole_number_from_array_following_coordinates(&multi_array, &target_number.0);
            let traced_number_c =
                trace_whole_number_from_array_following_coordinates(&multi_array, &target_number.2);
            //
            // let traced_coordinates_total: i32 = &traced_number_a.1.parse::<i32>().unwrap()
            //     + &traced_number_c.1.parse::<i32>().unwrap();
            // let traced_values_product: i32 = &traced_number_a.0.parse::<i32>().unwrap()
            //     * &traced_number_c.0.parse::<i32>().unwrap();

            coordinates_and_value_map.entry((&traced_number_a.1).parse().unwrap()).or_insert((&traced_number_a.0).parse().unwrap());
            coordinates_and_value_map.entry((&traced_number_c.1).parse().unwrap()).or_insert((&traced_number_c.0).parse().unwrap());

        }
        println!("{:?}", coordinates_and_value_map);



    }


}
