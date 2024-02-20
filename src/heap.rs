use crate::misc;
use crate::misc::heap;

fn bubble_up(heap: &mut [i32; 10], index: usize) {
    let parent_index = heap::get_parent(index);

    if index == 0 {
        return;
    }

    let parent = heap[parent_index];
    let element = heap[index];

    if parent < element {
        misc::swap(heap, parent_index, index);
        bubble_up(heap, parent_index);
    }
}

fn heapify(arr: [i32; 10]) -> [i32; 10] {
    let mut heap: [i32; 10] = [0; 10];

    for i in 0..10 {
        heap[i] = arr[i];
        bubble_up(&mut heap, i);
    }

    heap
}

fn bubble_down(heap: &mut [i32; 10], heap_size: usize, index: usize) {
    let left_index = heap::get_left(index);
    let right_index = heap::get_right(index);

    if left_index >= heap_size && right_index >= heap_size {
        return;
    }

    let mut child_index = 0;
    let mut child = heap[child_index];

    if left_index < heap_size {
        child_index = left_index;
        child = heap[left_index];
    }

    if right_index < heap_size {
        if heap[right_index] > child {
            child_index = right_index;
            child = heap[right_index];
        }
    }

    if child > heap[index] {
        misc::swap(heap, child_index, index);
        bubble_down(heap, heap_size, child_index);
    }
}

fn heap_sort(heap: &mut [i32; 10]) {
    for i in 0..10 {
        misc::swap(heap, 0, heap.len() - 1 - i);
        bubble_down(heap, heap.len() - 1 - i, 0);
    }
}

pub fn run() {
    let arr: [i32; 10] = misc::generate_random_array();
    let mut heap = heapify(arr);
    println!("{:?}", heap);

    heap_sort(&mut heap);

    println!("{:?}", heap);
}
