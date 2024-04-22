pub fn rabin_karp(text: &str, pattern: &str) -> bool {
    let (t_len, p_len) = (text.len(), pattern.len());
    //@ If either the pattern or text is empty or the pattern is longest than the text, there are
    //@ no possible matches, so return false.
    if text.is_empty() || pattern.is_empty() || p_len > t_len {
        return false;
    }

    let base = 257; //@ A prime base for hashing
    let modulus = 1_000_000_007; //@ A large prime modulus

    let mut pattern_hash = 0;
    let mut text_hash = 0;
    //@ Highest power of base used for leftmost character
    let mut highest_pow = 1;

    //@ Precompute the highest power of base needed
    for _ in 0..p_len - 1 {
        highest_pow = highest_pow * base % modulus;
    }

    let pattern_bytes = pattern.as_bytes();
    let text_bytes = text.as_bytes();

    //@ Calculate the hash of the pattern and the first window of the text
    for i in 0..p_len {
        pattern_hash = (pattern_hash * base + pattern_bytes[i] as usize) % modulus;
        text_hash = (text_hash * base + text_bytes[i] as usize) % modulus;
    }

    //@ Slide over the text to compare hashes and if needed, check for actual matches
    for s in 0..=t_len - p_len {
        if pattern_hash == text_hash {
            // Verify this is not a hash collision
            if &text[s..s + p_len] == pattern {
                return true;
            }
        }

        //@ Compute the hash of the next window
        if s < t_len - p_len {
            text_hash =
                (text_hash + modulus - text_bytes[s] as usize * highest_pow % modulus) % modulus;
            text_hash = (text_hash * base + text_bytes[s + p_len] as usize) % modulus;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    //@ Here since the stdlib implements the algorithm correctly, we can compare our implementation
    //@ against the stdlib's
    #[quickcheck]
    fn correct(s: String, t: String) -> bool {
        s.contains(&t) == rabin_karp(&s, &t)
    }

    #[test]
    fn match_case() {
        let text = "hello world";
        let pattern = "wor";
        let matches = rabin_karp(text, pattern);
        assert!(matches);
    }

    #[test]
    fn fail_case() {
        let text = "hello world";
        let pattern = "help";
        let matches = rabin_karp(text, pattern);
        assert!(!matches);
    }
}
