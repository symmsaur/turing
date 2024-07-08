struct Rule {
    new_state: u8,
    write_tape: u8,
    move_tape: i8,
}

pub struct Machine {
    rules: Vec<Rule>,
    tape: Vec<u8>,
    state: u8,
    offset: usize,
}

fn parse_rule(rule_string: &str) -> Rule {
    let bytes = rule_string.as_bytes();
    Rule {
        new_state: bytes[2] - b'A',
        write_tape: bytes[0] - b'0',
        move_tape: match bytes[1] {
            b'R' => 1,
            b'L' => -1,
            _ => panic!("Unexpected string!"),
        },
    }
}

impl Machine {
    pub fn new(machine_string: &str) -> Machine {
        // Example string:
        // 1RB1LC_1RC1RB_ ...
        let mut rules = Vec::new();
        let parts = machine_string.split('_');
        for part in parts {
            rules.push(parse_rule(&part[0..3]));
            rules.push(parse_rule(&part[3..6]));
        }
        Machine {
            rules,
            tape: vec![0; 1024],
            state: 0,
            offset: 512,
        }
    }

    pub fn step(&mut self) -> bool {
        let rule_index = self.state as usize * 2 + self.tape[self.offset] as usize;
        let rule = &self.rules[rule_index];
        self.state = rule.new_state;
        self.tape[self.offset] = rule.write_tape;
        if self.offset == 0 && rule.move_tape == -1 {
            self.offset = self.tape.len();
            let mut new_tape = vec![0u8; self.tape.len()];
            new_tape.extend(&self.tape);
            self.tape = new_tape;
        }
        if self.offset == self.tape.len() - 1 && rule.move_tape == 1 {
            self.tape.extend(vec![0; self.tape.len()]);
        }
        self.offset = (self.offset as isize + rule.move_tape as isize) as usize;
        self.state != b'Z' - b'A'
    }
}
