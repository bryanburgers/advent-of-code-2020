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

    let valid = passports
        .iter()
        .filter(|passport| passport.is_valid_2())
        .count();
    println!("{}", valid);

    Ok(())
}

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
struct Passport {
    pub birth_year: Option<Year>,
    pub issue_year: Option<Year>,
    pub expiration_year: Option<Year>,
    pub height: Option<Height>,
    pub hair_color: Option<HairColor>,
    pub eye_color: Option<EyeColor>,
    pub passport_id: Option<PassportId>,
    pub country_id: Option<CountryId>,
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

    fn is_valid_2(&self) -> bool {
        self.birth_year
            .as_ref()
            .map(|y| y.is_valid(1920, 2002))
            .unwrap_or(false)
            && self
                .issue_year
                .as_ref()
                .map(|y| y.is_valid(2010, 2020))
                .unwrap_or(false)
            && self
                .expiration_year
                .as_ref()
                .map(|y| y.is_valid(2020, 2030))
                .unwrap_or(false)
            && self.height.as_ref().map(|h| h.is_valid()).unwrap_or(false)
            && self
                .hair_color
                .as_ref()
                .map(|h| h.is_valid())
                .unwrap_or(false)
            && self
                .eye_color
                .as_ref()
                .map(|e| e.is_valid())
                .unwrap_or(false)
            && self
                .passport_id
                .as_ref()
                .map(|p| p.is_valid())
                .unwrap_or(false)
    }
}

#[derive(thiserror::Error, Debug, Eq, PartialEq)]
enum PassportParseError {
    #[error("Invalid key: {0}")]
    InvalidKey(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Year(String);

impl Year {
    fn is_valid(&self, min: usize, max: usize) -> bool {
        if let Ok(value) = self.0.parse::<usize>() {
            min <= value && value <= max
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Height(String);

impl Height {
    fn is_valid(&self) -> bool {
        let len = self.0.len();
        if len < 4 || len > 5 {
            return false;
        }

        match &self.0[(len - 2)..len] {
            "in" => len == 4 && "59in" <= self.0.as_str() && self.0.as_str() <= "76in",
            "cm" => len == 5 && "150cm" <= self.0.as_str() && self.0.as_str() <= "193cm",
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct HairColor(String);

impl HairColor {
    fn is_valid(&self) -> bool {
        let r = regex::Regex::new("^#[0-9a-f]{6}$").unwrap();
        r.is_match(&self.0)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct EyeColor(String);

impl EyeColor {
    fn is_valid(&self) -> bool {
        match self.0.as_str() {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct PassportId(String);

impl PassportId {
    fn is_valid(&self) -> bool {
        let r = regex::Regex::new("^[0-9]{9}$").unwrap();
        r.is_match(&self.0)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct CountryId(String);

impl std::str::FromStr for Passport {
    type Err = PassportParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace();

        let mut passport = Passport::default();

        for part in parts {
            let key = &part[0..3];
            let value = &part[4..];

            match key {
                "byr" => passport.birth_year = Some(Year(value.to_string())),
                "iyr" => passport.issue_year = Some(Year(value.to_string())),
                "eyr" => passport.expiration_year = Some(Year(value.to_string())),
                "hgt" => passport.height = Some(Height(value.to_string())),
                "hcl" => passport.hair_color = Some(HairColor(value.to_string())),
                "ecl" => passport.eye_color = Some(EyeColor(value.to_string())),
                "pid" => passport.passport_id = Some(PassportId(value.to_string())),
                "cid" => passport.country_id = Some(CountryId(value.to_string())),
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
                eye_color: Some(EyeColor("gry".to_string())),
                passport_id: Some(PassportId("860033327".to_string())),
                expiration_year: Some(Year("2020".to_string())),
                hair_color: Some(HairColor("#fffffd".to_string())),
                birth_year: Some(Year("1937".to_string())),
                issue_year: Some(Year("2017".to_string())),
                country_id: Some(CountryId("147".to_string())),
                height: Some(Height("183cm".to_string())),
            })
        );
    }

    #[test]
    fn valid_year() {
        assert_eq!(Year("2002".to_string()).is_valid(1920, 2002), true);
        assert_eq!(Year("2003".to_string()).is_valid(1920, 2002), false);
        assert_eq!(Year("02".to_string()).is_valid(1920, 2002), false);
    }

    #[test]
    fn valid_height() {
        assert_eq!(Height("60in".to_string()).is_valid(), true);
        assert_eq!(Height("190cm".to_string()).is_valid(), true);
        assert_eq!(Height("190in".to_string()).is_valid(), false);
        assert_eq!(Height("190".to_string()).is_valid(), false);
    }

    #[test]
    fn valid_hari_color() {
        assert_eq!(HairColor("#123abc".to_string()).is_valid(), true);
        assert_eq!(HairColor("#123abz".to_string()).is_valid(), false);
        assert_eq!(HairColor("123abc".to_string()).is_valid(), false);
    }

    #[test]
    fn valid_passport() {
        assert_eq!(PassportId("000000001".to_string()).is_valid(), true);
        assert_eq!(PassportId("0123456789".to_string()).is_valid(), false);
    }
}
