//Linear solution O(n)

//At the moment of submission (24/09/22):
/*
Runtime: 5 ms, faster than 99.28% of Rust online submissions for Palindrome Number.
Memory Usage: 2.3 MB, less than 59.71% of Rust online submissions for Palindrome Number.
*/


impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res: Vec<i32> = vec![0; n];
        
        if n == 1 {
            res[0] = nums[0]*nums[0];
        }
        else if nums[0] >= 0 {
            for i in 0..n {
                res[i] = (nums[i]*nums[i]);
            }
        }
        else if nums[n-1] <= 0 {
            for i in 0..n {
                res[n-i-1] = nums[i]*nums[i];
            }
        }
        else {
            let mut s: usize = 0;
            let mut d = n-1;
            while s < d {
                if nums[s].abs() <= nums[d].abs() {
                    res[d-s] = nums[d]*nums[d];
                    d = d - 1;
                } 
                else {
                    res[d-s] = nums[s]*nums[s];
                    s = s + 1;
                }
            }
            res[0] = nums[s]*nums[s];
        }
        return res;
    }
}
