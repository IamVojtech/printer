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
        let mut chars = HashMap::from([
            (
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
            ),
            (
                'B',
                Char {
                    char: 'B',
                    instructions: vec![
                        Instruction::MoveTo { x: 0, y: 8 },
                        Instruction::PenDown,
                        Instruction::MoveTo { x: 8, y: 8 },
                        Instruction::MoveTo { x: 10, y: 10 },
                        Instruction::MoveTo { x: 10, y: 16 },
                        Instruction::MoveTo { x: 0, y: 16 },
                        Instruction::MoveTo { x: 0, y: 0 },
                        Instruction::MoveTo { x: 10, y: 0 },
                        Instruction::MoveTo { x: 10, y: 6 },
                        Instruction::MoveTo { x: 8, y: 8 }
                    ],
                    width: 10
                }
            ),
            (
                'C',
                Char {
                    char: 'C',
                    instructions: vec![
                        Instruction::MoveTo { x: 10, y: 14 },
                        Instruction::PenDown,
                        Instruction::MoveTo { x: 10, y: 16 },
                        Instruction::MoveTo { x: 0, y: 16 },
                        Instruction::MoveTo { x: 0, y: 0 },
                        Instruction::MoveTo { x: 10, y: 0 },
                        Instruction::MoveTo { x: 10, y: 2 },
                    ],
                    width: 10
                }
            ),
            (
                'D',
                Char {
                    char: 'D',
                    instructions: vec![
                        Instruction::MoveTo { x: 0, y: 16 },
                        Instruction::PenDown,
                        Instruction::MoveTo { x: 0, y: 0 },
                        Instruction::MoveTo { x: 8, y: 0 },
                        Instruction::MoveTo { x: 10, y: 2 },
                        Instruction::MoveTo { x: 10, y: 14 },
                        Instruction::MoveTo { x: 8, y: 16 },
                        Instruction::MoveTo { x: 0, y: 16 },
                    ],
                    width: 10
                }
            ),
            (
                'E',
                Char {
                    char: 'E',
                    instructions: vec![
                        Instruction::MoveTo { x: 8, y: 16 },
                        Instruction::PenDown,
                        Instruction::MoveTo { x: 0, y: 16 },
                        Instruction::MoveTo { x: 0, y: 0 },
                        Instruction::MoveTo { x: 8, y: 0 },
                        Instruction::PenUp,
                        Instruction::MoveTo { x: 0, y: 8 },
                        Instruction::PenDown,
                        Instruction::MoveTo { x: 6, y: 8 },
                    ],
                    width: 8
                }
            ),
            (
                'F',
                Char {
                    char: 'F',
                    instructions: vec![
                        Instruction::MoveTo { x: 0, y: 16 },
                        Instruction::PenDown,
                        Instruction::MoveTo { x: 0, y: 0 },
                        Instruction::MoveTo { x: 8, y: 0 },
                        Instruction::PenUp,
                        Instruction::MoveTo { x: 0, y: 8 },
                        Instruction::PenDown,
                        Instruction::MoveTo { x: 6, y: 8 },
                    ],
                    width: 8
                }
            ),
            (
                'G',
                Char {
                    char: 'G',
                    instructions: vec![
                        Instruction::MoveTo { x: 4, y: 8 },
                        Instruction::PenDown,
                        Instruction::MoveTo { x: 8, y: 8 },
                        Instruction::MoveTo { x: 8, y: 16 },
                        Instruction::PenUp,
                        Instruction::MoveTo { x: 0, y: 16 },
                        Instruction::MoveTo { x: 0, y: 0 },
                        Instruction::MoveTo { x: 8, y: 0 },
                    ],
                    width: 8
                }
            ),
            (
                'H',
                Char {
                    char: 'H',
                    instructions: vec![
                        Instruction::MoveTo { x: 0, y: 0 },
                        Instruction::PenDown,
                        Instruction::MoveTo { x: 0, y: 16 },
                        Instruction::PenUp,
                        Instruction::MoveTo { x: 0, y: 8 },
                        Instruction::PenDown,
                        Instruction::MoveTo { x: 8, y: 8 },
                        Instruction::PenUp,
                        Instruction::MoveTo { x: 8, y: 0 },
                        Instruction::PenDown,
                        Instruction::MoveTo { x: 8, y: 16 },
                    ],
                    width: 8
                }
            ),
            (
                'I',
                Char {
                    char: 'I',
                    instructions: vec![
                        Instruction::MoveTo { x: 8, y: 0 },
                        Instruction::PenDown,
                        Instruction::MoveTo { x: 8, y: 16 },
                    ],
                    width: 1
                }
            ),
            (
                'J',
                Char {
                    char: 'J',
                    instructions: vec![
                        Instruction::MoveTo { x: 0, y: 0 },
                        Instruction::PenDown,
                        Instruction::MoveTo { x: 8, y: 0 },
                        Instruction::MoveTo { x: 8, y: 13 },
                        Instruction::MoveTo { x: 5, y: 16 },
                        Instruction::MoveTo { x: 0, y: 16 },
                    ],
                    width: 8
                }
            ),
            (
                'K',
                Char {
                    char: 'K',
                    instructions: vec![
                        Instruction::MoveTo { x: 0, y: 0 },
                        Instruction::PenDown,
                        Instruction::MoveTo { x: 0, y: 16 },
                        Instruction::PenUp,
                        Instruction::MoveTo { x: 8, y: 0 },
                        Instruction::PenDown,
                        Instruction::MoveTo { x: 0, y: 8 },
                        Instruction::MoveTo { x: 8, y: 16 },
                    ],
                    width: 8
                }
            ),
            (
                'L',
                Char {
                    char: 'L',
                    instructions: vec![
                        Instruction::MoveTo { x: 0, y: 0 },
                        Instruction::PenDown,
                        Instruction::MoveTo { x: 0, y: 16 },
                        Instruction::MoveTo { x: 8, y: 16 },
                    ],
                    width: 8
                }
            ),
            (
                'M',
                Char {
                    char: 'M',
                    instructions: vec![
                        Instruction::MoveTo { x: 0, y: 16 },
                        Instruction::PenDown,
                        Instruction::MoveTo { x: 0, y: 0 },
                        Instruction::MoveTo { x: 6, y: 8 },
                        Instruction::MoveTo { x: 12, y: 0 },
                        Instruction::MoveTo { x: 12, y: 16 },
                    ],
                    width: 12
                }
            ),
            (
                'N',
                Char {
                    char: 'N',
                    instructions: vec![
                        Instruction::MoveTo { x: 0, y: 16 },
                        Instruction::PenDown,
                        Instruction::MoveTo { x: 0, y: 0 },
                        Instruction::MoveTo { x: 10, y: 16 },
                        Instruction::MoveTo { x: 10, y: 0 },
                    ],
                    width: 10
                }
            ),
            (
                'O',
                Char {
                    char: 'O',
                    instructions: vec![
                        Instruction::MoveTo { x: 0, y: 0 },
                        Instruction::PenDown,
                        Instruction::MoveTo { x: 10, y: 0 },
                        Instruction::MoveTo { x: 10, y: 16 },
                        Instruction::MoveTo { x: 0, y: 16 },
                    ],
                    width: 10
                }
            ),
            (
                'P',
                Char {
                    char: 'P',
                    instructions: vec![
                        Instruction::MoveTo { x: 0, y: 16 },
                        Instruction::PenDown,
                        Instruction::MoveTo { x: 0, y: 0 },
                        Instruction::MoveTo { x: 8, y: 0 },
                        Instruction::MoveTo { x: 8, y: 8 },
                        Instruction::MoveTo { x: 0, y: 8 },
                    ],
                    width: 8
                }
            ),
            (
                'Q',
                Char {
                    char: 'Q',
                    instructions: vec![
                        Instruction::MoveTo { x: 0, y: 0 },
                        Instruction::PenDown,
                        Instruction::MoveTo { x: 10, y: 0 },
                        Instruction::MoveTo { x: 10, y: 14 },
                        Instruction::MoveTo { x: 8, y: 16 },
                        Instruction::MoveTo { x: 0, y: 16 },
                        Instruction::MoveTo { x: 0, y: 0 },
                        Instruction::PenDown,
                        Instruction::MoveTo { x: 7, y: 13 },
                        Instruction::MoveTo { x: 11, y: 17 },
                    ],
                    width: 10
                }
            ),
            (
                'R',
                Char {
                    char: 'R',
                    instructions: vec![
                        Instruction::MoveTo { x: 0, y: 16 },
                        Instruction::PenDown,
                        Instruction::MoveTo { x: 0, y: 0 },
                        Instruction::MoveTo { x: 8, y: 0 },
                        Instruction::MoveTo { x: 8, y: 8 },
                        Instruction::MoveTo { x: 0, y: 8 },
                        Instruction::MoveTo { x: 8, y: 16 },
                    ],
                    width: 8
                }
            ),
            (
                'S',
                Char {
                    char: 'S',
                    instructions: vec![
                        Instruction::MoveTo { x: 0, y: 16 },
                        Instruction::PenDown,
                        Instruction::MoveTo { x: 10, y: 16 },
                        Instruction::MoveTo { x: 10, y: 10 },
                        Instruction::MoveTo { x: 8, y: 8 },
                        Instruction::MoveTo { x: 0, y: 8 },
                        Instruction::MoveTo { x: 0, y: 0 },
                        Instruction::MoveTo { x: 10, y: 0 },
                    ],
                    width: 10
                }
            ),
            (
                'T',
                Char {
                    char: 'T',
                    instructions: vec![
                        Instruction::MoveTo { x: 0, y: 0 },
                        Instruction::PenDown,
                        Instruction::MoveTo { x: 10, y: 0 },
                        Instruction::PenUp,
                        Instruction::MoveTo { x: 5, y: 0 },
                        Instruction::PenDown,
                        Instruction::MoveTo { x: 5, y: 16 },
                    ],
                    width: 10
                }
            ),
            (
                'U',
                Char {
                    char: 'U',
                    instructions: vec![
                        Instruction::MoveTo { x: 0, y: 0 },
                        Instruction::PenDown,
                        Instruction::MoveTo { x: 0, y: 14 },
                        Instruction::MoveTo { x: 2, y: 16 },
                        Instruction::MoveTo { x: 8, y: 16 },
                        Instruction::MoveTo { x: 10, y: 14 },
                        Instruction::MoveTo { x: 10, y: 0 },
                    ],
                    width: 10
                }
            ),
            (
                'V',
                Char {
                    char: 'V',
                    instructions: vec![
                        Instruction::MoveTo { x: 0, y: 0 },
                        Instruction::PenDown,
                        Instruction::MoveTo { x: 0, y: 8 },
                        Instruction::MoveTo { x: 5, y: 16 },
                        Instruction::MoveTo { x: 10, y: 8 },
                        Instruction::MoveTo { x: 10, y: 0 },
                    ],
                    width: 10
                }
            ),
            (
                'W',
                Char {
                    char: 'W',
                    instructions: vec![
                        Instruction::MoveTo { x: 0, y: 0 },
                        Instruction::PenDown,
                        Instruction::MoveTo { x: 0, y: 16 },
                        Instruction::MoveTo { x: 6, y: 8 },
                        Instruction::MoveTo { x: 12, y: 16 },
                        Instruction::MoveTo { x: 12, y: 0 },
                    ],
                    width: 12
                }
            ),
            (
                'X',
                Char {
                    char: 'X',
                    instructions: vec![
                        Instruction::MoveTo { x: 0, y: 0 },
                        Instruction::PenDown,
                        Instruction::MoveTo { x: 10, y: 16 },
                        Instruction::PenUp,
                        Instruction::MoveTo { x: 0, y: 16 },
                        Instruction::PenDown,
                        Instruction::MoveTo { x: 10, y: 0 },
                    ],
                    width: 10
                }
            ),
            (
                'Y',
                Char {
                    char: 'Y',
                    instructions: vec![
                        Instruction::MoveTo { x: 0, y: 0 },
                        Instruction::PenDown,
                        Instruction::MoveTo { x: 5, y: 8 },
                        Instruction::MoveTo { x: 10, y: 0 },
                        Instruction::PenUp,
                        Instruction::MoveTo { x: 5, y: 8 },
                        Instruction::PenDown,
                        Instruction::MoveTo { x: 5, y: 16 },
                    ],
                    width: 10
                }
            ),
            (
                'Z',
                Char {
                    char: 'Z',
                    instructions: vec![
                        Instruction::MoveTo { x: 0, y: 0 },
                        Instruction::PenDown,
                        Instruction::MoveTo { x: 10, y: 0 },
                        Instruction::MoveTo { x: 0, y: 16 },
                        Instruction::MoveTo { x: 10, y: 16 },
                    ],
                    width: 10
                }
            ),
            (
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
            )
        ]);

        // let top = 2;
        // let left = 3;

        Self {
            chars
        }
    }
}
