use std::cmp::Ordering;

pub fn quickselect(arr: &mut [i32], k: usize) -> i32 {
    if arr.len() <= 5 {
        arr.sort(); // Use sort for small sizes for simplicity
        return arr[k];
    }

    let median_of_medians = select_median_of_medians(arr);
    let pivot_index = partition(arr, median_of_medians);

    match k.cmp(&pivot_index) {
        Ordering::Less => quickselect(&mut arr[..pivot_index], k),
        Ordering::Equal => arr[pivot_index],
        Ordering::Greater => quickselect(&mut arr[pivot_index + 1..], k - pivot_index - 1),
    }
}

fn select_median_of_medians(arr: &mut [i32]) -> i32 {
    let mut medians: Vec<i32> = arr
        .chunks_mut(5)
        .map(|chunk| {
            chunk.sort();
            chunk[chunk.len() / 2] // Get the median of the chunk
        })
        .collect();

    let mid = medians.len() / 2;
    quickselect(&mut medians, mid) // Recursively find the median of medians
}

fn partition(arr: &mut [i32], pivot: i32) -> usize {
    let mut i = 0;
    let mut j = arr.len() - 1;

    // Move pivot to the end
    let pivot_index = arr.iter().position(|&x| x == pivot).unwrap();
    arr.swap(pivot_index, j);

    while i < j {
        while i < j && arr[i] < pivot {
            i += 1;
        }
        while i < j && arr[j] >= pivot {
            j -= 1;
        }
        if i < j {
            arr.swap(i, j);
        }
    }

    // Swap back the pivot to its correct place
    arr.swap(i, arr.len() - 1);
    i
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn quickcheck_quickselect(mut xs: Vec<i32>) -> bool {
        if xs.is_empty() {
            return true; // Skip empty array cases
        }
        let len = xs.len();
        let mut sorted_copy = xs.clone();
        sorted_copy.sort();
        for i in (0..len) {
            let result = quickselect(&mut xs, i);

            if result != sorted_copy[i] {
                return false;
            }
        }
        true
    }
}
