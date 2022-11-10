fn permute(nums: &mut Vec<i32>, pos: usize) {
    if pos == nums.len() - 1 {
        if fit(nums) {
            println!("Found solution: {:?}", nums);
        }
        return;
    }
    for i in pos..nums.len() {
        let v = nums[pos];
        nums[pos] = nums[i];
        nums[i] = v;
        permute(nums, pos + 1);
        restore(nums, pos + 1);
    }
}

fn restore(nums: &mut Vec<i32>, pos: usize) {
    for i in pos..nums.len() - 1 {
        let v = nums[i];
        nums[i] = nums[i + 1];
        nums[i + 1] = v;
    }
}

fn fit(c: &Vec<i32>) -> bool {
    assert_eq!(c.len(), 5, "wrong input coin vector");
    c[0] + c[1] * c[2].pow(2) + c[3].pow(3) - c[4] == 399
}

fn main() {
    println!("Order coins this way:");
    let mut coins = vec![2, 3, 5, 7, 9];
    permute(&mut coins, 0);
}