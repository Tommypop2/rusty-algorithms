pub trait Sortable = PartialEq + Sized + PartialOrd;
pub enum SortOrder {
    ASCENDING,
    DESCENDING,
}
pub fn is_sorted<T: Sortable>(slice: &[T], order: SortOrder) -> bool {
    for i in 0..(slice.len() - 1) {
        let current = &slice[i];
        let next = &slice[i + 1];
        match order {
            SortOrder::ASCENDING => {
                if current > next {
                    return false;
                }
            }
            SortOrder::DESCENDING => {
                if current < next {
                    return false;
                }
            }
        };
    }
    true
}
#[cfg(test)]
mod test {
    use super::is_sorted;
    use super::SortOrder;

    #[test]
    fn ascending() {
        let ascending = vec![1, 2, 3, 4, 5];
        assert_eq!(is_sorted(&ascending, SortOrder::ASCENDING), true);
        assert_eq!(is_sorted(&ascending, SortOrder::DESCENDING), false);
    }
    #[test]
    fn descending() {
        let descending = vec![5, 4, 3, 2, 1];
        assert_eq!(is_sorted(&descending, SortOrder::ASCENDING), false);
        assert_eq!(is_sorted(&descending, SortOrder::DESCENDING), true);
    }
    #[test]
    fn no_order() {
        let unordered = vec![5,4,7,2,6];
        assert_eq!(is_sorted(&unordered, SortOrder::ASCENDING), false);
        assert_eq!(is_sorted(&unordered, SortOrder::DESCENDING), false);
    }
}
