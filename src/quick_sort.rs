use crate::misc;

const N: usize = 10;

fn quick_sort(arr: &mut [i32; N], start_index: usize, end_index: usize) {
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

pub fn sort(arr: &mut [i32; N]) {
    println!("Using quick sort");

    let len = arr.len();
    quick_sort(arr, 0, len - 1);
    
    println!("{:?}", arr);
}
