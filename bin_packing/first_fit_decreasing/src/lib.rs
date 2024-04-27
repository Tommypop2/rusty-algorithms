use first_fit::first_fit;
use quick_sort::quick_sort;
/**
 * Sorts the slice in place before applying the first fit algorithm
 */
pub fn first_fit_decreasing(capacity: u32, items: &mut [u32]) -> Vec<Vec<u32>> {
    // Sort array
    quick_sort::quick_sort(items);
    items.reverse();
    first_fit(capacity, items)
}
