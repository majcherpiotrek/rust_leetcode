use std::convert::TryFrom;

// https://leetcode.com/problems/two-sum/
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut sorted = nums.iter().enumerate().collect::<Vec<(usize, &i32)>>();
    sorted.sort_by(|(_, v_a), (_, v_b)| v_a.cmp(v_b));

    for (sorted_index, (i, num_a)) in sorted.iter().enumerate() {
        let slice_to_search = &sorted[sorted_index + 1..];
        let search = target - *num_a;
        match slice_to_search
            .binary_search_by(|elem| elem.1.cmp(&search))
            .ok()
            .and_then(|index| slice_to_search.get(index))
            .and_then(|(j, _)| i32::try_from(*j).ok())
            .and_then(|j| i32::try_from(*i).ok().map(|i| (i, j)))
            .map(|(i, j)| vec![i, j])
        {
            None => continue,
            Some(res) => return res,
        }
    }

    Vec::new()
}
