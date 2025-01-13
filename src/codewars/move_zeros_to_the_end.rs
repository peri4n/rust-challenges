fn move_zeros(arr: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(arr.len());
    let mut zeros = 0;
    
    for x in arr {
        if *x == 0 {
            zeros += 1;
        } else {
            result.push(*x);
        }
    }
    
    for i in 0..zeros {
        result.push(0);
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name() {
        assert_eq!(move_zeros(&vec![1,0,1,2,0,1,3,5]), vec![1,1,2,1,3,5,0,0]);
    }
}
