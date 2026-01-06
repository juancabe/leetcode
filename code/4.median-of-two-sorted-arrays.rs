impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {

        let n1 = nums1.len();
        let n2 = nums2.len();

        // Ensure nums1 is the smaller array to minimize binary search range
        if n1 > n2 {
            return Self::find_median_sorted_arrays(nums2, nums1);
        }

        let mut low = 0;
        let mut high = n1; 

        while low <= high {
            // Partition nums1
            let i = (low + high) / 2;
            // Partition nums2 based on nums1
            // (n1 + n2 + 1) / 2 ensures that if total is odd, left half has one extra element
            let j = (n1 + n2 + 1) / 2 - i;

            // 4. Handle Edge Cases using MIN/MAX to simulate Infinity
            // L1, R1 are the elements around the cut in nums1
            let l1 = if i == 0 { i32::MIN } else { nums1[i - 1] };
            let r1 = if i == n1 { i32::MAX } else { nums1[i] };

            // L2, R2 are the elements around the cut in nums2
            let l2 = if j == 0 { i32::MIN } else { nums2[j - 1] };
            let r2 = if j == n2 { i32::MAX } else { nums2[j] };

            // Check if the partition is valid (Cross conditions)
            if l1 <= r2 && l2 <= r1 {
                // FOUND IT!
                if (n1 + n2) % 2 == 0 {
                    // Even total length: Avg of max(lefts) and min(rights)
                    return (l1.max(l2) + r1.min(r2)) as f64 / 2.0;
                } else {
                    // Odd total length: Just the max of lefts
                    return l1.max(l2) as f64;
                }
            } else if l1 > r2 {
                // We have too many large elements from nums1 on the left. 
                // Move cut to the left.
                high = i - 1;
            } else {
                // We have too many small elements from nums1 on the left.
                // Move cut to the right.
                low = i + 1;
            }
        }

        unreachable!("Should not be reached for sorted arrays!")
    }
}

