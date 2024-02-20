mod bubble_sort;
mod heap_sort;
mod insertion_sort;
mod merge_sort;
mod misc;
mod selection_sort;
mod quick_sort;

const N: usize = 10;

fn main() {
    let arr: [i32; N] = misc::generate_random_array();
    println!("Original: {:?}\n", arr);

    heap_sort::sort(&mut arr.clone());
    bubble_sort::sort(&mut arr.clone());
    merge_sort::sort(&mut arr.clone());
    quick_sort::sort(&mut arr.clone());
    insertion_sort::sort(&mut arr.clone());
    selection_sort::sort(&mut arr.clone());
}
