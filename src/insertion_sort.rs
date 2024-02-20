use crate::misc;

const N: usize = 10;

pub fn sort(arr: &mut [i32; N]) {
    println!("Using insertion sort");

    let mut current_index: usize;

    for n in 0..N {
        current_index = n;

        for m in (0..n).rev() {
            if arr[m] < arr[current_index] {
                break;
            }

            misc::swap(arr, m, current_index);
            current_index -= 1;
        }
    }

    println!("{:?}", arr);
}
