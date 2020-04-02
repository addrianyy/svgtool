use std::rc::Rc;

use super::Shape;

pub(super) struct CombinedCommand {
    pub command_type: CommandType,
    pub command:      Command,
}

pub enum CommandType {
    Absolute,
    Relative,
}

pub(super) enum Command {
    MoveTo((f32, f32)),
    LineTo((f32, f32)),
    QuadCurveTo((f32, f32), (f32, f32)),
    CubicCurveTo((f32, f32), (f32, f32), (f32, f32)),
    SmoothQuadCurveTo((f32, f32)),
    SmoothCubicCurveTo((f32, f32), (f32, f32)),
    ClosePath,
}

pub struct Path {
    pub(super) commands: Vec<CombinedCommand>,
}

impl Path {
    fn add(mut self, typ: CommandType, command: Command) -> Self {
        self.commands.push(CombinedCommand {
            command,
            command_type: typ,
        });

        self
    }

    pub fn new() -> Self {
        Self { 
            commands: Vec::new()
        }
    }

    pub fn move_to(self, typ: CommandType, (x, y): (f32, f32)) -> Self {
        self.add(typ, Command::MoveTo((x, y)))
    }

    pub fn line_to(self, typ: CommandType, (x, y): (f32, f32)) -> Self {
        self.add(typ, Command::LineTo((x, y)))
    }

    pub fn quad_curve_to(self, typ: CommandType, (x, y): (f32, f32), 
            (x1, y1): (f32, f32)) -> Self {
        self.add(typ, Command::QuadCurveTo((x, y), (x1, y1)))
    }

    pub fn cubic_curve_to(self, typ: CommandType, (x, y): (f32, f32), 
            (x1, y1): (f32, f32), (x2, y2): (f32, f32)) -> Self {
        self.add(typ, Command::CubicCurveTo((x, y), (x1, y1), (x2, y2)))
    }

    pub fn cont_quad_curve_to(self, typ: CommandType, (x, y): (f32, f32)) -> Self {
        self.add(typ, Command::SmoothQuadCurveTo((x, y)))
    }

    pub fn cont_cubic_curve_to(self, typ: CommandType, 
            (x, y): (f32, f32), (x1, y1): (f32, f32)) -> Self {
        self.add(typ, Command::SmoothCubicCurveTo((x, y), (x1, y1)))
    }

    pub fn close(self) -> Self {
        self.add(CommandType::Absolute, Command::ClosePath)
    }

    pub fn shape(self) -> Shape {
        Shape::Path(Rc::new(self))
    }
}
