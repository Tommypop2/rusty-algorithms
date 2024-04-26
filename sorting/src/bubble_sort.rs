use crate::shared::Sortable;

pub fn bubble_sort<T: Sortable>(slice: &mut [T]) {
    let mut i = 0;
    loop {
        let mut swapped = false;
        i += 1;
        for i in 0..(slice.len() - i) {
            if slice[i] > slice[i + 1] {
                slice.swap(i, i + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_bubble_sort() {
        let mut vec = vec![5, 4, 3, 2, 1];
        super::bubble_sort(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 5]);
    }
}
