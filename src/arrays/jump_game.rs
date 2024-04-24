use num_traits::PrimInt;

fn jump_game<N: PrimInt>(a: &[N]) -> bool {
    let (mut max_so_far, mut last_index) = (0_usize, a.len() - 1);

    let mut i = 0;
    while i <= max_so_far && max_so_far < last_index {
        max_so_far = max_so_far.max(a[i].to_usize().unwrap() + i);
        i += 1;
    }
    max_so_far >= last_index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert!(jump_game(&[3, 3, 1, 0, 2, 0, 1]))
    }
}
