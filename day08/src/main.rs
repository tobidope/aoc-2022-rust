const INPUT: &str = include_str!("../input.txt");

fn main() {
    let trees = parse(INPUT);
    let result = visible_trees(&trees);
    println!("{result}");
    let result = best_scenic_score(&trees);
    println!("{result}");
}

fn parse(trees: &str) -> Vec<Vec<u32>> {
    trees
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn visible_trees(trees: &Vec<Vec<u32>>) -> usize {
    let mut visible = 4 * trees.len() - 4;

    for i in 1..trees.len() - 1 {
        let row = &trees[i];
        for j in 1..row.len() - 1 {
            let tree = row[j];
            if row[..j].iter().all(|&height| height < tree)
                || row[j + 1..].iter().all(|&height| height < tree)
                || (0..i).map(|x| trees[x][j]).all(|height| height < tree)
                || (i + 1..trees.len())
                    .map(|x| trees[x][j])
                    .all(|height| height < tree)
            {
                visible += 1;
            }
        }
    }
    visible
}

fn best_scenic_score(trees: &Vec<Vec<u32>>) -> usize {
    let mut max_scenic_score = 0;

    for i in 1..trees.len() - 1 {
        let row = &trees[i];
        for j in 1..row.len() - 1 {
            let tree = row[j];
            let left = row[..j]
                .iter()
                .rev()
                .position(|&height| height >= tree)
                .unwrap_or(j - 1)
                + 1;
            let right = row[j + 1..]
                .iter()
                .position(|&height| height >= tree)
                .unwrap_or(row.len() - j - 2)
                + 1;
            let top = (0..i)
                .map(|x| trees[x][j])
                .rev()
                .position(|height| height >= tree)
                .unwrap_or(i - 1)
                + 1;
            let down = (i + 1..trees.len())
                .map(|x| trees[x][j])
                .position(|height| height >= tree)
                .unwrap_or(trees.len() - i - 2)
                + 1;
            let scenic_score = left * right * top * down;
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }
    max_scenic_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = r#"30373
25512
65332
33549
35390"#;
        let result = parse(input);
        assert_eq!(5, result.len());
        assert!(result.iter().all(|l| l.len() == 5));
        assert_eq!(vec![3, 0, 3, 7, 3], result[0]);
    }

    #[test]
    fn test_visible_trees() {
        let result = trees();

        assert_eq!(21, visible_trees(&result));
    }

    fn trees() -> Vec<Vec<u32>> {
        let input = r#"30373
25512
65332
33549
35390"#;

        parse(input)
    }

    #[test]
    fn test_best_scenic_score() {
        assert_eq!(8, best_scenic_score(&trees()))
    }
}
