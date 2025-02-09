use ev3dev_lang_rust::{motors::{LargeMotor, MotorPort}, Ev3Result};

use crate::{chars::{Char, Instruction}, writer::Writer};

// Motors are commented, because for testing is compiled for normal cpus, not arm robot
pub struct Printer {
    xa_motor: LargeMotor,
    // ya_motor: LargeMotor,
    // za_motor: LargeMotor,

    drawing: bool,

    pen_position: PenPosition, // Position is very relative
    scale: u32,
    writer: Writer
}

impl Printer {
    pub fn init() -> Ev3Result<Self> {
        let xa_motor = LargeMotor::get(MotorPort::OutA)?;
        // let ya_motor = LargeMotor::get(MotorPort::OutB)?;
        // let za_motor = LargeMotor::get(MotorPort::OutC)?;
        
        xa_motor.set_speed_sp(100);
        xa_motor.run_to_abs_pos(Some(0));

        let drawing = false;

        let pen_position = PenPosition::new(0, 0);
        let scale = 10;

        let writer = Writer::init("Hello, world! and ahoj".into());

        Ok(Self {
            xa_motor,
            // ya_motor,
            // za_motor,

            drawing,
            pen_position,
            scale,

            writer
        })
    }

    pub fn set_drawing(&mut self, drawing: bool) {
        if self.drawing == drawing {
            return
        }

        if drawing {
            println!("zam move down");
        } else {
            println!("zam move up");
        }
    }

    pub fn start_drawing(&mut self) {
        for character in self.writer.to_write.clone() {
            self.draw_character(character);
        }
    }

    pub fn draw_character(&mut self, character: Char) {
        let padding_left = self.writer.written.len() * 10;
        
        self.set_drawing(false);
        
        for instruction in character.clone().instructions {
            match instruction {
                Instruction::PenUp => {
                    self.set_drawing(false);
                }
                Instruction::PenDown => {
                    self.set_drawing(true);
                }
                Instruction::MoveBy { x, y } => {
                    self.pen_position.x += x * self.scale;
                    self.pen_position.y += y * self.scale;
                    println!("Moved by x: {x}, y: {y}, on char {}", character.char);
                }
                Instruction::MoveTo { x, y } => {
                    let x_pos = (x + padding_left as u32) * self.scale;

                    self.xa_motor.run_to_abs_pos(Some(self.pen_position.x as i32));

                    #[cfg(target_os = "linux")]
                    self.xa_motor.wait_until_not_moving(None);

                    self.pen_position.x = x_pos;
                    self.pen_position.y = y * self.scale;

                    println!("Moved to x: {}, y: {}, on char {}", self.pen_position.x, self.pen_position.y, character.char);
                }
            }
        }

        self.set_drawing(false);
        self.writer.written.push(character);
        self.writer.to_write.remove(0);
        println!("----------------")
    }
}

pub struct PenPosition {
    x: u32,
    y: u32
}

impl PenPosition {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}
