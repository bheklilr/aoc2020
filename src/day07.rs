use crate::prelude::*;

pub fn day07() -> Result<Answer<usize>, String> {
    let mut answer = Answer::new();
    let rules = parse_bag_rules(&read_file("inputs/day07.txt").ok_or("Failed to read file")?)?;
    answer.part1(
        rules
            .keys()
            .filter(|color| can_contain_shiny_gold(&rules, color).unwrap_or(false))
            .count(),
    );
    answer.part2(bag_count(&rules, "shiny gold").unwrap() - 1);
    Ok(answer)
}

struct BagRule {
    pub count: usize,
    pub dependency: String,
}
type BagRules = HashMap<String, Vec<BagRule>>;

fn bag_count(rules: &BagRules, color: &str) -> Option<usize> {
    let contents = rules.get(color)?;
    let count: usize = contents
        .iter()
        .filter_map(|rule| bag_count(rules, &rule.dependency).map(|x| rule.count * x))
        .sum();
    Some(1 + count)
}

fn can_contain_shiny_gold(rules: &BagRules, color: &str) -> Option<bool> {
    let contents = rules.get(color)?;
    Some(
        contents.iter().map(|rule| &rule.dependency).any(|dep| {
            *dep == *"shiny gold" || can_contain_shiny_gold(rules, &dep).unwrap_or(false)
        }),
    )
}

fn parse_bag_rules(file: &str) -> Result<BagRules, String> {
    let mut parsed = HashMap::new();
    for line in file.lines() {
        let mut contains = Vec::new();
        let mut split = line.split(" bags contain ");
        let color = split.next().ok_or("No initial color")?;
        let rules = split.next().ok_or("No dependency colors")?;
        for rule in rules.split(',') {
            let rule = rule
                .trim()
                .replace("bags", "")
                .replace("bag", "")
                .replace('.', "");
            let rule = rule.trim();
            if !(rule.starts_with("no other")) {
                let count = rule[..1]
                    .parse()
                    .map_err(|_| format!("Failed to parse number: {:?}", rule))?;
                let dependency = rule[1..].trim().to_string();
                contains.push(BagRule { count, dependency });
            }
        }
        parsed.insert(color.to_string(), contains);
    }
    Ok(parsed)
}