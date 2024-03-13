/// Bubble sort is a simple sorting algorithm that repeatedly steps through the list, compares adjacent elements and swaps them if they are in the wrong order.
///
/// Time complexity: O(n^2)
/// Space complexity: O(1)
pub fn sort<T: PartialOrd + Copy>(items: &mut [T]) -> &mut [T] {
    let n = items.len();
    for i in 0..n {
        for j in (i + 1..n).rev() {
            if items[j] < items[j - 1] {
                items.swap(j, j - 1);
            }
        }
    }
    items
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut numbers = vec![3, 2, 1];
        let n = sort(&mut numbers);
        assert_eq!(n, vec![1, 2, 3]);
        assert_eq!(n.as_mut_ptr(), numbers.as_mut_ptr());
    }

    #[test]
    fn test_bubble_sort_empty() {
        let mut numbers: Vec<i32> = vec![];
        sort(&mut numbers);
        assert_eq!(numbers, vec![]);
    }

    #[test]
    fn test_bubble_sort_single() {
        let mut numbers = vec![1];
        sort(&mut numbers);
        assert_eq!(numbers, vec![1]);
    }

    #[test]
    fn test_bubble_sort_duplicates() {
        let mut numbers = vec![3, 2, 1, 3, 2, 1];
        sort(&mut numbers);
        assert_eq!(numbers, vec![1, 1, 2, 2, 3, 3]);
    }

    #[test]
    fn test_bubble_sort_strings() {
        let mut numbers = vec!["cat", "bob", "a", "aa", ""];
        sort(&mut numbers);
        assert_eq!(numbers, vec!["", "a", "aa", "bob", "cat"]);
    }
}