use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Char {
    pub char: char,
    pub instructions: Vec<Instruction>,
    pub width: u32
}

// Every char draw should start with pen up automaticaly
// Position is relative to character
#[derive(Debug, Clone)]
pub enum Instruction {
    MoveBy { x: u32, y: u32 },
    MoveTo { x: u32, y: u32 },
    PenDown,
    PenUp
}

pub struct CharMap {
    pub chars: HashMap<char, Char>
}

impl CharMap {
    pub fn init() -> Self {
        let mut chars = HashMap::new();
        let top = 2;
        let left = 3;

        chars.insert(
            'A',
            Char {
                char: 'A',
                instructions: vec![
                    Instruction::MoveTo { x: 0, y: 16 },
                    Instruction::PenDown,
                    Instruction::MoveTo { x: 0, y: 8 },
                    Instruction::MoveTo { x: 3, y: 0 },
                    Instruction::MoveTo { x: 7, y: 0 },
                    Instruction::MoveTo { x: 10, y: 8 },
                    Instruction::MoveTo { x: 10, y: 16 },
                    Instruction::PenUp,
                    Instruction::MoveTo { x: 10, y: 8 },
                    Instruction::PenDown,
                    Instruction::MoveTo { x: 0, y: 8 }
                ],
                width: 10
            }
        );

        chars.insert(
            '_', 
            Char {
                char: '_',
                instructions: vec![
                    Instruction::MoveTo { x: 0, y: 16 },
                    Instruction::PenDown,
                    Instruction::MoveTo { x: 10, y: 16 }
                ],
                width: 10
            }
        );

        Self {
            chars
        }
    }
}
