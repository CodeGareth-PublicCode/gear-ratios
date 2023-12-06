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
    use itertools::{interleave, izip, peek_nth, Itertools};
    use ndarray::iter::Windows;
    use ndarray::ArrayView;
    use ndarray::Dim;
    use ndarray::{Array, Ix};
    use std::collections::HashMap;

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

        for (anchor_row, chunk) in not_a_dot_mask.chunks(chunk_length).enumerate() {
            window_store.entry(anchor_row).or_insert(chunk);
        }

        let anchor_row_zero = window_store.get(&0).unwrap();
        let target_window = anchor_row_zero.get(2).unwrap();

        let a1: bool = target_window.get((0, 0)).unwrap().to_owned();
        let a2: bool = target_window.get((0, 1)).unwrap().to_owned();
        let b1: bool = target_window.get((1, 0)).unwrap().to_owned();
        let b2: bool = target_window.get((1, 1)).unwrap().to_owned();

        let combinations: [(bool, bool); 6] =
            [(a1, a2), (b1, b2), (a1, b1), (a2, b2), (b1, a2), (a1, b2)];

        for (position, &combo) in combinations.iter().enumerate() {
            dbg!(position);

            if combo == (true, true) {
                let window_coordinates_for_known_symbol_and_adjacent_number =
                    (map_position_to_coordinate(position));
                dbg!(target_window);
                assert_eq!(
                    ([0, 0], [1, 1]),
                    window_coordinates_for_known_symbol_and_adjacent_number
                )
            }
        }

        // to do - is it possible to then establish your row number then relative position to main grid?
    }

    fn map_position_to_coordinate(position: usize) -> ([i32; 2], [i32; 2]) {
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
