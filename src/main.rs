mod sort;

fn main() {
    println!("Insert sort: {:?}", sort::insert_sort::sort(&mut vec![3, 2, 1]));
    println!("Merge sort: {:?}", sort::merge_sort::sort(&mut vec![3, 2, 1]));
    println!("Bubble sort: {:?}", sort::bubble_sort::sort(&mut vec![3, 2, 1]));
    println!("Heap sort: {:?}", sort::heap_sort::sort(&mut vec![3, 2, 1]));
}
