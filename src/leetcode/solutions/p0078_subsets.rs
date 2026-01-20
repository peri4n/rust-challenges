pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    let mut buffer = Vec::new();
    subsets_helper(&mut res, &mut buffer, &nums, 0);
    res
}

fn subsets_helper(res: &mut Vec<Vec<i32>>, buffer: &mut Vec<i32>, nums: &Vec<i32>, index: usize) {
    res.push(buffer.clone());
    println!("Added subset: {:?}", buffer);

    for i in index..nums.len() {
        buffer.push(nums[i]);
        subsets_helper(res, buffer, nums, i + 1);
        buffer.pop();
    }
}

#[cfg(test)]
mod test {

    use super::subsets;
    use googletest::prelude::*;

    #[test]
    fn case1() {
        assert_that!(
            subsets(vec![1, 2, 3]),
            unordered_elements_are![
                &Vec::<i32>::new(),
                &vec![1],
                &vec![2],
                &vec![3],
                &vec![1, 2],
                &vec![1, 3],
                &vec![2, 3],
                &vec![1, 2, 3]
            ]
        );
    }

    #[test]
    fn case2() {
        assert_that!(
            subsets(vec![0]),
            unordered_elements_are![
                &Vec::<i32>::new(),
                &vec![0]
            ]
        );
    }
}

