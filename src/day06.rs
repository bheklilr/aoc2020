use crate::prelude::*;

pub fn day06() -> R<Answer<usize>> {
    let mut answer = Answer::new();
    let values = parse_customs_answers("inputs/day06.txt")?;
    answer.part1(values.iter().map(|answers| answers.len()).sum());
    let values = parse_customs_answers2("inputs/day06.txt")?;
    answer.part2(
        values
            .iter()
            .map(|group| {
                let mut unique_answers: HashMap<&char, usize> = HashMap::new();
                for person in group {
                    for answer in person {
                        unique_answers
                            .entry(answer)
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                    }
                }
                let people = group.len();
                unique_answers
                    .iter()
                    .filter(|(_ans, count)| **count == people)
                    .count()
            })
            .sum(),
    );
    Ok(answer)
}

fn parse_customs_answers2(filename: &str) -> R<Vec<Vec<Vec<char>>>> {
    Ok(
        read_file(filename)?
            .replace('\r', "")
            .split("\n\n")
            .map(|chunk| chunk.lines().map(|line| line.chars().collect()).collect())
            .collect(),
    )
}

fn parse_customs_answers(filename: &str) -> R<Vec<HashSet<char>>> {
    Ok(
        read_file(filename)?
            .replace('\r', "")
            .split("\n\n")
            .map(|chunk| chunk.replace('\n', "").chars().collect())
            .collect(),
    )
}
