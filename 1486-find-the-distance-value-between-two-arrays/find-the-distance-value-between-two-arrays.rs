impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        let mut arr2 = arr2;
        arr2.sort_unstable();
        
        let mut count = 0;
        
        for &val in &arr1 {
            let mut left = 0;
            let mut right = arr2.len() as i32 - 1;
            let mut valid = true;
            
            while left <= right {
                let mid = left + (right - left) / 2;
                let diff = (arr2[mid as usize] - val).abs();
                
                if diff <= d {
                    valid = false;
                    break;
                }
                
                if arr2[mid as usize] < val {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
            
            if valid {
                count += 1;
            }
        }
        
        count
    }
}