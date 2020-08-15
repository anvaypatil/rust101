use std::collections::HashMap;


pub fn execute() {
    let nums: Vec<i32> = Vec::from([3, 2, 4]);
    let target: i32 = 6;
    let result = Solution::two_sums_functional(nums, target);
    println!("{:?}", result);
}

struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn two_sums(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for (index, num) in nums.iter().enumerate() {
            map.insert(*num, index as i32);
        }
        for (index, num) in nums.iter().enumerate() {
            let remain = *num - target;
            if map.contains_key(&remain) && index as i32 != *map.get(&remain).unwrap(){
                let matched_index: &i32 = map.get(&remain).unwrap();
                return Vec::from([index as i32, *matched_index]);
            }
        }
        return Vec::new();
    }
    pub fn two_sums_functional(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let map: HashMap<i32, i32> = nums.iter()
            .enumerate()
            .fold(HashMap::<i32, i32>::new(),
                  |mut acc, (index, num)| {
                      acc.insert(*num, index as i32);
                      acc
                  });
        let result = nums.iter()
            .enumerate()
            .find_map(|(index, num): (usize, &i32)| -> Option<Vec<i32>>{
                let remain =  target - *num;
                return match map.contains_key(&remain) && index as i32 != *map.get(&remain).unwrap(){
                    true => Some(vec![index as i32, *map.get(&remain).unwrap()]),
                    false => None
                };
            });

        result.unwrap()
    }
}
