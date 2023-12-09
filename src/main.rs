use std::slice::Chunks;
use ndarray::{Array, ArrayBase, ArrayView, Dim, Ix, OwnedRepr};

fn main() {
    println!("Hello");
}

pub fn parse_string_to_array(input: &str) -> Vec<String> {
    input.chars().into_iter().map(|char| String::from(char)).collect()
}
pub fn form_multidimensional_array(input: &Vec<String>) -> Array<String, Dim<[Ix; 2]>> {
    Array::from_shape_vec((2,7), input.to_vec()).unwrap()
}

pub fn form_window_array(input: &ArrayBase<OwnedRepr<String>, Dim<[Ix; 2]>>) -> Vec<ArrayView<String, Dim<[Ix; 2]>>> {
    let windows: Vec<ArrayView<String, Dim<[Ix; 2]>>> = input.windows(Dim((2, 2))).into_iter().collect();
    windows
}

pub fn form_boolean_mask(input: &Vec<ArrayView<String, Dim<[Ix; 2]>>>) -> Vec<Array<bool, Dim<[Ix; 2]>>> {
    input.iter().map(|window| window.map(|element| !(element.to_owned() == "."))).collect()
}

pub fn chunk_window_array(input: &Vec<Array<bool, Dim<[Ix; 2]>>>, chunk_size: usize) -> Chunks<Array<bool, Dim<[Ix; 2]>>> {

    input.chunks(chunk_size)

}


#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::{Array2};

    #[test]
    fn test_parse_string_to_array() {
        let giant_string = "467..114.....*";
        let array = parse_string_to_array(giant_string);
        let expected_array = vec!["4","6","7",".",".","1","1","4",".",".",".",".",".","*"];
        assert_eq!(array, expected_array);
    }

    #[test]
    fn test_multidimensional_array() {
        let giant_string = "467..114.....*";
        let array = parse_string_to_array(giant_string);
        let expected_array = vec!["4","6","7",".",".","1","1","4",".",".",".",".",".","*"];

        let multi_array = form_multidimensional_array(&array);
        let expected = Array2::from_shape_vec((7,2),expected_array).unwrap();
        assert_eq!(multi_array, expected);
    }

    #[test]
    fn test_window_array() {
        let giant_string = "467..114.....*";
        let array = parse_string_to_array(giant_string);
        let multi_array = form_multidimensional_array(&array);
        let window_array = form_window_array(&multi_array);

        let expected = Array2::from_shape_vec((2,2),vec!["4","6","7","."]).unwrap();
        assert_eq!(expected, window_array.get(0).unwrap())
    }

    #[test]
    fn test_form_boolean_mask() {
        let giant_string = "467..114.....*";
        let array = parse_string_to_array(giant_string);
        let multi_array = form_multidimensional_array(&array);
        let window_array = form_window_array(&multi_array);

        let boolean_mask = form_boolean_mask(&window_array);
        let expected = Array2::from_shape_vec((2,2),vec![true,true,true,false]).unwrap();

        assert_eq!(boolean_mask.get(0).unwrap(), expected);

    }

    #[test]
    fn test_form_chunk_boolean_window_array() {
        let giant_string = "467..114.....*";
        let array = parse_string_to_array(giant_string);
        let multi_array = form_multidimensional_array(&array);

        println!("{}", &multi_array);

        let window_array = form_window_array(&multi_array);
        let boolean_mask = form_boolean_mask(&window_array);
        let chunked_window_array = chunk_window_array(&boolean_mask, 6);

        // Given a max row of 7, you will never iterate on to just the last column
        // You will stop short in order to satisfy the 2 x 2 window
        assert_eq!(chunked_window_array.len(), 1);
    }

}
