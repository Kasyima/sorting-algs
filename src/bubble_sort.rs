use super::{Sorter};

pub struct  BubbleSort;

impl Sorter for BubbleSort{
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord + std::fmt::Display,
    {
        let mut swapped = true;

        while swapped {
            swapped = false;
            for i in 0..(slice.len() - 1) {

                /*if i == slice.len() - 1 {
                    continue
                }*/
                if slice[i] > slice[i + 1] {
                    slice.swap(i, i+1);
                    swapped = true;
                }
            }

        }


    }
}

#[test]
fn bubble_sort_works() {
    let mut things = vec![4, 2, 3, 1];
    BubbleSort.sort(&mut things);
    assert_eq!(things, &[1,2,3,4]);
}