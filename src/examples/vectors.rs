// vec1.rs
pub fn basic_vec_push() {
  let mut v = Vec::new();
  v.push(10);
  v.push(20);
  v.push(30);

  let first = v[0]; // will panic if out-of-range
  let maybe_first = v.get(0);

  println!("v is {:?}", v);
  println!("first is {}", first);
  println!("maybe_first is {:?}", maybe_first);
}
// v is [10, 20, 30]
// first is 10
// maybe_first is Some(10)

// vec2.rs
fn dump(arr: &[i32]) {
  println!("arr is {:?}", arr);
}

pub fn vec_slice() {
  let mut v = Vec::new();
  v.push(10);
  v.push(20);
  v.push(30);

  dump(&v);

  let slice = &v[1..];
  println!("slice is {:?}", slice);
}

// vec3.rs
pub fn extend_ex() {
  let mut v1 = vec![10, 20, 30, 40];
  v1.pop();

  let mut v2 = Vec::new();
  v2.push(10);
  v2.push(20);
  v2.push(30);

  assert_eq!(v1, v2);
  if v1 == v2 {
    println!("v1 and v2 are equal");
  }

  v2.extend(0..2);
  assert_eq!(v2, [10, 20, 30, 0, 1]);
  assert_eq!(v2, &[10, 20, 30, 0, 1]);
}

pub fn vectors_methods_ex() {
  let mut v1 = vec![10, 20, 30, 40];
  v1.insert(1, 100);
  println!("{:?}", v1);
  v1.remove(1);
  println!("{:?}", v1);
  v1.push(99);
  println!("{:?}", v1);
  v1.pop();
  println!("{:?}", v1);
  v1.extend(78..80);
  println!("{:?}", v1);
  let mut v2 = v1.clone();
  v1.clear();
  println!("{:?}", v1);
  println!("{:?}", v2);
  v2.insert(1, 1);
  v2.insert(1, 2);
  v2.insert(1, 3);
  v2.insert(1, 10);
  println!("{:?}", v2);
  v2.sort();
  println!("{:?}", v2);
  v2.dedup(); // removes the duplicate elems
  println!("{:?}", v2);
}
