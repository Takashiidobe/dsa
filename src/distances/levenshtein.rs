pub fn levenshtein_distance<S: AsRef<str>>(a: &S, b: &S) -> usize {
    let a = a.as_ref();
    let b = b.as_ref();

    let (len_a, len_b) = (a.len(), b.len());
    let mut cost = 0;
    let mut d = vec![vec![0; len_b + 1]; len_a + 1];

    for i in 0..=len_a {
        d[i][0] = i;
    }
    for j in 0..=len_b {
        d[0][j] = j;
    }

    for i in 1..=len_a {
        for j in 1..=len_b {
            cost = if a.chars().nth(i - 1) == b.chars().nth(j - 1) {
                0
            } else {
                1
            };
            d[i][j] = *[
                d[i - 1][j] + 1,        // deletion
                d[i][j - 1] + 1,        // insertion
                d[i - 1][j - 1] + cost, // substitution
            ]
            .iter()
            .min()
            .unwrap();
        }
    }

    d[len_a][len_b]
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn prop_levenshtein_distance_symmetric(a: String, b: String) -> bool {
        if a.len() > 50 || b.len() > 50 {
            return true;
        }
        levenshtein_distance(&a, &b) == levenshtein_distance(&b, &a)
    }

    #[quickcheck]
    fn prop_levenshtein_distance_identity(a: String) -> bool {
        if a.len() > 50 {
            return true;
        }
        levenshtein_distance(&a, &a) == 0
    }

    #[quickcheck]
    fn prop_levenshtein_triangle_inequality(a: String, b: String, c: String) -> bool {
        if a.len() > 50 || b.len() > 50 || c.len() > 50 {
            return true;
        }
        levenshtein_distance(&a, &c) <= levenshtein_distance(&a, &b) + levenshtein_distance(&b, &c)
    }

    #[quickcheck]
    fn prop_levenshtein_distance_positive(a: String, b: String) -> bool {
        if a.len() > 50 || b.len() > 50 {
            return true;
        }
        let distance = levenshtein_distance(&a, &b);
        distance <= a.len().max(b.len())
    }

    #[quickcheck]
    fn prop_levenshtein_distance_to_empty(a: String) -> bool {
        if a.len() > 50 {
            return true;
        }
        levenshtein_distance(&a, &"".to_string()) == a.len()
    }
}
