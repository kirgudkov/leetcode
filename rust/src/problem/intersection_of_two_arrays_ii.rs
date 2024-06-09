// https://leetcode.com/problems/intersection-of-two-arrays-ii
// Two pointers approach. TC is O(n log n + m log m)
pub fn intersection(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
    let mut res = vec![];

    nums1.sort_unstable(); // O(n log n)
    nums2.sort_unstable(); // O(m log m)

    let mut a = 0;
    let mut b = 0;

    while a < nums1.len() && b < nums2.len() { // O(min(n, m))
        match nums1[a].cmp(&nums2[b]) {
            std::cmp::Ordering::Equal => {
                res.push(nums1[a]);
                a += 1;
                b += 1;
            }
            std::cmp::Ordering::Less => {
                a += 1;
            }
            std::cmp::Ordering::Greater => {
                b += 1;
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection_bs() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        assert_eq!(intersection(nums1, nums2), vec![2, 2]);

        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        assert_eq!(intersection(nums1, nums2), vec![4, 9]);

        let nums1 = vec![3, 1, 2];
        let nums2 = vec![1, 1];
        assert_eq!(intersection(nums1, nums2), vec![1]);
    }
}