/// Sort the given numbers using the insert sort algorithm
///
/// Sorting is done in place, so the given vector will be modified and the function will return nothing.
///
/// Time complexity: O(n^2)
/// Space complexity: O(1)
pub fn sort<T: PartialOrd + Copy>(numbers: &mut [T]) -> &mut [T] {
    // Start at the second element
    for i in 1..numbers.len() {
        // The current element `n` to be inserted into the sorted portion of the vector numbers[0:i]
        let n = numbers[i];
        let mut j = i;
        // Iterate backwards through the sorted portion of the vector
        while j > 0 && numbers[j - 1] > n {
            // Shift the current element to the right if it is greater than `n`
            numbers[j] = numbers[j - 1];
            j -= 1;
        }
        // Insert the current element in the correct position. In the extreme case where `n` is the smallest element, `j` will be 0
        numbers[j] = n;
    }
    numbers
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_sort() {
        let mut numbers = vec![3, 2, 1];
        let n = sort(&mut numbers);
        assert_eq!(n, vec![1, 2, 3]);
        assert_eq!(n.as_mut_ptr(), numbers.as_mut_ptr());
    }

    #[test]
    fn test_insert_sort_empty() {
        let mut numbers: Vec<i32> = vec![];
        sort(&mut numbers);
        assert_eq!(numbers, vec![]);
    }

    #[test]
    fn test_insert_sort_single() {
        let mut numbers = vec![1];
        sort(&mut numbers);
        assert_eq!(numbers, vec![1]);
    }

    #[test]
    fn test_insert_sort_duplicates() {
        let mut numbers = vec![3, 2, 1, 3, 2, 1];
        sort(&mut numbers);
        assert_eq!(numbers, vec![1, 1, 2, 2, 3, 3]);
    }

    #[test]
    fn test_insert_sort_strings() {
        let mut numbers = vec!["cat", "bob", "a", "aa", ""];
        sort(&mut numbers);
        assert_eq!(numbers, vec!["", "a", "aa", "bob", "cat"]);
    }

    #[test]
    fn test_insert_sort_chars() {
        let mut numbers = vec!['c', 'b', 'a'];
        sort(&mut numbers);
        assert_eq!(numbers, vec!['a', 'b', 'c']);
    }

    #[test]
    fn test_insert_sort_floats() {
        let mut numbers = vec![3.0, 2.0, 1.0];
        sort(&mut numbers);
        assert_eq!(numbers, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_insert_sort_structs() {
        #[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
        struct TestStruct {
            a: i32,
            b: i32,
        }
        let mut numbers = vec![
            TestStruct { a: 3, b: 3 },
            TestStruct { a: 2, b: 2 },
            TestStruct { a: 1, b: 1 },
        ];
        sort(&mut numbers);
        assert_eq!(
            numbers,
            vec![
                TestStruct { a: 1, b: 1 },
                TestStruct { a: 2, b: 2 },
                TestStruct { a: 3, b: 3 },
            ]
        );
    }
}