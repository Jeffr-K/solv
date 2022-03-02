

/**
 * case: Max Consecutive Ones
 * owner: jeffrey
 * provider: Leetcode
 * created: '03.02.2022'
 * updated: 'none'
 */
pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut count: i32 = 0;
    let mut result: i32 = 0;
    for el in nums {
        if el == 1 {
            count += 1;
        } else {
            result = result.max(count);
            count = 0;
        }
    }

    result.max(count)

    // nums.split(|n| *n == 0)
    //     .map(|i| i.len())
    //     .max()
    //     .unwrap() as i32
}

/**
 * case: Find Numbers with Even Number of Digits
 * owner: jeffrey
 * provider: Leetcode
 * created: '03.02.2022'
 * updated: 'none'
 */
pub fn find_numbers(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    
    for el in nums {
        let digits = el.to_string();

        if digits.len() % 2 == 0 {
            count += 1;
        } 
    }

    count
    // nums.iter()
    // .map(|num| num.to_string().len())
    // .filter(|len| len % 2 == 0)
    // .count() as i32   
}

/**
 * case: Squares of a Sorted Array
 * owner: jeffrey
 * provider: Leetcode
 * created: '03.02.2022'
 * updated: 'none'
 * time complexity: 'none'
 * space complexity: 'none'
 */
pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut result = nums
    .iter()
    .map(|num| num * num)
    .collect::<Vec<i32>>();

    result.sort();
    result
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_max_consecutive_ones_test() {
        let test_case_one = vec![1,1,0,1,1,1];
        let test_case_two = vec![1,0,1,1,0,1];
        assert_eq!(3, find_max_consecutive_ones(test_case_one));
        assert_eq!(2, find_max_consecutive_ones(test_case_two));
    }

    #[test]
    fn find_numbers_test() {
        let test_case_one = vec![12,345,2,6,7896];
        let test_case_two = vec![555,901,482,1771];
        assert_eq!(2, find_numbers(test_case_one));
        assert_eq!(1, find_numbers(test_case_two));
    }

    #[test]
    fn sorted_squares_test() {
        let test_case_one = vec![-4,-1,0,3,10];
        let test_case_one_expected = vec![0,1,9,16,100];
        let test_case_two = vec![-7,-3,2,3,11];
        let test_case_two_expected = vec![4,9,9,49,121];
        assert_eq!(test_case_one_expected, sorted_squares(test_case_one));
        assert_eq!(test_case_two_expected, sorted_squares(test_case_two));
    }
    
}
