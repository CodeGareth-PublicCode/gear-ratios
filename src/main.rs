use ndarray::iter::Windows;
use ndarray::{array, Array, ArrayBase, ArrayView, Dim, Ix, OwnedRepr};
use std::slice::Chunks;

fn main() {
    println!("Hello");
}

pub fn parse_string_to_array<'a>(input: &str) -> Vec<&str> {
    let array = input.split("").filter(|&s| s != "").collect();
    array
}

pub fn form_multidimensional_array(
    array_dimensions: Dim<[usize; 2]>,
    raw_input: Vec<&str>,
) -> ArrayBase<OwnedRepr<&str>, Dim<[usize; 2]>> {
    Array::from_shape_vec(array_dimensions, raw_input).unwrap()
}

pub fn form_two_by_two_window_views_of_multidimensional_array<'b>(
    raw_input: &[&'b str], array_dimensions: Dim<[usize; 2]>,

) -> Windows<'b, &'b str, Dim<[usize; 2]>> {

    Array::from_shape_vec(array_dimensions, raw_input.to_vec()).unwrap().windows(Dim((2, 2)))

}

pub fn form_boolean_mask_based_on_dot_or_not<'a>(
    input_array: Vec<ArrayView<'a, &'a str, Dim<[Ix; 2]>>>,
) -> Vec<Array<bool, Dim<[Ix; 2]>>> {
    input_array
        .iter()
        .map(|window| window.map(|element| !(element.to_owned() == ".")))
        .collect()
}

pub fn chunk_boolean_arrays_to_reflect_rows(
    boolean_arrays: &Vec<Array<bool, Dim<[Ix; 2]>>>,
    chunk_size: usize,
) -> Chunks<Array<bool, Dim<[Ix; 2]>>> {
    let iter = boolean_arrays.chunks(chunk_size);
    iter
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::assert_equal;
    use ndarray::{Array2, Ix2};

    #[test]
    fn test_parse_string_to_array() {
        let giant_string = "467..114.....*";
        let array = parse_string_to_array(giant_string);
        assert_eq!(
            array.get(13).unwrap(),
            &"*"
        );
    }

    #[test]
    fn test_form_multidimensional_array() {
        let giant_string = "467..114.....*";
        let array = parse_string_to_array(giant_string);
        let multidimensional_array = form_multidimensional_array(Dim((2, 7)), array);

        let expected_array = Array2::from_shape_vec(Dim((2,2)), vec![
            "4", "6", "7", ".", ".", "1", "1",
            "4", ".", ".", ".", ".", ".", "*"
        ]).unwrap();

        assert_eq!(multidimensional_array,expected_array);
    }

    #[test]
    fn test_form_window_array() {
        let giant_string = "467..114.....*";
        let array = parse_string_to_array(giant_string);
        let multidimensional_array = form_multidimensional_array(Dim((2, 7)), array);

        let array_for_boolean = parse_string_to_array(giant_string);
        let windows =
            form_two_by_two_window_views_of_multidimensional_array(&array_for_boolean, Dim((2, 7)));
    }
}
