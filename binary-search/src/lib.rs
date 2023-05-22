use std::cmp::Ordering;

pub fn find<T: Ord + Copy, A: AsRef<[T]>>(array: A, key: T) -> Option<usize> {
    binary_search(array, key)
}

fn binary_search<T: Ord + Copy, A: AsRef<[T]>>(array: A, key: T) -> Option<usize> {
    let array = array.as_ref();
    if array.is_empty() {
        return None;
    }
    let middle_idx = array.len() / 2;
    let middle_elem = array[middle_idx];
    match key.cmp(&middle_elem) {
        Ordering::Equal => Some(middle_idx),
        Ordering::Less => {
            if array.len() == 1 {
               return None;
            }
            binary_search(&array[..middle_idx], key)
        },
        Ordering::Greater => {
            if array.len() == 1 {
                return None;
            }
            binary_search(&array[middle_idx..], key).and_then(|mi| Some(mi + middle_idx))
        },
    }
}
