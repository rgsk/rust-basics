use super::super::utils;
pub fn bubble_sort(arr: &mut [i32]) {
  for i in 0..arr.len() - 1 {
    for j in 0..arr.len() - i - 1 {
      if arr[j + 1] < arr[j] {
        utils::general::swap_array_indexes(arr, j, j + 1);
      }
    }
  }
}

pub fn selection_sort(arr: &mut [i32]) {
  for i in 0..arr.len() - 1 {
    let mut min_index = i;
    for j in i + 1..arr.len() {
      if arr[j] < arr[min_index] {
        min_index = j;
      }
    }
    utils::general::swap_array_indexes(arr, i, min_index);
  }
}

pub fn insertion_sort(arr: &mut [i32]) {
  for i in 1..arr.len() {
    let mut j = i;
    while j > 0 && arr[j] < arr[j - 1] {
      utils::general::swap_array_indexes(arr, j, j - 1);
      j -= 1;
    }
  }
}
