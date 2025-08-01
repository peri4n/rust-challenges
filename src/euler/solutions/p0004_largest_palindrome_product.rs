use crate::helper::misc;

pub fn largest_palindrome_product() -> i32 {
    let mut max_palindrome = 0;

    for i in (100..1000).rev() {
        for j in (i..1000).rev() {
            let product = i * j;

            if product <= max_palindrome {
                break; // No need to check smaller products
            }

            if misc::is_palindrome(product) {
                max_palindrome = product;
            }
        }
    }

    max_palindrome
}

#[cfg(test)]
mod test {
    use super::largest_palindrome_product;

    #[test]
    fn solution() {
        assert_eq!(largest_palindrome_product(), 906609);
    }
}
