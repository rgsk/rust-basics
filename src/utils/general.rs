pub fn print_array(arr: &[i32]) {
  for i in 0..arr.len() {
    print!("{} ", arr[i]);
  }
  println!();
}
pub fn swap_array_indexes(arr: &mut [i32], i: usize, j: usize) {
  let temp = arr[j];
  arr[j] = arr[i];
  arr[i] = temp;
}
