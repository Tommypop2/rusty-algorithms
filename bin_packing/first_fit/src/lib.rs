pub fn first_fit(capacity: u32, items: &[u32]) -> Vec<Vec<u32>> {
    let mut bins: Vec<Vec<u32>> = Vec::new();
    for i in items {
        // Check if item can fit in each bin
        let mut fitted = false;
        for b in &mut bins {
            let used_space = b.iter().sum::<u32>();
            dbg!(used_space);
            if *i + used_space <= capacity {
                // There is enough space for i to fit into the bin
                b.push(i.clone());
                fitted = true;
                break;
            }
        }
        if !fitted {
            // Item cannot fit into bin, so must make new bin for item
            bins.push(vec![i.clone()])
        }
    }
    bins
}
#[cfg(test)]
mod test {
    #[test]
    fn test() {}
}
