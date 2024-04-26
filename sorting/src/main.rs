use std::vec;

use quick_sort::quick_sort::quick_sort;

fn main() {
    let mut vec = vec![5, 4, 3, 2, 1];
    println!("Unsorted: {:?}", vec);
    quick_sort(&mut vec);
    println!("Sorted: {:?}", vec);
}
