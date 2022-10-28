//This solution uses a deque (double ended queue) and it is linear (O(n) worst case)

//At the moment of submission (19/10/22):
/*
Runtime: 62 ms, faster than 99.27% of Rust online submissions for Palindrome Number.
Memory Usage: 4.3 MB, less than 28.73% of Rust online submissions for Palindrome Number.
*/

use std::collections::VecDeque;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = nums.len();
    let mut maximums: Vec<i32> = Vec::with_capacity(n - k + 1);
    let mut deq: VecDeque<(i32, usize)> = VecDeque::new();
    let mut cont: bool = false; //A control variable to check if we took an element from deque or not

    for i in 0..n {
        while !cont {
            if !deq.is_empty() && i >= k && (*((deq.front()).unwrap())).1 < (i - k + 1) {
                //If the deque is not empty remove the head element if it is not in the window
                deq.pop_front();
            } else {
                cont = true; //If the deque is empty or if the head is in the window, set cont to true and go ahead
            }
        }
        cont = false; //Reset cont

        while !cont {
            if !deq.is_empty() && (*((deq.back()).unwrap())).0 < nums[i] {
                //Check if the deq is not empty and if the tail elements are smaller than the new element
                deq.pop_back(); //If so, remove them
            } else {
                cont = true; //When the deque is empty or the tail element is not smaller than the new one, set cont to true and go ahead
            }
        }
        cont = false; //Reset cont
        deq.push_back((nums[i], i)); //Move the window one position to the right and add the new element

        if i >= (k - 1) {
            //Check if we already loaded the first k elements
            maximums.push((*((deq.front()).unwrap())).0) //If so, take the maximum as the front element
        }
    }
    maximums
    }
}
