use crate::misc;

const N: usize = 10;

pub fn sort(arr: &mut [i32; N]) {
    println!("Using selection sort");

    let mut len_sorted_arr: usize = 0;
    let mut min_index: usize;

    for _ in 0..N {
        min_index = len_sorted_arr;

        for x in len_sorted_arr..N {
            if arr[x] < arr[min_index] {
                min_index = x;
            }
        }

        misc::swap(arr, min_index, len_sorted_arr);

        len_sorted_arr += 1;
    }

    println!("{:?}", arr);
}
