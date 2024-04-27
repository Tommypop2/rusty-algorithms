use first_fit::first_fit;
use quick_sort::quick_sort;
pub fn first_fit_decreasing(capacity: u32, items: &[u32]) -> Vec<Vec<u32>> {
    // Sort array
    let mut cloned = items.to_vec();
    quick_sort::quick_sort(&mut cloned);
    cloned.reverse();
    first_fit(capacity, &cloned)
}
