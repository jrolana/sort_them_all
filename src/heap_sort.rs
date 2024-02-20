use crate::misc;
use crate::misc::heap;

const N: usize = 10;

fn bubble_up(heap: &mut [i32; N], index: usize) {
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

fn heapify(arr: &mut [i32; N]) -> [i32; N] {
    let mut heap: [i32; N] = [0; N];

    for i in 0..N {
        heap[i] = arr[i];
        bubble_up(&mut heap, i);
    }

    heap
}

fn bubble_down(heap: &mut [i32; N], heap_size: usize, index: usize) {
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

fn heap_sort(heap: &mut [i32; N]) {
    for i in 0..N {
        misc::swap(heap, 0, heap.len() - 1 - i);
        bubble_down(heap, heap.len() - 1 - i, 0);
    }
}

pub fn sort(arr: &mut [i32; N]) {
    println!("Using heap sort");

    let mut heap = heapify(arr);

    heap_sort(&mut heap);

    println!("{:?}", heap);
}
