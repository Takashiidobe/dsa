pub fn even_odd(arr: &mut [i32]) {
    if arr.is_empty() {
        return;
    }
    let (mut next_even, mut next_odd) = (0, arr.len() - 1);

    while next_even < next_odd {
        if arr[next_even] % 2 == 0 {
            next_even += 1;
        } else {
            arr.swap(next_odd, next_even);
            next_odd -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;
    use test::{black_box, Bencher};

    fn is_even_odd(arr: &[i32]) -> bool {
        let mut even = true;
        for num in arr {
            match (num % 2 == 0, even) {
                (false, false) | (true, true) => {}
                (true, false) => {
                    return false;
                }
                (false, true) => {
                    even = false;
                }
            }
        }
        true
    }

    #[quickcheck]
    fn verify_even_odd(mut arr: Vec<i32>) -> bool {
        even_odd(&mut arr);
        is_even_odd(&arr)
    }
}
