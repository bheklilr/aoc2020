use crate::prelude::*;

pub fn day02() -> Option<Answer<i32>> {
    let mut answer = Answer::new();
    let values: &Vec<PasswordPolicy> = &parse_passwords("inputs/day02.txt")?;
    answer.part1(
        values
            .iter()
            .filter(|pp| pp.is_valid_sled_rental_pw())
            .count() as i32,
    );

    answer.part2(values.iter().filter(|pp| pp.is_valid_toboggan_pw()).count() as i32);

    Some(answer)
}

#[derive(Debug)]
struct PasswordPolicy {
    pub min: i32,
    pub max: i32,
    pub letter: char,
    pub password: String,
}

impl PasswordPolicy {
    pub fn is_valid_sled_rental_pw(&self) -> bool {
        let count = self.password.chars().filter(|&c| c == self.letter).count() as i32;
        (self.min <= count) && (count <= self.max)
    }

    pub fn is_valid_toboggan_pw(&self) -> bool {
        let first = self
            .password
            .chars()
            .nth(self.min as usize - 1)
            .unwrap_or('.');
        let second = self
            .password
            .chars()
            .nth(self.max as usize - 1)
            .unwrap_or('.');
        println!("{:?}: {} {}", self, first, second);
        (first == self.letter) ^ (second == self.letter)
    }
}

impl FromStr for PasswordPolicy {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PASSWORD_POLICY_RE: Regex = Regex::new(r"(.+)-(.+) (.): (.+)").unwrap();
        }
        let captures = PASSWORD_POLICY_RE
            .captures(input)
            .ok_or(format!("Invalid password policy: {}", &input))?;
        let min = captures[1]
            .parse()
            .map_err(|_| format!("Failed to parse min: {}", &captures[0]))?;
        let max = captures[2].parse().or(Err("Failed to parse max"))?;
        let letter = captures[3].chars().next().ok_or("Failed to parse letter")?;
        let password = (&captures[4]).to_string();
        Ok(PasswordPolicy {
            min,
            max,
            letter,
            password,
        })
    }
}

fn parse_passwords(filename: &str) -> Option<Vec<PasswordPolicy>> {
    Some(
        read_file(filename)?
            .lines()
            .filter_map(|line| line.parse::<PasswordPolicy>().ok())
            .collect(),
    )
}