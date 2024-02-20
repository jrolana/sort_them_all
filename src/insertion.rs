use crate::misc;

pub fn run() {
    let mut arr: [i32; 10] = misc::generate_random_array();
    let mut current_index: usize;

    for n in 0..10 {
        current_index = n;

        for m in (0..n).rev() {
            if arr[m] < arr[current_index] {
                break;
            }

            misc::swap(&mut arr, m, current_index);
            current_index -= 1;
        }
    }

    println!("{:?}", arr);
}
