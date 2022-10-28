//At the moment of submission (25/10/22):
/*
Runtime: 0 ms, faster than 100.00% of Rust online submissions for Two Sum.
Memory Usage: 2.7 MB, less than 6.84% of Rust online submissions for Two Sum.
*/

//This is a linear solution of the problem which uses hash tables,
//it enters the second for loop only one time, so the asympthotic cost is O(n)+O(n)=O(n)

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        
        let mut tab = HashMap::new(); //Hash table
        let mut res = Vec::<i32>::new(); //Output
        
        for i in 0..nums.len() { 
            //Checking for the complementar of nums[i]
            if tab.contains_key(&(target-nums[i])) { 
                //Searching the 
                for j in 0..nums.len() {
                    if j != i {
                        if nums[j] == target-nums[i] {
                            if i < j {
                                res.push(i as i32);
                                res.push(j as i32);
                                return res;
                            }
                            else {
                                res.push(j as i32);
                                res.push(i as i32);
                                return res;
                            }
                        }
                    }
                }
            }
            tab.insert(nums[i],i);
        }
        return res;
    }
}
