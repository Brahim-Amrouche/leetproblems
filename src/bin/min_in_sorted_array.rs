pub mod solution {

    pub fn find_min(nums: &[i32]) -> i32 {
        let mut left  = 0;
        let mut right = nums.len() - 1;
        while left < right {
            let mid = (left + right) / 2;
            if nums[mid] < nums[right] {
                right = mid;
            }
            else {
                left = mid + 1;
            }
        }
        return nums[left];
    }
}

fn main() {

}