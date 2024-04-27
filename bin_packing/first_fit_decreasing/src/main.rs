use first_fit_decreasing::first_fit_decreasing;
fn main() {
    let nums: [u32; 11] = [8, 7, 14, 9, 6, 9, 5, 15, 6, 7, 8];
    let res = first_fit_decreasing(20, &nums);
    println!("{:?}", res);
}
