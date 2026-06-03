//  Category: algorithms
//  Level: Medium
//  Percent: 34.445%

//  You are given two categories of theme park attractions: land rides and water rides.
//
//
//  	Land rides
//
//
//  		landStartTime[i] – the earliest time the ith land ride can be boarded.
//  		landDuration[i] – how long the ith land ride lasts.
//
//
//  	Water rides
//
//  		waterStartTime[j] – the earliest time the jth water ride can be boarded.
//  		waterDuration[j] – how long the jth water ride lasts.
//
//
//
//
//  A tourist must experience exactly one ride from each category, in either order.
//
//
//  	A ride may be started at its opening time or any later moment.
//  	If a ride is started at time t, it finishes at time t + duration.
//  	Immediately after finishing one ride the tourist may board the other (if it is already open) or wait until it opens.
//
//
//  Return the earliest possible time at which the tourist can finish both rides.
//
//
//  Example 1:
//
//
//  Input: landStartTime = [2,8], landDuration = [4,1], waterStartTime = [6], waterDuration = [3]
//
//  Output: 9
//
//  Explanation:​​​​​​​
//
//
//  	Plan A (land ride 0 → water ride 0):
//
//  		Start land ride 0 at time landStartTime[0] = 2. Finish at 2 + landDuration[0] = 6.
//  		Water ride 0 opens at time waterStartTime[0] = 6. Start immediately at 6, finish at 6 + waterDuration[0] = 9.
//
//
//  	Plan B (water ride 0 → land ride 1):
//
//  		Start water ride 0 at time waterStartTime[0] = 6. Finish at 6 + waterDuration[0] = 9.
//  		Land ride 1 opens at landStartTime[1] = 8. Start at time 9, finish at 9 + landDuration[1] = 10.
//
//
//  	Plan C (land ride 1 → water ride 0):
//
//  		Start land ride 1 at time landStartTime[1] = 8. Finish at 8 + landDuration[1] = 9.
//  		Water ride 0 opened at waterStartTime[0] = 6. Start at time 9, finish at 9 + waterDuration[0] = 12.
//
//
//  	Plan D (water ride 0 → land ride 0):
//
//  		Start water ride 0 at time waterStartTime[0] = 6. Finish at 6 + waterDuration[0] = 9.
//  		Land ride 0 opened at landStartTime[0] = 2. Start at time 9, finish at 9 + landDuration[0] = 13.
//
//
//
//
//  Plan A gives the earliest finish time of 9.
//
//
//  Example 2:
//
//
//  Input: landStartTime = [5], landDuration = [3], waterStartTime = [1], waterDuration = [10]
//
//  Output: 14
//
//  Explanation:​​​​​​​
//
//
//  	Plan A (water ride 0 → land ride 0):
//
//  		Start water ride 0 at time waterStartTime[0] = 1. Finish at 1 + waterDuration[0] = 11.
//  		Land ride 0 opened at landStartTime[0] = 5. Start immediately at 11 and finish at 11 + landDuration[0] = 14.
//
//
//  	Plan B (land ride 0 → water ride 0):
//
//  		Start land ride 0 at time landStartTime[0] = 5. Finish at 5 + landDuration[0] = 8.
//  		Water ride 0 opened at waterStartTime[0] = 1. Start immediately at 8 and finish at 8 + waterDuration[0] = 18.
//
//
//
//
//  Plan A provides the earliest finish time of 14.​​​​​​​
//
//
//
//  Constraints:
//
//
//  	1 <= n, m <= 5 * 10⁴
//  	landStartTime.length == landDuration.length == n
//  	waterStartTime.length == waterDuration.length == m
//  	1 <= landStartTime[i], landDuration[i], waterStartTime[j], waterDuration[j] <= 10⁵
//

//  // @submit start
impl Solution {
    #[inline]
    fn calculate_time(min_start: i32, to_choose: impl Iterator<Item = (i32, i32)>) -> i32 {
        to_choose
            .map(|(t, d)| if t >= min_start { t + d } else { min_start + d })
            .min()
            .unwrap()
    }

    pub fn earliest_finish_time(
        mut land_start_time: Vec<i32>,
        mut land_duration: Vec<i32>,
        mut water_start_time: Vec<i32>,
        mut water_duration: Vec<i32>,
    ) -> i32 {
        let land_iter = land_start_time
            .iter()
            .zip(land_duration.iter())
            .map(|(&t, &d)| (t, d));
        let water_iter = water_start_time
            .iter()
            .zip(water_duration.iter())
            .map(|(&t, &d)| (t, d));

        let land_min = land_iter.clone().map(|(t, d)| t + d).min().unwrap();
        let land_min = Self::calculate_time(land_min, water_iter.clone());

        let water_min = water_iter.map(|(t, d)| t + d).min().unwrap();
        let water_min = Self::calculate_time(water_min, land_iter);

        land_min.min(water_min)
    }
}
//  // @submit end
