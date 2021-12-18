#[cfg(test)]

mod tests {
    use crate::{add, calc_magnitude, convert, convert_back, explode, reduce};

    #[test]
    fn test_explode1() {
        let s = "[[[[[9,8],1],2],3],4]";
        let result = convert_back(&explode(convert(s)));
        assert_eq!(&result, "[[[[0,9],2],3],4]")
    }

    #[test]
    fn test_explode2() {
        let s = "[7,[6,[5,[4,[3,2]]]]]";
        let result = convert_back(&explode(convert(s)));
        assert_eq!(&result, "[7,[6,[5,[7,0]]]]")
    }

    #[test]
    fn test_explode3() {
        let s = "[[6,[5,[4,[3,2]]]],1]";
        let result = convert_back(&explode(convert(s)));
        assert_eq!(&result, "[[6,[5,[7,0]]],3]")
    }

    #[test]
    fn test_explode4() {
        let s = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]";
        let result = convert_back(&explode(convert(s)));
        assert_eq!(&result, "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")
    }

    #[test]
    fn test_explode5() {
        let s = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]";
        let result = convert_back(&explode(convert(s)));
        assert_eq!(&result, "[[3,[2,[8,0]]],[9,[5,[7,0]]]]")
    }

    #[test]
    fn test_reduce1() {
        let s = "[[[[4,3],4],4],[7,[[8,4],9]]]\n[1,1]";
        let input: Vec<Vec<i32>> = s.lines().map(|s| convert(s)).collect();
        let result = input[1..]
            .iter()
            .fold(input[0].clone(), |acc, s| reduce(add(&acc, &s)));
        assert_eq!("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", convert_back(&result));
    }

    #[test]
    fn test_reduce2() {
        let s = "[1,1]\n[2,2]\n[3,3]\n[4,4]";
        let input: Vec<Vec<i32>> = s.lines().map(|s| convert(s)).collect();
        let result = input[1..]
            .iter()
            .fold(input[0].clone(), |acc, s| reduce(add(&acc, &s)));
        assert_eq!("[[[[1,1],[2,2]],[3,3]],[4,4]]", convert_back(&result));
    }

    #[test]
    fn test_reduce3() {
        let s = "[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]";
        let input: Vec<Vec<i32>> = s.lines().map(|s| convert(s)).collect();
        let result = input[1..]
            .iter()
            .fold(input[0].clone(), |acc, s| reduce(add(&acc, &s)));
        assert_eq!("[[[[3,0],[5,3]],[4,4]],[5,5]]", convert_back(&result));
    }

    #[test]
    fn test_reduce4() {
        let s = "[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]\n[6,6]";
        let input: Vec<Vec<i32>> = s.lines().map(|s| convert(s)).collect();
        let result = input[1..]
            .iter()
            .fold(input[0].clone(), |acc, s| reduce(add(&acc, &s)));
        assert_eq!("[[[[5,0],[7,4]],[5,5]],[6,6]]", convert_back(&result));
    }

    #[test]
    fn test_reduce5() {
        let s = " [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
        [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]";
        let input: Vec<Vec<i32>> = s.lines().map(|s| convert(s.trim())).collect();
        let result = input[1..]
            .iter()
            .fold(input[0].clone(), |acc, s| reduce(add(&acc, &s)));
        assert_eq!(
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
            convert_back(&result)
        );
    }

    #[test]
    fn test_reduce6() {
        let s = "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]
        [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]";
        let input: Vec<Vec<i32>> = s.lines().map(|s| convert(s.trim())).collect();
        let result = input[1..]
            .iter()
            .fold(input[0].clone(), |acc, s| reduce(add(&acc, &s)));
        assert_eq!(
            "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]",
            convert_back(&result)
        );
    }

    #[test]
    fn test_reduce_large() {
        let s = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
        [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
        [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
        [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
        [7,[5,[[3,8],[1,4]]]]
        [[2,[2,2]],[8,[8,1]]]
        [2,9]
        [1,[[[9,3],9],[[9,0],[0,7]]]]
        [[[5,[7,4]],7],1]
        [[[[4,2],2],6],[8,7]]";
        let input: Vec<Vec<i32>> = s.lines().map(|s| convert(s.trim())).collect();
        let result = input[1..]
            .iter()
            .fold(input[0].clone(), |acc, s| reduce(add(&acc, &s)));
        assert_eq!(
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
            convert_back(&result)
        );
    }

    #[test]
    fn test_calc_magnitude1() {
        let s = "[[1,2],[[3,4],5]]";
        let input: Vec<Vec<i32>> = s.lines().map(|s| convert(s.trim())).collect();
        let result = input[1..]
            .iter()
            .fold(input[0].clone(), |acc, s| reduce(add(&acc, &s)));
        assert_eq!(143, calc_magnitude(&result));
    }

    #[test]
    fn test_calc_magnitude2() {
        let s = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";
        let input: Vec<Vec<i32>> = s.lines().map(|s| convert(s.trim())).collect();
        let result = input[1..]
            .iter()
            .fold(input[0].clone(), |acc, s| reduce(add(&acc, &s)));
        assert_eq!(1384, calc_magnitude(&result));
    }

    #[test]
    fn test_calc_magnitude3() {
        let s = "[[[[1,1],[2,2]],[3,3]],[4,4]]";
        let input: Vec<Vec<i32>> = s.lines().map(|s| convert(s.trim())).collect();
        let result = input[1..]
            .iter()
            .fold(input[0].clone(), |acc, s| reduce(add(&acc, &s)));
        assert_eq!(445, calc_magnitude(&result));
    }

    #[test]
    fn test_calc_magnitude4() {
        let s = "[[[[3,0],[5,3]],[4,4]],[5,5]]";
        let input: Vec<Vec<i32>> = s.lines().map(|s| convert(s.trim())).collect();
        let result = input[1..]
            .iter()
            .fold(input[0].clone(), |acc, s| reduce(add(&acc, &s)));
        assert_eq!(791, calc_magnitude(&result));
    }

    #[test]
    fn test_calc_magnitude5() {
        let s = "[[[[5,0],[7,4]],[5,5]],[6,6]]";
        let input: Vec<Vec<i32>> = s.lines().map(|s| convert(s.trim())).collect();
        let result = input[1..]
            .iter()
            .fold(input[0].clone(), |acc, s| reduce(add(&acc, &s)));
        assert_eq!(1137, calc_magnitude(&result));
    }

    #[test]
    fn test_calc_magnitude6() {
        let s = "[[[[5,0],[7,4]],[5,5]],[6,6]]";
        let input: Vec<Vec<i32>> = s.lines().map(|s| convert(s.trim())).collect();
        let result = input[1..]
            .iter()
            .fold(input[0].clone(), |acc, s| reduce(add(&acc, &s)));
        assert_eq!(1137, calc_magnitude(&result));
    }

    #[test]
    fn test_calc_magnitude7() {
        let s = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";
        let input: Vec<Vec<i32>> = s.lines().map(|s| convert(s.trim())).collect();
        let result = input[1..]
            .iter()
            .fold(input[0].clone(), |acc, s| reduce(add(&acc, &s)));
        assert_eq!(3488, calc_magnitude(&result));
    }

    #[test]
    fn test_calc_magnitude8() {
        let s = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        let input: Vec<Vec<i32>> = s.lines().map(|s| convert(s.trim())).collect();
        let result = input[1..]
            .iter()
            .fold(input[0].clone(), |acc, s| reduce(add(&acc, &s)));
        assert_eq!(4140, calc_magnitude(&result));
    }
}
