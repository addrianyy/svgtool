use std::rc::Rc;
use super::{Shape, Vector};

pub(super) struct CombinedCommand {
    pub command_type: CommandType,
    pub command:      Command,
}

pub enum CommandType {
    Absolute,
    Relative,
}

pub(super) enum Command {
    MoveTo(Vector),
    LineTo(Vector),
    QuadCurveTo(Vector, Vector),
    CubicCurveTo(Vector, Vector, Vector),
    SmoothQuadCurveTo(Vector),
    SmoothCubicCurveTo(Vector, Vector),
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

    pub fn move_to(self, typ: CommandType, pos: Vector) -> Self {
        self.add(typ, Command::MoveTo(pos))
    }

    pub fn line_to(self, typ: CommandType, pos: Vector) -> Self {
        self.add(typ, Command::LineTo(pos))
    }

    pub fn quad_curve_to(self, typ: CommandType, pos: Vector, ctrl: Vector) -> Self {
        self.add(typ, Command::QuadCurveTo(pos, ctrl))
    }

    pub fn cubic_curve_to(self, typ: CommandType, pos: Vector, ctrl1: Vector, ctrl2: Vector) 
            -> Self {
        self.add(typ, Command::CubicCurveTo(pos, ctrl1, ctrl2))
    }

    pub fn cont_quad_curve_to(self, typ: CommandType, pos: Vector) -> Self {
        self.add(typ, Command::SmoothQuadCurveTo(pos))
    }

    pub fn cont_cubic_curve_to(self, typ: CommandType, pos: Vector, ctrl: Vector) -> Self {
        self.add(typ, Command::SmoothCubicCurveTo(pos, ctrl))
    }

    pub fn close(self) -> Self {
        self.add(CommandType::Absolute, Command::ClosePath)
    }

    pub fn shape(self) -> Shape {
        Shape::Path(Rc::new(self))
    }
}
