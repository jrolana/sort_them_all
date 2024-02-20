use rand::{self, Rng};

const N: usize = 10;

pub fn swap(arr: &mut [i32; N], i: usize, j: usize) {
    let temp: i32 = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
}

pub mod heap {
    pub fn get_parent(index: usize) -> usize {
        index / 2
    }

    pub fn get_left(index: usize) -> usize {
        index * 2
    }

    pub fn get_right(index: usize) -> usize {
        (index * 2) + 1
    }
}

pub fn generate_random_array() -> [i32; N] {
    let mut arr: [i32; N] = [0; N];

    for i in 0..N {
        arr[i] = rand::thread_rng().gen_range(-100..100);
    }

    arr
}
