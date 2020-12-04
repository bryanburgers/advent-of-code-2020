use std::io::Read;

fn main() -> Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let passports = input
        .split("\n\n")
        .map(|block| block.parse())
        .collect::<Result<Vec<Passport>, _>>()?;

    let valid = passports
        .iter()
        .filter(|passport| passport.is_valid())
        .count();
    println!("{}", valid);

    Ok(())
}

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
struct Passport {
    pub birth_year: Option<String>,
    pub issue_year: Option<String>,
    pub expiration_year: Option<String>,
    pub height: Option<String>,
    pub hair_color: Option<String>,
    pub eye_color: Option<String>,
    pub passport_id: Option<String>,
    pub country_id: Option<String>,
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }
}

#[derive(thiserror::Error, Debug, Eq, PartialEq)]
enum PassportParseError {
    #[error("Invalid key: {0}")]
    InvalidKey(String),
}

impl std::str::FromStr for Passport {
    type Err = PassportParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace();

        let mut passport = Passport::default();

        for part in parts {
            let key = &part[0..3];
            let value = &part[4..];

            match key {
                "byr" => passport.birth_year = Some(value.to_string()),
                "iyr" => passport.issue_year = Some(value.to_string()),
                "eyr" => passport.expiration_year = Some(value.to_string()),
                "hgt" => passport.height = Some(value.to_string()),
                "hcl" => passport.hair_color = Some(value.to_string()),
                "ecl" => passport.eye_color = Some(value.to_string()),
                "pid" => passport.passport_id = Some(value.to_string()),
                "cid" => passport.country_id = Some(value.to_string()),
                _ => return Err(PassportParseError::InvalidKey(key.to_string())),
            }
        }

        Ok(passport)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse1() {
        let s = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm"#;

        assert_eq!(
            s.parse(),
            Ok(Passport {
                eye_color: Some("gry".to_string()),
                passport_id: Some("860033327".to_string()),
                expiration_year: Some("2020".to_string()),
                hair_color: Some("#fffffd".to_string()),
                birth_year: Some("1937".to_string()),
                issue_year: Some("2017".to_string()),
                country_id: Some("147".to_string()),
                height: Some("183cm".to_string()),
            })
        );
    }
}
