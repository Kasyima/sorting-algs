use super::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord + std::fmt::Display,
    {
        for unsorted in 0..slice.len() {
            let mut smallest_in_rest = unsorted;
            for i in unsorted..slice.len(){

                    if slice[i] <= slice[smallest_in_rest] {
                        smallest_in_rest = i;
                    }


            }

            let  smallest_in_rest = slice[unsorted..]
                .iter()
                .enumerate()
                .min_by_key(|&(i, v)| v)
                .map( |(i, _)| {unsorted + i })
                .expect("slice is non-empty");


            slice.swap(unsorted, smallest_in_rest);
            // let i = slice[..unsorted].binary_search(&slice[smallest_in_rest]).unwrap_or_else(|i| i);
            // slice[i..=smallest_in_rest].rotate_right(1);
        }
    }
}

#[test]
fn selection_sort_test() {
    let mut items = vec![3,4,2,7,22,8,9,12,1] ;
    // vec![3,4,2,7,22,8,9,12,1];
    SelectionSort.sort(&mut items);
    assert_eq!(items, &[1,2,3,4,7,8,9,12,22]);
    // &[1,2,3,4,7,8,9,12,22]
}