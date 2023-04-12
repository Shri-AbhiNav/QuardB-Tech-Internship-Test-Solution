// Find the maximum subarray sum in Rust.

fn maxSum(nums: &[i32]) -> i32 {
    let mut max_sum=nums[0];
    let mut current_sum=0;

    for &num in nums {
        current_sum+=num;

        if current_sum<0 {
            current_sum=0;
        }

        if current_sum>max_sum {
            max_sum=current_sum;
        }
    }

    max_sum
}

fn main() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let max_sum = maxSum(&nums);
    println!("Max subarray sum: {}", max_sum);
}
