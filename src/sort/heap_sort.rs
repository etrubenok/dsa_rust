/// Heapsort algorithm implementation
///
/// Time complexity: O(n log n)
/// Space complexity: O(1)
pub fn sort<T: PartialOrd + Copy>(items: &mut [T]) -> &mut [T] {
    build_max_heap(items);
    // The max-heap property is maintained for the entire array so the largest element is at the root (index 0)
    // We swap the root with the last element in the array and call max_heapify on the root to maintain the max-heap property
    for i in (1..items.len()).rev() {
        items.swap(0, i);
        // The last `i` elements are already sorted so we exclude them from the max_heapify call to avoid moving them back to the root
        max_heapify(&mut items[0..i], 0);
    }
    items
}

/// Builds a max-heap from the given items
/// It iterates through the items from the middle to the beginning and calls max_heapify on each item going up the tree
/// This ensures that the max-heap property is maintained for each subtree
/// Second half of the items are leaves and are already max-heaps
///
/// Time complexity: O(n log n)
/// Space complexity: O(1)
fn build_max_heap<T: PartialOrd + Copy>(items: &mut [T]) {
    for i in (0..items.len() / 2).rev() {
        max_heapify(items, i);
    }
}

/// Maintains the max-heap property for a given index `i` in the heap
///
/// Time complexity: O(log n)
/// Space complexity: O(1)
fn max_heapify<T: PartialOrd + Copy>(items: &mut [T], i: usize) {
    let l = left(i);
    let r = right(i);

    let mut largest = i;
    if l < items.len() && items[l] > items[i] { // If the left child is greater than the current node
        largest = l;
    }
    if r < items.len() && items[r] > items[largest] { // If the right child is greater than the largest so far
        largest = r;
    }
    // If the largest element is not the current node, swap the current node with the largest and recursively call max_heapify on the largest node
    if largest != i {
        items.swap(i, largest);
        max_heapify(items, largest);
    }
}

/// Returns the index of the parent of the node at index `i` in the heap
fn _parent(i: usize) -> usize {
    (i - 1) / 2
}

/// Returns the index of the left child of the node at index `i` in the heap
fn left(i: usize) -> usize {
    2 * i + 1
}

/// Returns the index of the right child of the node at index `i` in the heap
fn right(i: usize) -> usize {
    2 * i + 2
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_heap_sort() {
        let mut numbers = vec![3, 2, 1];
        let n = super::sort(&mut numbers);
        assert_eq!(n, vec![1, 2, 3]);
        assert_eq!(n.as_mut_ptr(), numbers.as_mut_ptr());
    }
}