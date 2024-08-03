use crate::Sorter;

pub struct InsertionSort{
    smart: bool,
}

impl Sorter for InsertionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord + std::fmt::Display,
    {
        // [ sorted | not sorted ]
        // let self.smart = false;
        for unsorted in 1..slice.len() {
            if !self.smart {

                // slice[unsorted..] is not sorted
                // take slice[unsorted] and place in sorted location in slice[..=unsorted]
                // [ 1 3 4 | 2 ]
                // [ 1 3 4 2 | ]
                // [ 1 3 2 4 | ]
                // [ 1 2 3 4 | ]
                let mut i = unsorted;
                while i > 0 && slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    i -= 1;
                }
            } else {
                // use binary search to find index
                // then use .insert to splice in i
                let i = match slice[..unsorted].binary_search(&slice[unsorted]) {
                    Ok(i) => i,
                    Err(i) => i,
                };
                slice[i..=unsorted].rotate_right(1);
            }
        }
    }
}

#[test]
fn insertion_alg_smart_test() {
    // let S = InsertionSort{ smart: true, };
    let mut items = vec![3,4,2,7,22,8,9,12,1];
   InsertionSort{ smart: true }.sort(&mut items);
    assert_eq!(items, &[1,2,3,4,7,8,9,12,22]);
}

#[test]
fn insertion_alg_not_smart_test() {
    let mut items = vec![3,4,2,7,22,8,9,12,1];
    InsertionSort{ smart: false }.sort(&mut items);
    assert_eq!(items, &[1,2,3,4,7,8,9,12,22]);
}