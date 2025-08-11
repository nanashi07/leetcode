// # 2438. Range Product Queries of Powers
// https://leetcode.com/problems/range-product-queries-of-powers/description/?envType=daily-question&envId=2025-08-11

struct Solution;

impl Solution {
    pub fn product_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        println!("n: {}, queries: {:?}", n, &queries);
        const M: i32 = 10_i32.pow(9) + 7;

        let powers = {
            let mut tmp: Vec<i32> = vec![];
            let mut m = n;
            let mut p = 0;

            while m > 0 {
                if m & 1 == 1 {
                    tmp.push(p);
                }
                m = m >> 1;
                p += 1;
            }

            tmp
        };

        println!("powers: {:?}", &powers);
        println!(
            "powers: {:?}",
            &powers
                .iter()
                .map(|&p| 2_i32.pow(p as u32))
                .collect::<Vec<i32>>()
        );

        let multiply = |r: i32, p: i32| {
            let mut r = r;
            let mut p = p;
            while p > 0 {
                r = (r * 2) % M;
                p -= 1;
            }
            r
        };

        let mut answers: Vec<i32> = vec![];

        for range in queries {
            let value = (range[0]..=range[1])
                .into_iter()
                .map(|i| powers[i as usize])
                .fold(1_i32, |a, b| multiply(a, b));
            answers.push(value);
        }

        answers
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::range_product_queries_of_powers::Solution;

    #[test]
    fn test_product_queries_1() {
        let n = 15;
        let queries = [[0, 1], [2, 2], [0, 3]]
            .iter()
            .map(|&l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(vec![2, 4, 64], Solution::product_queries(n, queries));
    }

    #[test]
    fn test_product_queries_2() {
        let n = 2;
        let queries = [[0, 0]]
            .iter()
            .map(|&l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(vec![2], Solution::product_queries(n, queries));
    }

    #[test]
    fn test_product_queries_3() {
        let n = 13;
        let queries = [[1, 2], [1, 1]]
            .iter()
            .map(|&l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(vec![32, 4], Solution::product_queries(n, queries));
    }
}
