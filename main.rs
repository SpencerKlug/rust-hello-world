fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main() {
    println!("Hello, world!");
    println!("{} days", 31);
    let logical: bool = true;
    println!("The value of logical is: {}", logical);
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    analyze_slice(&xs);
}