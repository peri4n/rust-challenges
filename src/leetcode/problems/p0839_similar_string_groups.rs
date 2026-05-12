use crate::helper::quickfind::QuickFind;

fn similar(str1: &str, str2: &str) -> bool {
    let mut mismatches = 0;
    for (c1, c2) in str1.chars().zip(str2.chars()) {
        if c1 != c2 {
            mismatches += 1
        }

        if mismatches > 2 {
            return false;
        }
    }

    mismatches == 2 || mismatches == 0
}

pub fn num_similar_groups(strs: Vec<String>) -> i32 {
    let mut find = QuickFind::new(strs.len());

    for i in 0..strs.len() {
        for j in i + 1..strs.len() {
            if similar(&strs[i], &strs[j]) {
                find.connect(i, j);
            }
        }
    }

    find.groups() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cases() {
        assert_eq!(
            num_similar_groups(vec![
                "tars".into(),
                "rats".into(),
                "arts".into(),
                "star".into()
            ]),
            2
        );
        assert_eq!(num_similar_groups(vec!["omv".into(), "ovm".into()]), 1);
        assert_eq!(num_similar_groups(vec!["abc".into(), "abc".into()]), 1);
    }
}
