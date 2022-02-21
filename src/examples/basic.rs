pub fn swap_ex() {
  let mut a = 1;
  let mut b = 2;
  println!("a {}, b {}", a, b);
  std::mem::swap(&mut a, &mut b);
  println!("a {}, b {}", a, b);
}
pub fn mutation_ref_ex() {
  let mut i = 32;
  add1(&mut i);
  add1(&mut i);
  add1(&mut i);
  println!("i={}", i);
}
fn add1(i: &mut i32) {
  *i += 1;
}
pub fn assert_ex_1() {
  let answer = 42;
  assert_eq!(answer, 42);
  // assert_eq!(answer, 40); // panic
}
pub fn ternary_ex() {
  for i in 0..5 {
    let even_odd = if i % 2 == 0 { "even" } else { "odd" };
    println!("{} {}", even_odd, i);
  }
}
pub fn calc_sum() {
  // add2.rs
  let mut sum = 0;
  for i in 0..5 {
    sum += i;
  }
  println!("sum is {}", sum);
}
pub fn casting_ex() {
  // add3.rs
  let mut sum = 0.0;
  for i in 0..5 {
    sum += i as f64;
  }
  println!("sum is {}", sum);
}
pub fn ex123() {
  println!("{}", sqr(3.0));
  println!("{}", abs(3.0));
  println!("{}", abs(-3.0));
  println!("{}", clamp(3.0, 1.0, 5.0));
  println!("{}", clamp(0.0, 1.0, 5.0));
  println!("{}", clamp(10.0, 1.0, 5.0));
  println!("{}", factorial(5));
}

// last line without semi-colon is returned
// This is because the body of the function (inside {})
// has the value of its last expression, just like with if-as-an-expression.

fn sqr(x: f64) -> f64 {
  x * x
}
// absolute value of a floating-point number
fn abs(x: f64) -> f64 {
  if x > 0.0 {
    x
  } else {
    -x
  }
}

// ensure the number always falls in the given range
fn clamp(x: f64, x1: f64, x2: f64) -> f64 {
  if x < x1 {
    x1
  } else if x > x2 {
    x2
  } else {
    x
  }
}

fn factorial(n: u64) -> u64 {
  if n == 0 {
    // for returning early this can be used
    return 1;
  }
  n * factorial(n - 1)
}
pub fn factorial2(n: u64) -> u64 {
  if n == 0 {
    1
  } else {
    n * factorial(n - 1)
  }
}

pub fn explicit_type() {
  let bigint: i64 = 0;
  let int = 0;
  println!("{}", bigint == int);
}

use std::f64::consts;

pub fn std_lib_ex() {
  let x = consts::PI / 2.0;
  let cosine = x.cos();
  println!("{}", cosine < 1e-10);
}

pub fn arrays_ex() {
  let arr = [10, 20, 30, 40];
  let first = arr[0];
  println!("first {}", first);

  for i in 0..4 {
    println!("[{}] = {}", i, arr[i]);
  }
  println!("length {}", arr.len());
  // index out of bounds: the length is 4 but the index is 4
  // compile time error
  // println!("{}", arr[4]);
}
pub fn sum_array_1() {
  let arr = [10, 20, 30, 40];
  // look at that &
  let res = sum(&arr);
  println!("sum {}", res);
}
fn sum(values: &[i32]) -> i32 {
  //  Rust slices keep track of their size
  // (and will panic if you try to access outside that size)
  // println!("{}", values[33]);
  let mut res = 0;
  for i in 0..values.len() {
    res += values[i]
  }
  res
}
pub fn printing_arrays_ex() {
  let ints = [1, 2, 3];
  let floats = [1.1, 2.1, 3.1];
  let strings = ["hello", "world"];
  let ints_ints = [[1, 2], [10, 20]];
  println!("ints {:?}", ints);
  println!("floats {:?}", floats);
  println!("strings {:?}", strings);
  println!("ints_ints {:?}", ints_ints);
}

/**
 * a copy of the data is never made.
 * These slices all borrow their data from their arrays.
 * They have a very intimate relationship with that array,
 * and Rust spends a lot of effort to make sure
 * that relationship does not break down.
 */
pub fn slicing_arrays() {
  let ints = [1, 2, 3, 4, 5];
  let slice1 = &ints[0..2];
  let slice2 = &ints[1..]; // open range!
  let slice3 = &ints[..3];
  println!("ints {:?}", ints);
  println!("slice1 {:?}", slice1);
  println!("slice2 {:?}", slice2);
  println!("slice2 {:?}", slice3);
}

pub fn slicing_arrays_mutable() {
  let mut ints = [1, 2, 3, 4, 5];
  let slice1 = &mut ints[0..2];
  println!("slice1 {:?}", slice1);
  slice1[1] += 2;
  println!("slice1 {:?}", slice1);
}

pub fn slice_assessing_safely_ex() {
  // slice2.rs
  let ints = [1, 2, 3, 4, 5];
  let slice = &ints;
  let first = slice.get(0);
  let last = slice.get(5);
  println!("first {:?}", first);
  println!("last {:?}", last);
  // first Some(1)
  // last None
  let v = slice.get(1);
  let value = v.unwrap();
  println!("v: {}", value);
  println!("{}", accessing_option(slice, 100));
  println!("{}", accessing_option(slice, 1));
  // alternative to accessing_option method;
  let last = *slice.get(5).unwrap_or(&-1000);
  println!("{}", last);
}
fn accessing_option(slice: &[i32], i: usize) -> i32 {
  let maybe_last = slice.get(i);
  let last = if maybe_last.is_some() {
    *maybe_last.unwrap()
  } else {
    -1
  };
  return last;
}
pub fn pass_by_value() {
  let ints = [1, 2, 3, 4, 5];
  let mut slice = ints;
  slice[0] += 1;
  println!("slice {:?}", slice);
  println!("ints {:?}", ints);
}
pub fn pass_by_ref() {
  let mut ints = [1, 2, 3, 4, 5];
  let slice = &mut ints;
  slice[0] += 1;
  println!("slice {:?}", slice);
  println!("ints {:?}", ints);
}
