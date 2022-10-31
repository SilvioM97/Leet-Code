//Linear solution O(n)

//At the moment of submission (31/10/22):
/*
Runtime: 7 ms, faster than 98.20% of Rust online submissions for Squares of a Sorted Array.
Memory Usage: 2.2 MB, less than 93.53% of Rust online submissions for Squares of a Sorted Array.
*/


impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res: Vec<i32> = vec![0; n];
        
        if n == 1 {
            res[0] = nums[0].pow(2);
        }
        else if nums[0] >= 0 {
            for i in 0..n {
                res[i] = (nums[i].pow(2));
            }
        }
        else if nums[n-1] <= 0 {
            for i in 0..n {
                res[n-i-1] = nums[i].pow(2);
            }
        }
        else {
            let mut s: usize = 0;
            let mut d = n-1;
            while s < d {
                if nums[s].abs() <= nums[d].abs() {
                    res[d-s] = nums[d].pow(2);
                    d = d - 1;
                } 
                else {
                    res[d-s] = nums[s].pow(2);
                    s = s + 1;
                }
            }
            res[0] = nums[s].pow(2);
        }
        return res;
    }
}
