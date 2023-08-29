struct Solution;

impl Solution {
    fn max(x:i32, y:i32) -> i32 {
        if x>y { x } else {y}
    }

    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut maxsum = i32::MIN;
        let mut bestprefix = 0;

        for n in nums {
            bestprefix += n;

            maxsum = Self::max(bestprefix, maxsum);
            bestprefix = Self::max(bestprefix, 0);
        }

        maxsum
    }
}

fn main() {
    let z = vec![
        vec![-2,1,-3,4,-1,2,1,-5,4],
        vec![1],
        vec![5,4,-1,7,8],
        vec![-1],
    ];

    for v in z {
        println!("{}", Solution::max_sub_array(v));
    }
}
