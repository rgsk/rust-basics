pub fn first() {
  let mut iter = 0..3;
  println!("{:?}", iter.next());
  println!("{:?}", iter.next());
  println!("{:?}", iter.next());
  println!("{:?}", iter.next());
}
pub fn second() {
  let mut iter = 0..3;

  println!("{}", iter.next().unwrap_or(-1));
  println!("{}", iter.next().unwrap_or(-1));
  println!("{}", iter.next().unwrap_or(-1));
  println!("{}", iter.next().unwrap_or(-1));
}

/**
 * In fact, it is more efficient to iterate over an array or
 * slice this way than to use for i in 0..slice.len() {}
 * because Rust does not have to obsessively check every index operation.
 */
pub fn array_slice_iterators() {
  let arr = [10, 20, 30];
  for i in arr.iter() {
    print!("{} ", i);
  }
  println!();
  // slices will be converted implicitly to iterators...
  let slice = &arr;
  for i in slice {
    print!("{} ", i);
  }
  println!();
}

pub fn pro_sum_calc() {
  // sum1.rs
  let sum: i32 = (0..5).sum();
  println!("sum was {}", sum);

  let sum: i64 = [10, 20, 30].iter().sum();
  println!("sum was {}", sum);
}

pub fn windows_ex() {
  let ints = [1, 2, 3, 4, 5, 6, 7];
  let slice = &ints;

  println!("slice.windows(2)");
  for s in slice.windows(2) {
    println!("window {:?}", s);
  }
  println!("slice.windows(3)");
  for s in slice.windows(3) {
    println!("window {:?}", s);
  }
}
pub fn chunks_ex() {
  let ints = [1, 2, 3, 4, 5, 6, 7];
  let slice = &ints;

  println!("slice.chunks(2)");
  for s in slice.chunks(2) {
    println!("window {:?}", s);
  }
  println!("slice.chunks(3)");
  for s in slice.chunks(3) {
    println!("window {:?}", s);
  }
}
