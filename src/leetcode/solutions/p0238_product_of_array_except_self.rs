/// The initial idea was to do two scans:
/// 1. From right to left, compute the product of all elements to the right of
///    each index and store them in an auxiliary array.
/// 2. From left to right, compute the product of all elements to the right of
///    each index and store them in an auxiliary array.
///
/// Afterwards we can compute the result by multiplying the products from both
/// scans for each index.
///
/// However, we can optimize the space usage by only storing the results
/// from the right-to-left scan in an auxiliary array, and using a single
/// variable to keep track of the left-to-right product during the second scan.
///
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let right_scan_prod = {
        let mut prod = 1;
        nums.iter()
            .rev()
            .map(|&x| {
                let res = prod;
                prod *= x;
                res
            })
            .collect::<Vec<i32>>()
    };

    let mut result = vec![0; nums.len()];
    let mut prod = 1;
    for i in 0..nums.len() {
        result[i] = prod * right_scan_prod[nums.len() - 1 - i];
        prod = nums[i] * prod;
    }
    result
}

#[cfg(test)]
mod test {

    use super::product_except_self;

    #[test]
    fn examples() {
        assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);

        assert_eq!(
            product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }
}
