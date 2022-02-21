// string1.rs
fn dump(s: &str) {
  println!("str '{}'", s);
}

pub fn first() {
  let text = "hello dolly"; // the string slice
  let s = text.to_string(); // it's now an allocated string

  dump(text);
  dump(&s);
}
pub fn second() {
  let i = 5;
  let five = String::from("5");
  println!("{}", five == i.to_string());
}
// string5.rs
pub fn third() {
  let mut s = String::new();
  // initially empty!
  s.push('H');
  s.push_str("ello");
  s.push(' ');
  s += "World!"; // short for `push_str`

  s.pop(); // remove the last char
  println!("{}", s);
  assert_eq!(s, "Hello World");
}
fn array_to_str(arr: &[i32]) -> String {
  let mut res = '['.to_string();
  for v in arr {
    res += &v.to_string();
    res.push(',');
  }
  res.pop();
  res.push(']');
  res
}
pub fn fourth() {
  let arr = array_to_str(&[10, 20, 30]);
  let res = format!("hello {}", arr);
  println!("{}", res);
  assert_eq!(res, "hello [10,20,30]");
}
pub fn slice_notation_on_strings() {
  let text = "static";
  let string = "dynamic".to_string();

  let text_s = &text[1..];
  let string_s = &string[2..4];

  println!("slices {:?} {:?}", text_s, string_s);
}

pub fn string_looping() {
  let multilingual = "Hi! Hola!! nora";
  for ch in multilingual.chars() {
    print!("'{}' ", ch);
  }
  println!("");
  println!("len {}", multilingual.len());
  println!("count {}", multilingual.chars().count());

  let maybe = multilingual.find('n');
  if maybe.is_some() {
    let hi = &multilingual[maybe.unwrap()..];
    println!("Russian hi {}", hi);
  }
  let s = "i";
  println!("{}", &s[0..1]);
}

pub fn collect() {
  let text = "the red fox and the lazy dog, Rahul Gupta";
  // text.split_whitespace() returns an iterator
  // .collect converts an iterator over strings into vector
  let words: Vec<&str> = text.split_whitespace().collect();
  println!("{:?}", words);
  let mut words_list = Vec::new();
  words_list.extend(text.split_whitespace());
  println!("{:?}", words_list);
}
pub fn simple_split() {
  let text = "the red fox andr the lazyr dog, Rahul Gupta";
  let words: Vec<&str> = text.split("r").collect();
  println!("{:?}", words);
}

pub fn magic() {
  let text = "the red fox andr the lazyr dog";
  // here collects characters into a string
  let stripped: String = text.chars().filter(|x| !x.is_whitespace()).collect();
  println!("{:?}", stripped);

  let characters = text.chars();
  let without_whitespace = characters.filter(|x| !x.is_whitespace());
  for c in without_whitespace {
    print!("{}", c);
  }
  println!();
}
