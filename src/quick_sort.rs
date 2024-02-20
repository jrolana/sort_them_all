use crate::misc;

fn quick_sort(arr: &mut [i32; 10], start_index: usize, end_index: usize) {
    let mut i = start_index;
    let mut j = start_index;

    let pivot = arr[end_index];

    while j < end_index {
        if arr[j] <= pivot {
            misc::swap(arr, i, j);
            i += 1;
        }

        j += 1;
    }

    misc::swap(arr, i, j);

    if i > start_index + 1 {
        quick_sort(arr, start_index, i - 1);
    }

    if i < end_index - 1 {
        quick_sort(arr, i + 1, end_index);
    }
}

pub fn run() {
    let mut arr: [i32; 10] = misc::generate_random_array();
    let len = arr.len();
    quick_sort(&mut arr, 0, len - 1);
    println!("{:?}", arr);
}
