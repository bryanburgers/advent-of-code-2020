use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    let entries: Vec<_> = lines
        .map(|line| line.unwrap().parse::<PasswordEntry>().unwrap())
        .collect();

    let num_valid = entries.iter().filter(|entry| entry.is_valid()).count();
    println!("{}", num_valid);

    let num_valid = entries.iter().filter(|entry| entry.is_valid_2()).count();
    println!("{}", num_valid);
}

#[derive(Debug)]
struct PasswordEntry {
    min: usize,
    max: usize,
    ch: char,
    password: String,
}

impl PasswordEntry {
    fn is_valid(&self) -> bool {
        let count = self.password.chars().filter(|c| *c == self.ch).count();
        self.min <= count && count <= self.max
    }

    fn is_valid_2(&self) -> bool {
        let chars: Vec<char> = self.password.chars().collect();
        let c1 = chars.get(self.min - 1);
        let c2 = chars.get(self.max - 1);
        (c1 == Some(&self.ch)) ^ (c2 == Some(&self.ch))
    }
}

#[derive(Debug, thiserror::Error, Clone, Copy)]
enum PasswordEntryParseError {
    #[error("Missing Range")]
    MissingRange,
    #[error("Missing Character")]
    MissingCharacter,
    #[error("Missing Password")]
    MissingPassword,
    #[error("Invalid Range")]
    InvalidRange,
    #[error("Invalid Character")]
    InvalidCharacter,
}

impl std::str::FromStr for PasswordEntry {
    type Err = PasswordEntryParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(3, ' ');
        let range = parts.next().ok_or(PasswordEntryParseError::MissingRange)?;
        let ch = parts
            .next()
            .ok_or(PasswordEntryParseError::MissingCharacter)?;
        let password = parts
            .next()
            .ok_or(PasswordEntryParseError::MissingPassword)?
            .to_string();

        let mut range_parts = range.splitn(2, '-');
        let min = range_parts
            .next()
            .ok_or(PasswordEntryParseError::InvalidRange)?;
        let min = min
            .parse()
            .map_err(|_| PasswordEntryParseError::InvalidRange)?;
        let max = range_parts
            .next()
            .ok_or(PasswordEntryParseError::InvalidRange)?;
        let max = max
            .parse()
            .map_err(|_| PasswordEntryParseError::InvalidRange)?;

        let ch = ch
            .chars()
            .next()
            .ok_or(PasswordEntryParseError::InvalidCharacter)?;

        Ok(PasswordEntry {
            min,
            max,
            ch,
            password,
        })
    }
}
