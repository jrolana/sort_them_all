const N: usize = 10;

fn merge(left_arr: &[i32], rigth_arr: &[i32], arr: &mut [i32]) {
    let left_arr_size: usize = left_arr.len();
    let rigth_arr_size: usize = rigth_arr.len();

    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut k: usize = 0;

    while i < left_arr_size && j < rigth_arr_size {
        if left_arr[i] > rigth_arr[j] {
            arr[k] = rigth_arr[j];
            j += 1;
        } else {
            arr[k] = left_arr[i];
            i += 1;
        }
        k += 1;
    }

    while i < left_arr_size {
        arr[k] = left_arr[i];
        i += 1;
        k += 1;
    }

    while j < rigth_arr_size {
        arr[k] = rigth_arr[j];
        j += 1;
        k += 1;
    }
}

fn merge_sort(arr: &mut [i32]) {
    let arr_len = arr.len();

    if arr_len < 2 {
        return;
    }

    let mid: usize = arr_len / 2;

    let mut left_arr: Vec<i32> = Vec::new();

    for k in 0..mid {
        left_arr.push(arr[k]);
    }

    let mut right_arr: Vec<i32> = Vec::new();

    for k in mid..arr_len {
        right_arr.push(arr[k]);
    }

    merge_sort(&mut left_arr);
    merge_sort(&mut right_arr);

    merge(&mut left_arr, &mut right_arr, arr);
}

pub fn sort(arr: &mut [i32; N]) {
    println!("Using merge sort");

    merge_sort(arr);

    println!("{:?}", arr);
}
