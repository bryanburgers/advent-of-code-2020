use std::collections::HashMap;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let instructions = input
        .trim()
        .split('\n')
        .map(|line| line.parse())
        .collect::<Result<Vec<Instruction>, _>>()
        .unwrap();

    let mut program = Program::default();
    for instruction in &instructions {
        program.apply(*instruction);
    }
    let mut sum = 0;
    for (_key, value) in &program.address_space {
        sum += value;
    }
    println!("{}", sum);
}

#[derive(Copy, Clone, Default)]
struct Mask {
    and_mask: u64,
    or_mask: u64,
}

impl std::str::FromStr for Mask {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 36 {
            return Err("Not exactly 36 characters long");
        }
        let mut offset = 36;
        let mut or_mask = 0;
        let mut and_mask = (1 << (offset + 1)) - 1;
        for ch in s.chars() {
            offset -= 1;
            match ch {
                'X' => {}
                '1' => {
                    or_mask |= 1 << offset;
                }
                '0' => {
                    and_mask &= !(1 << offset);
                }
                _ => {
                    return Err("Invalid character");
                }
            }
        }
        Ok(Mask { and_mask, or_mask })
    }
}

impl std::fmt::Debug for Mask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Mask")
            .field("and_mask", &format!("{:036b}", self.and_mask))
            .field("or_mask", &format!("{:036b}", self.or_mask))
            .finish()
    }
}

impl Mask {
    fn apply(&self, value: u64) -> u64 {
        (value & self.and_mask) | self.or_mask
    }
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Mask(Mask),
    Mem(usize, u64),
}

impl std::str::FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instr = match &s[0..3] {
            "mas" => {
                let mask = (&s[7..]).parse()?;
                Instruction::Mask(mask)
            }
            "mem" => {
                let closing_bracket = s.find(']').ok_or("Failed to find closing bracket")?;
                let address = &s[4..closing_bracket];
                let address = address.parse().map_err(|_| "Failed to parse address")?;
                let equals = s.find('=').ok_or("Failed to find equals")?;
                let value = &s[(equals + 2)..];
                let value = value.parse().map_err(|_| "Failed to parse value")?;
                Instruction::Mem(address, value)
            }
            _ => {
                return Err("Unknown command");
            }
        };
        Ok(instr)
    }
}

#[derive(Default, Debug)]
struct Program {
    mask: Mask,
    address_space: HashMap<usize, u64>,
}

impl Program {
    fn apply(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Mask(mask) => {
                self.mask = mask;
            }
            Instruction::Mem(address, value) => {
                self.write(address, value);
            }
        }
    }

    fn write(&mut self, address: usize, value: u64) {
        self.address_space.insert(address, self.mask.apply(value));
    }
}
