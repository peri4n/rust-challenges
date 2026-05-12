pub fn are_almost_equal(s1: String, s2: String) -> bool {
    let mut mismatches = s1.chars().zip(s2.chars()).filter(|(c1, c2)| c1 != c2);

    let Some(first_mismatch) = mismatches.next() else {
        // both strings are equal
        return true;
    };

    let Some(second_mismatch) = mismatches.next() else {
        // there is only one mismatch
        return false;
    };

    if mismatches.next().is_some() {
        // there is a third mismatch
        false
    } else {
        // check if you can swap the characters of the two mismatches
        first_mismatch.1 == second_mismatch.0 && first_mismatch.0 == second_mismatch.1
    }
}

#[cfg(test)]
mod test {

    use super::are_almost_equal;

    #[test]
    fn cases() {
        assert!(are_almost_equal("bank".into(), "kanb".into()));
        assert!(!are_almost_equal("attack".into(), "defend".into()));
        assert!(are_almost_equal("kelp".into(), "kelp".into()));
        assert!(!are_almost_equal("caa".into(), "aaz".into()));
    }
}
