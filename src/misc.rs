use rand::{self, Rng};

pub fn swap(arr: &mut [i32; 10], i: usize, j: usize) {
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

pub fn generate_random_array() -> [i32; 10] {
    let mut arr: [i32; 10] = [0; 10];

    for i in 0..10 {
        arr[i] = rand::thread_rng().gen_range(-100..100);
    }

    arr
}
