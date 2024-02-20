use crate::misc;

const N: usize = 10;

pub fn sort(arr: &mut [i32; N]) {
    println!("Using bubble sort");

    let mut num_swap: i32;

    for _ in 1..N {
        num_swap = 0;

        for x in 1..N {
            if arr[x] < arr[x - 1] {
                misc::swap(arr, x, x - 1);
                num_swap = 1;
            }
        }

        if num_swap == 0 {
            break;
        }
    }
    println!("{:?}", arr);
}
