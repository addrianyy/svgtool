#![allow(dead_code)]

mod writer;
mod path;
pub mod prelude;

use std::rc::Rc;

pub use path::{CommandType::{Absolute, Relative}, Path};


pub type Vector = (f32, f32);

#[derive(Clone)]
enum Transform {
    Translation((f32, f32)),
    Rotation(f32),
    RotationAroundPoint((f32, f32), f32),
    Scale((f32, f32)),
}

#[derive(Clone)]
enum Color {
    Solid(u8, u8, u8),
    None,
}

#[derive(Clone)]
pub enum TextAnchor {
    Start,
    Middle,
    End,
}

#[derive(Clone, Default)]
pub struct ShapeStyle {
    stroke:         Option<Color>,
    fill:           Option<Color>,
    fill_opacity:   Option<f32>,
    stroke_opacity: Option<f32>,
    stroke_width:   Option<f32>,
    font_size:      Option<u32>,
    font_family:    Option<&'static str>,
    text_anchor:    Option<TextAnchor>,
}

impl ShapeStyle {
    fn has_style(&self) -> bool {
        self.stroke.is_some()         ||
        self.fill.is_some()           ||
        self.fill_opacity.is_some()   ||
        self.stroke_opacity.is_some() ||
        self.stroke_width.is_some()   ||
        self.font_size.is_some()      ||
        self.font_family.is_some()    ||
        self.text_anchor.is_some()
    }
}

#[derive(Clone, Default)]
pub struct ShapeTransform {
    transforms: Vec<Transform>,
}

#[derive(Clone)]
pub enum Shape {
    Rect((f32, f32), (f32, f32)),
    RoundRect((f32, f32), (f32, f32), (f32, f32)),
    Circle((f32, f32), f32),
    Ellipse((f32, f32), (f32, f32)),
    Line((f32, f32), (f32, f32)),
    Polyline(Vec<(f32, f32)>),
    Polygon(Vec<(f32, f32)>),
    Text((f32, f32), String),
    Complex(Vec<Shape>),
    Ref(Rc<Shape>),
    Path(Rc<Path>),
    StyledTransformed(Box<Shape>, ShapeStyle, ShapeTransform),
}

impl Shape {
    fn style_transform(self, func: impl FnOnce(&mut ShapeStyle, &mut ShapeTransform)) -> Self {
        if let Shape::StyledTransformed(shape, mut style, mut transform) = self {
            func(&mut style, &mut transform);
            Shape::StyledTransformed(shape, style, transform)
        } else {
            Shape::StyledTransformed(Box::new(self), ShapeStyle::default(), 
                ShapeTransform::default()).style_transform(func)
        }
    }

    fn add_transform(self, added_transform: Transform) -> Self {
        self.style_transform(|_style, transform| {    
            transform.transforms.push(added_transform);
        })
    }

    pub fn translate(self, (x, y): (f32, f32)) -> Self {
        self.add_transform(Transform::Translation((x, y)))
    }

    pub fn rotate_around_point(self, (x, y): (f32, f32), angle: f32) -> Self {
        self.add_transform(Transform::RotationAroundPoint((x, y), angle))
    }

    pub fn rotate(self, angle: f32) -> Self {
        self.add_transform(Transform::Rotation(angle))
    }

    pub fn scale(self, (x, y): (f32, f32)) -> Self {
        self.add_transform(Transform::Scale((x, y)))
    }

    pub fn stroke(self, (r, g, b): (u8, u8, u8)) -> Self {
        self.style_transform(|style, _transform| {
            style.stroke = Some(Color::Solid(r, g, b));
        })
    }

    pub fn no_stroke(self) -> Self {
        self.style_transform(|style, _transform| {
            style.stroke = Some(Color::None);
        })
    }

    pub fn fill(self, (r, g, b): (u8, u8, u8)) -> Self {
        self.style_transform(|style, _transform| {
            style.fill = Some(Color::Solid(r, g, b));
        })
    }

    pub fn no_fill(self) -> Self {
        self.style_transform(|style, _transform| {
            style.fill = Some(Color::None);
        })
    }

    pub fn stroke_width(self, stroke_width: f32) -> Self {
        self.style_transform(|style, _transform| {
            style.stroke_width = Some(stroke_width);
        })
    }

    pub fn stroke_opacity(self, stroke_opacity: f32) -> Self {
        self.style_transform(|style, _transform| {
            style.stroke_opacity = Some(stroke_opacity);
        })
    }

    pub fn fill_opacity(self, fill_opacity: f32) -> Self {
        self.style_transform(|style, _transform| {
            style.fill_opacity = Some(fill_opacity);
        })
    }

    pub fn font_family(self, font_family: &'static str) -> Self {
        self.style_transform(|style, _transform| {
            style.font_family = Some(font_family);
        })
    }

    pub fn font_size(self, font_size: u32) -> Self {
        self.style_transform(|style, _transform| {
            style.font_size = Some(font_size);
        })
    }

    pub fn text_anchor(self, text_anchor: TextAnchor) -> Self {
        self.style_transform(|style, _transform| {
            style.text_anchor = Some(text_anchor);
        })
    }

    pub fn make_ref(self) -> Self {
        Shape::Ref(Rc::new(self))
    }
}

pub struct SVG {
    shapes: Vec<Shape>,
    size:   (u32, u32),
}

impl SVG {
    pub fn new(size: (u32, u32)) -> Self {
        Self {
            size,
            shapes: Vec::new(),
        }
    }

    pub fn add(&mut self, shape: Shape) {
        self.shapes.push(shape);
    }

    pub fn add_many(&mut self, shapes: &[Shape]) {
        self.shapes.extend_from_slice(shapes);
    }
}
