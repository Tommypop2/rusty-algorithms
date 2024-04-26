use crate::shared::Sortable;

/**
 * Partitions the given slice in place and returns the index of the pivot
 */
fn partition_slice<T: Sortable>(slice: &mut [T], len: usize) -> usize {
    // Partition the slice
    let pivot_ind = len - 1;
    let mut smaller = 0usize;
    for i in 0..(len - 1) {
        if slice[i] < slice[pivot_ind] {
            slice.swap(i, smaller);
            smaller += 1;
        }
    }
    slice.swap(smaller, pivot_ind);
    smaller
}
/**
 * Sorts the given vector in place
 */
pub fn quick_sort<T: Sortable>(slice: &mut [T]) {
    let len = slice.len();
    if len <= 1 {
        return;
    }
    // Partition the slice
    let pivot = partition_slice(slice, len);

    quick_sort(&mut slice[0..pivot]);
    quick_sort(&mut slice[(pivot + 1)..len]);
}

#[cfg(test)]
mod test {
    #[test]
    fn test_quick_sort() {
        let mut vec = vec![5, 4, 3, 2, 1];
        super::quick_sort(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 5]);
    }
}
