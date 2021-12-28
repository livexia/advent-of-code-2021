#[cfg(test)]

mod tests {
    use std::error::Error;

    type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

    use crate::Number;

    #[test]
    fn test_explode1() -> Result<()> {
        let mut s = Number::from_str("[[[[[9,8],1],2],3],4]")?;
        s.as_mut().unwrap().explode(None, None, false);
        let result = Number::from_str("[[[[0,9],2],3],4]")?;
        assert_eq!(result, s);
        Ok(())
    }

    #[test]
    fn test_explode2() -> Result<()> {
        let mut s = Number::from_str("[7,[6,[5,[4,[3,2]]]]]")?;
        s.as_mut().unwrap().explode(None, None, false);
        let result = Number::from_str("[7,[6,[5,[7,0]]]]")?;
        assert_eq!(result, s);
        Ok(())
    }

    #[test]
    fn test_explode3() -> Result<()> {
        let mut s = Number::from_str("[[6,[5,[4,[3,2]]]],1]")?;
        s.as_mut().unwrap().explode(None, None, false);
        let result = Number::from_str("[[6,[5,[7,0]]],3]")?;
        assert_eq!(result, s);
        Ok(())
    }

    #[test]
    fn test_explode4() -> Result<()> {
        let s = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]";
        let result = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]";
        // let result = "[[3,[2,[8,0]]],[9,[5,[7,0]]]]";

        let mut s = Number::from_str(s)?;
        s.as_mut().unwrap().explode(None, None, false);
        let result = Number::from_str(result)?;
        assert_eq!(result, s);
        Ok(())
    }

    #[test]
    fn test_explode5() -> Result<()> {
        let s = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]";
        let result = "[[3,[2,[8,0]]],[9,[5,[7,0]]]]";

        let mut s = Number::from_str(s)?;
        s.as_mut().unwrap().explode(None, None, false);
        let result = Number::from_str(result)?;
        assert_eq!(result, s);
        Ok(())
    }

    #[test]
    fn test_reduce1() -> Result<()> {
        let s = "[[[[4,3],4],4],[7,[[8,4],9]]]\n[1,1]";
        let expected = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";

        let expected = Number::from_str(expected)?;
        let mut input: Vec<Option<Box<Number>>> = s
            .lines()
            .map(|s| Number::from_str(s))
            .collect::<Result<Vec<Option<Box<Number>>>>>()?;

        let first = input.remove(0);
        let result = input.into_iter().fold(first, |acc, s| {
            let mut acc = Number::addition(acc, s);
            acc.as_mut().unwrap().reduce();
            acc
        });
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_reduce2() -> Result<()> {
        let s = "[1,1]\n[2,2]\n[3,3]\n[4,4]";
        let expected = "[[[[1,1],[2,2]],[3,3]],[4,4]]";

        let mut input: Vec<Option<Box<Number>>> = s
            .lines()
            .map(|s| Number::from_str(s))
            .collect::<Result<Vec<Option<Box<Number>>>>>()?;

        let first = input.remove(0);
        let result = input.into_iter().fold(first, |acc, s| {
            let mut acc = Number::addition(acc, s);
            acc.as_mut().unwrap().reduce();
            acc
        });
        let expected = Number::from_str(expected)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_reduce3() -> Result<()> {
        let s = "[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]";
        let expected = "[[[[3,0],[5,3]],[4,4]],[5,5]]";

        let expected = Number::from_str(expected)?;
        let mut input: Vec<Option<Box<Number>>> = s
            .lines()
            .map(|s| Number::from_str(s))
            .collect::<Result<Vec<Option<Box<Number>>>>>()?;

        let first = input.remove(0);
        let result = input.into_iter().fold(first, |acc, s| {
            let mut acc = Number::addition(acc, s);
            acc.as_mut().unwrap().reduce();
            acc
        });
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_reduce4() -> Result<()> {
        let s = "[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]\n[6,6]";
        let expected = "[[[[5,0],[7,4]],[5,5]],[6,6]]";

        let expected = Number::from_str(expected)?;
        let mut input: Vec<Option<Box<Number>>> = s
            .lines()
            .map(|s| Number::from_str(s))
            .collect::<Result<Vec<Option<Box<Number>>>>>()?;

        let first = input.remove(0);
        let result = input.into_iter().fold(first, |acc, s| {
            let mut acc = Number::addition(acc, s);
            acc.as_mut().unwrap().reduce();
            acc
        });
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_reduce5() -> Result<()> {
        let s = " [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
        [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]";
        let expected = "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]";

        let expected = Number::from_str(expected)?;
        let mut input: Vec<Option<Box<Number>>> = s
            .lines()
            .map(|s| Number::from_str(s))
            .collect::<Result<Vec<Option<Box<Number>>>>>()?;

        let first = input.remove(0);
        let result = input.into_iter().fold(first, |acc, s| {
            let mut acc = Number::addition(acc, s);
            acc.as_mut().unwrap().reduce();
            acc
        });

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_reduce6() -> Result<()> {
        let s = "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]
        [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]";
        let expected = "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]";

        let expected = Number::from_str(expected)?;
        let mut input: Vec<Option<Box<Number>>> = s
            .lines()
            .map(|s| Number::from_str(s))
            .collect::<Result<Vec<Option<Box<Number>>>>>()?;

        let first = input.remove(0);
        let result = input.into_iter().fold(first, |acc, s| {
            let mut acc = Number::addition(acc, s);
            acc.as_mut().unwrap().reduce();
            acc
        });

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_reduce_large() -> Result<()> {
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
        let expected = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";

        let expected = Number::from_str(expected)?;
        let mut input: Vec<Option<Box<Number>>> = s
            .lines()
            .map(|s| Number::from_str(s))
            .collect::<Result<Vec<Option<Box<Number>>>>>()?;

        let first = input.remove(0);
        let result = input.into_iter().fold(first, |acc, s| {
            let mut acc = Number::addition(acc, s);
            acc.as_mut().unwrap().reduce();
            acc
        });

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_calc_magnitude1() -> Result<()> {
        let s = "[[1,2],[[3,4],5]]";
        let mut input: Vec<Option<Box<Number>>> = s
            .lines()
            .map(|s| Number::from_str(s))
            .collect::<Result<Vec<Option<Box<Number>>>>>()?;

        let first = input.remove(0);
        let result = input.into_iter().fold(first, |acc, s| {
            let mut acc = Number::addition(acc, s);
            acc.as_mut().unwrap().reduce();
            acc
        });

        assert_eq!(143, result.as_ref().unwrap().calc_magnitude());
        Ok(())
    }

    #[test]
    fn test_calc_magnitude2() -> Result<()> {
        let s = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";
        let mut input: Vec<Option<Box<Number>>> = s
            .lines()
            .map(|s| Number::from_str(s))
            .collect::<Result<Vec<Option<Box<Number>>>>>()?;

        let first = input.remove(0);
        let result = input.into_iter().fold(first, |acc, s| {
            let mut acc = Number::addition(acc, s);
            acc.as_mut().unwrap().reduce();
            acc
        });

        assert_eq!(1384, result.as_ref().unwrap().calc_magnitude());
        Ok(())
    }

    #[test]
    fn test_calc_magnitude3() -> Result<()> {
        let s = "[[[[1,1],[2,2]],[3,3]],[4,4]]";
        let mut input: Vec<Option<Box<Number>>> = s
            .lines()
            .map(|s| Number::from_str(s))
            .collect::<Result<Vec<Option<Box<Number>>>>>()?;

        let first = input.remove(0);
        let result = input.into_iter().fold(first, |acc, s| {
            let mut acc = Number::addition(acc, s);
            acc.as_mut().unwrap().reduce();
            acc
        });

        assert_eq!(445, result.as_ref().unwrap().calc_magnitude());
        Ok(())
    }

    #[test]
    fn test_calc_magnitude4() -> Result<()> {
        let s = "[[[[3,0],[5,3]],[4,4]],[5,5]]";
        let mut input: Vec<Option<Box<Number>>> = s
            .lines()
            .map(|s| Number::from_str(s))
            .collect::<Result<Vec<Option<Box<Number>>>>>()?;

        let first = input.remove(0);
        let result = input.into_iter().fold(first, |acc, s| {
            let mut acc = Number::addition(acc, s);
            acc.as_mut().unwrap().reduce();
            acc
        });

        assert_eq!(791, result.as_ref().unwrap().calc_magnitude());
        Ok(())
    }

    #[test]
    fn test_calc_magnitude5() -> Result<()> {
        let s = "[[[[5,0],[7,4]],[5,5]],[6,6]]";
        let mut input: Vec<Option<Box<Number>>> = s
            .lines()
            .map(|s| Number::from_str(s))
            .collect::<Result<Vec<Option<Box<Number>>>>>()?;

        let first = input.remove(0);
        let result = input.into_iter().fold(first, |acc, s| {
            let mut acc = Number::addition(acc, s);
            acc.as_mut().unwrap().reduce();
            acc
        });

        assert_eq!(1137, result.as_ref().unwrap().calc_magnitude());
        Ok(())
    }

    #[test]
    fn test_calc_magnitude6() -> Result<()> {
        let s = "[[[[5,0],[7,4]],[5,5]],[6,6]]";
        let mut input: Vec<Option<Box<Number>>> = s
            .lines()
            .map(|s| Number::from_str(s))
            .collect::<Result<Vec<Option<Box<Number>>>>>()?;

        let first = input.remove(0);
        let result = input.into_iter().fold(first, |acc, s| {
            let mut acc = Number::addition(acc, s);
            acc.as_mut().unwrap().reduce();
            acc
        });

        assert_eq!(1137, result.as_ref().unwrap().calc_magnitude());
        Ok(())
    }

    #[test]
    fn test_calc_magnitude7() -> Result<()> {
        let s = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";
        let mut input: Vec<Option<Box<Number>>> = s
            .lines()
            .map(|s| Number::from_str(s))
            .collect::<Result<Vec<Option<Box<Number>>>>>()?;

        let first = input.remove(0);
        let result = input.into_iter().fold(first, |acc, s| {
            let mut acc = Number::addition(acc, s);
            acc.as_mut().unwrap().reduce();
            acc
        });

        assert_eq!(3488, result.as_ref().unwrap().calc_magnitude());
        Ok(())
    }

    #[test]
    fn test_calc_magnitude8() -> Result<()> {
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
        let mut input: Vec<Option<Box<Number>>> = s
            .lines()
            .map(|s| Number::from_str(s))
            .collect::<Result<Vec<Option<Box<Number>>>>>()?;

        let first = input.remove(0);
        let result = input.into_iter().fold(first, |acc, s| {
            let mut acc = Number::addition(acc, s);
            acc.as_mut().unwrap().reduce();
            acc
        });

        assert_eq!(4140, result.as_ref().unwrap().calc_magnitude());
        Ok(())
    }
}
