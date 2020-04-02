#![allow(dead_code)]

use std::fmt::{self, Display};

#[derive(Clone)]
enum Transform {
    Translation(f32, f32),
    Rotation(f32),
    RotationAroundPoint(f32, f32, f32),
    Scale(f32, f32),
}

#[derive(Clone)]
enum Color {
    Present(u8, u8, u8),
    NonPresent
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

#[derive(Clone, Default)]
pub struct ShapeTransform {
    transforms: Vec<Transform>
}

#[derive(Clone)]
pub enum Shape {
    Rect(f32, f32, f32, f32),
    RoundRect(f32, f32, f32, f32, f32, f32),
    Circle(f32, f32, f32),
    Ellipse(f32, f32, f32, f32),
    Line(f32, f32, f32, f32),
    Polyline(Vec<(f32, f32)>),
    Polygon(Vec<(f32, f32)>),
    Text(f32, f32, String),
    Complex(Vec<Shape>),
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

    pub fn translate(self, x: f32, y: f32) -> Self {
        self.add_transform(Transform::Translation(x, y))
    }

    pub fn rotate(self, angle: f32) -> Self {
        self.add_transform(Transform::Rotation(angle))
    }

    pub fn rotate_around_point(self, angle: f32, x: f32, y: f32) -> Self {
        self.add_transform(Transform::RotationAroundPoint(angle, x, y))
    }

    pub fn scale(self, x: f32, y: f32) -> Self {
        self.add_transform(Transform::Scale(x, y))
    }

    pub fn stroke(self, color: (u8, u8, u8)) -> Self {
        self.style_transform(|style, _transform| {
            style.stroke = Some(Color::Present(color.0, color.1, color.2));
        })
    }

    pub fn no_stroke(self) -> Self {
        self.style_transform(|style, _transform| {
            style.stroke = Some(Color::NonPresent);
        })
    }

    pub fn fill(self, color: (u8, u8, u8)) -> Self {
        self.style_transform(|style, _transform| {
            style.fill = Some(Color::Present(color.0, color.1, color.2));
        })
    }

    pub fn no_fill(self) -> Self {
        self.style_transform(|style, _transform| {
            style.fill = Some(Color::NonPresent);
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
}

impl Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Shape::Rect(x, y, w, h) => {
                write!(f, r#"<rect x="{}" y="{}" width="{}" height="{}" />"#,
                    x, y, w, h)
            },
            Shape::RoundRect(x, y, w, h, rx, ry) => {
                write!(f, r#"<rect x="{}" y="{}" width="{}" height="{}" rx="{}" ry="{}" />"#,
                    x, y, w, h, rx, ry)
            },
            Shape::Circle(cx, cy, r) => {
                write!(f, r#"<circle cx="{}" cy="{}" r="{}" />"#,
                    cx, cy, r)
            },
            Shape::Ellipse(cx, cy, rx, ry) => {
                write!(f, r#"<ellipse cx="{}" cy="{}" rx="{}" ry="{}" />"#,
                    cx, cy, rx, ry)
            },
            Shape::Line(x1, y1, x2, y2) => {
                write!(f, r#"<line x1="{}" y1="{}" x2="{}" y2="{}" />"#,
                    x1, y1, x2, y2)
            },
            Shape::Polyline(points) |
            Shape::Polygon(points) => {
                if !points.is_empty() {
                    match self {
                        Shape::Polyline(..) => write!(f, r#"<polyline points=""#)?,
                        Shape::Polygon(..)  => write!(f, r#"<polygon points=""#)?,
                        _                   => unreachable!(),
                    };

                    for (idx, point) in points.iter().enumerate() {
                        write!(f, "{},{}", point.0, point.1)?;
                        if idx + 1 != points.len() {
                            write!(f, " ")?;
                        }
                    }
                    write!(f, r#"" />"#)?;
                }
                Ok(())
            },
            Shape::Text(x, y, text) => {
                write!(f, r#"<text x="{}" y="{}">{}</text>"#,
                    x, y, text)
            },
            Shape::Complex(subshapes) => {
                for shape in subshapes {
                    writeln!(f, "{}", shape)?;
                }
                Ok(())
            },
            Shape::StyledTransformed(shape, style, transform) => {
                if !transform.transforms.is_empty() {
                    write!(f, r#"<g transform=""#)?;
                    for (idx, trans) in transform.transforms.iter().rev().enumerate() {
                        match trans {
                            Transform::Translation(x, y) => write!(f, "translate({}, {})", x, y)?,
                            Transform::Rotation(angle)   => write!(f, "rotate({})", angle)?,
                            Transform::Scale(x, y)       => write!(f, "scale({}, {})", x, y)?,
                            Transform::RotationAroundPoint(angle, x, y) => 
                                write!(f, "rotate({}, {}, {})", angle, x, y)?,
                        };

                        if idx + 1 != transform.transforms.len() {
                            write! (f, " ")?;
                        }
                    }

                    writeln!(f, r#"">"#)?;
                }

                let format_color = |color: &Color| {
                    match color {
                        Color::Present(r, g, b) => format!("#{:02x}{:02x}{:02x}", r, g, b),
                        Color::NonPresent       => "none".to_string(),
                    }
                };


                write!(f, r#"<g style=""#)?;

                if let Some(stroke) = &style.stroke {
                    write!(f, "stroke:{};", format_color(stroke))?;
                }

                if let Some(fill) = &style.fill {
                    write!(f, "fill:{};", format_color(fill))?;
                }

                if let Some(stroke_width) = &style.stroke_width {
                    write!(f, "stroke-width:{};", stroke_width)?;
                }

                if let Some(fill_opacity) = &style.fill_opacity {
                    write!(f, "fill-opacity:{};", fill_opacity)?;
                }

                if let Some(stroke_opacity) = &style.stroke_opacity {
                    write!(f, "opacity:{};", stroke_opacity)?;
                }

                if let Some(font_family) = &style.font_family {
                    write!(f, "font-family:{};", font_family)?;
                }

                if let Some(font_size) = &style.font_size {
                    write!(f, "font-size:{};", font_size)?;
                }

                if let Some(text_anchor) = &style.text_anchor {
                    let as_text = match text_anchor {
                        TextAnchor::Start    => "start",
                        TextAnchor::Middle   => "middle",
                        TextAnchor::End      => "end",
                    };

                    write!(f, "text-anchor:{};", as_text)?;
                }

                writeln!(f, r#"">"#)?;

                writeln!(f, "{}", shape)?;

                if !transform.transforms.is_empty() {
                    writeln!(f, "</g>")?;
                }
                writeln!(f, "</g>")?;

                Ok(())
            }
        }
    }
}

pub struct SVG {
    shapes: Vec<Shape>,
    width:  u32,
    height: u32,
}

impl SVG {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
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

impl Display for SVG {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"#,
            self.width, self.height)?;

        for shape in &self.shapes {
            writeln!(f, "{}", shape)?;
        }

        writeln!(f, "</svg>")
    }
}
