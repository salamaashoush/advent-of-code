#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        left.push(parts.next().unwrap().parse::<usize>().unwrap());
        right.push(parts.next().unwrap().parse::<usize>().unwrap());
    }
    let sum = left
        .iter()
        .map(|x| x * right.iter().filter(|y| &x == y).count())
        .sum::<usize>();

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
