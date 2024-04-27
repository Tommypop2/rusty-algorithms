use first_fit::first_fit;
fn main() {
    let nums: [u32; 11] = [8, 7, 14, 9, 6, 9, 5, 15, 6, 7, 8];
    let res = first_fit(20, &nums);
    println!("{:?}", res);
}
