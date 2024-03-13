/// Merge sort is an example of the divide and conquer strategy. It divides the input array into two halves, calls itself for the two halves, and then merges the two sorted halves.
/// Time complexity: O(n log n)
/// Space complexity: O(n)
pub fn sort<T: PartialOrd + Copy>(items: &mut [T]) -> &mut [T] {
    merge_sort(items, 0, items.len() - 1);
    items
}

/// Sorts the slice `items` (starting from `l` to `r`) using the merge sort algorithm
fn merge_sort<T: PartialOrd + Copy>(items: &mut [T], l: usize, r: usize) {
    if l < r {
        let m = (l + r) / 2; // get the middle index in the slice
        merge_sort(items, l, m); // recursively sort the left half
        merge_sort(items, m + 1, r); // recursively sort the right half
        merge(items, l, m, r); // merge the sorted halves
    }
}

fn merge<T: PartialOrd + Copy>(items: &mut [T], l: usize, m: usize, r: usize) {
    let n_left = m - l + 1;
    let n_right = r - m;

    // Copy data to temp arrays left[] and right[]
    let left = items[l..=m].to_vec();
    let right = items[m + 1..=r].to_vec();

    assert_eq!(left.len(), n_left);
    assert_eq!(right.len(), n_right);

    let mut i = 0;
    let mut j = 0;
    let mut k = l; // `k` is the index in the original array `items`. We will write items from `left` and `right` into `items` starting at index `l`

    while i < n_left && j < n_right {
        if left[i] <= right[j] {
            items[k] = left[i]; // take the smaller element from left
            i += 1;
        } else {
            items[k] = right[j]; // take the smaller element from right
            j += 1;
        }
        k += 1;
    }
    // At this point one of the arrays `left` or `right` has been exhausted but there may still be elements in the other array. We need to copy these elements to `items`

    // If there are any remaining elements in `left`, copy them to `items`
    while i < n_left {
        items[k] = left[i];
        i += 1;
        k += 1;
    }

    // If there are any remaining elements in `right`, copy them to `items`
    while j < n_right {
        items[k] = right[j];
        j += 1;
        k += 1;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_merge_sort() {
        let mut numbers = vec![3, 2, 1, 100, 101, 0, -1];
        let n = super::sort(&mut numbers);
        assert_eq!(n, vec![-1, 0, 1, 2, 3, 100, 101]);
        // Check that the input and output vectors are the same - i.e. the function is in-place
        assert_eq!(n.as_mut_ptr(), numbers.as_mut_ptr());
    }
}