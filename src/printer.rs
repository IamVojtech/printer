use ev3dev_lang_rust::{motors::{LargeMotor, MotorPort}, Ev3Result};

use crate::{chars::{Char, Instruction}, writer::Writer};

pub struct Printer {
    xa_motor: LargeMotor,
    ya_motor: LargeMotor,
    za_motor: LargeMotor,

    drawing: bool,

    pen_position: PenPosition, // Position is very relative
    scale: usize,
    writer: Writer
}

impl Printer {
    pub fn init(input: String) -> Ev3Result<Self> {
        let xa_motor = LargeMotor::get(MotorPort::OutA)?;
        let ya_motor = LargeMotor::get(MotorPort::OutB)?;
        let za_motor = LargeMotor::get(MotorPort::OutC)?;

        xa_motor.set_speed_sp(100);
        ya_motor.set_speed_sp(100);
        za_motor.set_speed_sp(100);

        za_motor.run_to_abs_pos(Some(0));

        #[cfg(target_os = "linux")]
        za_motor.wait_until_not_moving(None);

        ya_motor.set_position(0);


        let drawing = false;

        let pen_position = PenPosition::new(0, 0);
        let scale = 4;

        let writer = Writer::init(input);

        Ok(Self {
            xa_motor,
            ya_motor,
            za_motor,

            drawing,
            pen_position,
            scale,

            writer
        })
    }

    pub fn set_drawing(&mut self, drawing: bool) {
        if self.drawing == drawing {
            return;
        }

        let target_pos = if drawing { 25 } else { 0 };

        self.za_motor.run_to_abs_pos(Some(target_pos));

        self.drawing = drawing;

        #[cfg(target_os = "linux")]
        self.za_motor.wait_until_not_moving(Some(std::time::Duration::from_millis(100)));
    }


    pub fn start_drawing(&mut self) {
        println!("[DEBUG] Drawing started");
        for character in self.writer.to_write.clone() {
            self.draw_character(character);
        }
        self.end_drawing();
    }
    
    pub fn end_drawing(&mut self) {
        self.xa_motor.run_to_abs_pos(Some(0));
        self.ya_motor.run_to_abs_pos(Some(0));
        self.za_motor.run_to_abs_pos(Some(0));
        println!("[DEBUG] ending drawing");

        // Paper is not comming out because the function for it is in different file
    }

    pub fn wrap_text(&mut self) {
        println!("[DEBUG] Line wrap");
        self.writer.current_line += 1;
    }

    pub fn draw_character(&mut self, character: Char) {
        let char_padding_x = self.writer.char_map.char_width - (character.width / 2);
        let char_padding_y = self.writer.char_map.char_height - (character.height / 2);
        let x_char_index = self.writer.written.len() % self.writer.wrap_after;
        let padding_left = x_char_index * self.writer.char_map.char_width;
        let padding_top = (self.writer.current_line + 1) * self.writer.char_map.char_height;
        
        self.set_drawing(false);
        
        for instruction in character.clone().instructions {
            match instruction {
                Instruction::PenUp => {
                    self.set_drawing(false);
                    println!("[DEBUG] Moved pen up");
                }
                Instruction::PenDown => {
                    self.set_drawing(true);
                    println!("[DEBUG] Moved pen down");
                }
                Instruction::MoveTo { x, y } => {
                    let x_pos = (x + padding_left + char_padding_x) * self.scale;
                    let y_pos = (y + padding_top + char_padding_y) * self.scale;

                    self.ya_motor.run_to_abs_pos(Some(y_pos as i32));

                    // x motor is little bit slowed down
                    std::thread::sleep(std::time::Duration::from_millis(5));
                    self.xa_motor.run_to_abs_pos(Some(x_pos as i32));

                    #[cfg(target_os = "linux")]
                    self.xa_motor.wait_until_not_moving(Some(std::time::Duration::from_millis(100)));

                    #[cfg(target_os = "linux")]
                    self.ya_motor.wait_until_not_moving(Some(std::time::Duration::from_millis(100)));

                    self.pen_position.x = x_pos;
                    self.pen_position.y = y_pos;

                    println!("[DEBUG] Moved to x: {}, y: {}, on char {}", self.pen_position.x, self.pen_position.y, character.char);
                }
            }
        }

        self.set_drawing(false);

        if (self.writer.written.len() + 1) % self.writer.wrap_after == 0 {
            self.wrap_text();
        }

        self.writer.written.push(character);
        self.writer.to_write.remove(0);
        println!("-------------------------")
    }
}

pub struct PenPosition {
    x: usize,
    y: usize
}

impl PenPosition {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
