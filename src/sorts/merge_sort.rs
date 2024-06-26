fn merge<T: Ord + Clone>(arr: &mut [T], mid: usize) {
    let left_half = arr[..mid].to_vec();
    let right_half = arr[mid..].to_vec();
    let left_len = left_half.len();
    let right_len = right_half.len();

    let mut l_idx = 0;
    let mut r_idx = 0;

    for v in arr {
        let left_cond = l_idx < left_len;
        let right_cond = r_idx < right_len;
        let (left, right) = match (left_cond, right_cond) {
            (true, true) => (Some(&left_half[l_idx]), Some(&right_half[r_idx])),
            (true, false) => (Some(&left_half[l_idx]), None),
            (false, true) => (None, Some(&right_half[r_idx])),
            (false, false) => (None, None),
        };
        if r_idx == right_len || (left_cond && left < right) {
            *v = left_half[l_idx].clone();
            l_idx += 1;
        } else {
            *v = right_half[r_idx].clone();
            r_idx += 1;
        }
    }
}

pub fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    match arr.len() {
        0 | 1 => {}
        2.. => {
            let mid = arr.len() / 2;
            merge_sort(&mut arr[..mid]);
            merge_sort(&mut arr[mid..]);

            merge(arr, mid);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PEAK_ALLOC;
    use quickcheck_macros::quickcheck;
    use serde::Serialize;

    #[test]
    fn memory_usage_merge_sort() {
        let mut vec_to_sort: Vec<i32> = (0..=10000).rev().collect();
        merge_sort(&mut vec_to_sort);
        println!("{}", PEAK_ALLOC.peak_usage());
    }

    #[test]
    fn memory_usage_sort() {
        let mut vec_to_sort: Vec<i32> = (0..=10000).rev().collect();
        vec_to_sort.sort_unstable();
        println!("{}", PEAK_ALLOC.peak_usage());
    }

    fn is_sorted<T>(arr: &mut [T]) -> bool
    where
        T: Ord,
    {
        if arr.len() <= 1 {
            return true;
        }
        let mut i = 0;
        let mut j = 1;
        while j < arr.len() {
            if arr[i] > arr[j] {
                return false;
            }
            i += 1;
            j += 1;
        }
        true
    }

    #[quickcheck]
    fn merge_sort_is_sorted(mut arr: Vec<i32>) -> bool {
        merge_sort(&mut arr);
        is_sorted(&mut arr)
    }
}
