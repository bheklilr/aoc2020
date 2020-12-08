use crate::prelude::*;

pub fn day04() -> R<Answer<usize>> {
    use Field::*;
    let mut answer = Answer::new();
    let passports = parse_passports("inputs/day04.txt")?;
    answer.part1(
        passports
            .iter()
            .filter(|passport| {
                [BYR, IYR, EYR, HGT, HCL, ECL, PID]
                    .iter()
                    .all(|field| passport.contains_key(&field))
            })
            .count(),
    );
    answer.part2(
        passports
            .iter()
            .filter_map(|passport| {
                let byr = passport.get(&BYR)?.parse::<u32>().ok()?;
                let iyr = passport.get(&IYR)?.parse::<u32>().ok()?;
                let eyr = passport.get(&EYR)?.parse::<u32>().ok()?;
                let hgt = passport.get(&HGT)?;
                let hcl = passport.get(&HCL)?;
                let ecl = passport.get(&ECL)?;
                let pid = passport.get(&PID)?;
                if (byr < 1920) || (2002 < byr) {
                    println!("byr: {:?}", passport);
                    return None;
                }
                if (iyr < 2010) || (2020 < iyr) {
                    println!("iyr: {:?}", passport);
                    return None;
                }
                if (eyr < 2020) || (2030 < eyr) {
                    println!("eyr: {:?}", passport);
                    return None;
                }
                if hgt.ends_with("cm") {
                    let height = hgt.strip_suffix("cm")?.parse::<u32>().ok()?;
                    if (height < 150) || (193 < height) {
                        println!("hgt: {:?}", passport);
                        return None;
                    }
                } else if hgt.ends_with("in") {
                    let height = hgt.strip_suffix("in")?.parse::<u32>().ok()?;
                    if (height < 59) || (76 < height) {
                        println!("hgt: {:?}", passport);
                        return None;
                    }
                } else {
                    println!("hgt: {:?}", passport);
                    return None;
                }
                if !hcl.strip_prefix('#')?.chars().all(|c| c.is_digit(16)) {
                    println!("hcl: {:?}", passport);
                    return None;
                }
                match &ecl[..] {
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => Some(()),
                    _ => None,
                }?;
                if (pid.len() != 9) || (!pid.chars().all(char::is_numeric)) {
                    println!("pid: {:?}", passport);
                    return None;
                }
                Some(passport)
            })
            .count(),
    );
    Ok(answer)
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum Field {
    BYR,
    IYR,
    EYR,
    HGT,
    HCL,
    ECL,
    PID,
    CID,
}

type Passport = HashMap<Field, String>;

fn parse_passports(filename: &str) -> R<Vec<Passport>> {
    use Field::*;
    let text = read_file(filename)?;
    let chunks: Vec<&str> = text.split("\r\n\r\n").collect();
    Ok(chunks
        .iter()
        .map(|input| {
            input
                .split_whitespace()
                .filter_map(|field| {
                    let parts: Vec<&str> = field.split(':').collect();
                    let field = match parts[0] {
                        "byr" => Some(BYR),
                        "iyr" => Some(IYR),
                        "eyr" => Some(EYR),
                        "hgt" => Some(HGT),
                        "hcl" => Some(HCL),
                        "ecl" => Some(ECL),
                        "pid" => Some(PID),
                        "cid" => Some(CID),
                        _ => None,
                    }?;
                    Some((field, parts[1].trim().to_owned()))
                })
                .collect()
        })
        .collect())
}
