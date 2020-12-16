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

    let mut program = Program::default();
    for instruction in &instructions {
        program.apply_part_2(*instruction);
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
    floaters: u64,
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
        let mut floaters = 0;
        for ch in s.chars() {
            offset -= 1;
            match ch {
                'X' => {
                    floaters |= 1 << offset;
                }
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
        Ok(Mask {
            and_mask,
            or_mask,
            floaters,
        })
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

    fn apply_address(&self, address: u64) -> Vec<u64> {
        let base_address = (address | self.or_mask) & !self.floaters;
        let ones = self.floaters.count_ones();
        let num_options = 1 << ones;

        let mut floaters: Vec<u64> = Vec::with_capacity(num_options);
        for option in 0..num_options {
            let value = self.floater_option_to_value(ones, option as u64);
            floaters.push(value | base_address);
        }

        floaters
    }

    fn floater_option_to_value(&self, ones: u32, option: u64) -> u64 {
        let mut local_floaters = self.floaters;
        let mut result = 0;
        for i in 0..ones {
            let option_value = (option >> i) & 0x1;

            for j in 0..36 {
                if (local_floaters & (1 << j)) > 0 {
                    // j is the index of a 1!
                    result |= option_value << j;
                    local_floaters &= !(1 << j);
                    break;
                }
            }
        }
        result
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

    fn apply_part_2(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Mask(mask) => {
                self.mask = mask;
            }
            Instruction::Mem(address, value) => {
                for address in self.mask.apply_address(address as u64) {
                    self.write_part_2(address as usize, value);
                }
            }
        }
    }

    fn write(&mut self, address: usize, value: u64) {
        self.address_space.insert(address, self.mask.apply(value));
    }

    fn write_part_2(&mut self, address: usize, value: u64) {
        self.address_space.insert(address, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floaters_1() {
        let mask: Mask = "000000000000000000000000000000X1001X".parse().unwrap();
        let addresses = mask.apply_address(42);
        assert_eq!(addresses, vec![26, 27, 58, 59]);
    }

    #[test]
    fn floaters_2() {
        let mask: Mask = "00000000000000000000000000000000X0XX".parse().unwrap();
        let addresses = mask.apply_address(26);
        assert_eq!(addresses, vec![16, 17, 18, 19, 24, 25, 26, 27]);
    }
}
