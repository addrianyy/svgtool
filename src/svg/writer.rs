use super::{Vector, Shape, ShapeStyle, ShapeTransform};
use super::{SVG, Transform, TextAnchor, Color};
use super::path::{Path, CommandType, Command, CombinedCommand};
use std::fmt;

fn write_text(f: &mut fmt::Formatter, (x, y): Vector, text: &str) -> fmt::Result {
    let write_escaped_string = |f: &mut fmt::Formatter, text: &str| -> fmt::Result {
        for ch in text.chars() {
            match ch {
                '"'  => write!(f, "&quot;")?,
                '\'' => write!(f, "&apos;")?,
                '<'  => write!(f, "&lt;")?,
                '>'  => write!(f, "&gt;")?,
                '&'  => write!(f, "&amp;")?,
                ch   => write!(f, "{}", ch)?,
            };
        }

        Ok(())
    };

    write!(f, r#"<text x="{}" y="{}">"#, x, y)?;
    write_escaped_string(f, text)?;
    writeln!(f, "</text>")
}

fn write_poly(f: &mut fmt::Formatter, is_polyline: bool, points: &[Vector]) -> fmt::Result {
    if !points.is_empty() {
        if is_polyline {
            write!(f, r#"<polyline points=""#)?
        } else {
            write!(f, r#"<polygon points=""#)?
        };

        for (idx, (x, y)) in points.iter().enumerate() {
            write!(f, "{},{}", x, y)?;

            if idx + 1 != points.len() {
                write!(f, " ")?;
            }
        }

        writeln!(f, r#"" />"#)?;
    }

    Ok(())
}

fn write_styled_transformed(f: &mut fmt::Formatter, shape: &Shape,
        style: &ShapeStyle, transform: &ShapeTransform) -> fmt::Result {
    let has_transform = !transform.transforms.is_empty();
    let has_style     = style.has_style();
    let make_group    = has_transform || has_style;

    if make_group {
        writeln!(f, "<g {} {}>", style, transform)?;
    }
    
    write!(f, "{}", shape)?;
    
    if make_group {
        writeln!(f, "</g>")?;
    }

    Ok(())
}

impl fmt::Display for ShapeStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.has_style() {
            return Ok(());
        }

        let format_color = |color: &Color| {
            match color {
                Color::Solid(r, g, b) => format!("#{:02x}{:02x}{:02x}", r, g, b),
                Color::None           => "none".to_string(),
            }
        };

        write!(f, r#"style=""#)?;

        if let Some(stroke) = &self.stroke {
            write!(f, "stroke:{};", format_color(stroke))?;
        }

        if let Some(fill) = &self.fill {
            write!(f, "fill:{};", format_color(fill))?;
        }

        if let Some(stroke_width) = &self.stroke_width {
            write!(f, "stroke-width:{};", stroke_width)?;
        }

        if let Some(fill_opacity) = &self.fill_opacity {
            write!(f, "fill-opacity:{};", fill_opacity)?;
        }

        if let Some(stroke_opacity) = &self.stroke_opacity {
            write!(f, "opacity:{};", stroke_opacity)?;
        }

        if let Some(font_family) = &self.font_family {
            write!(f, "font-family:{};", font_family)?;
        }
        if let Some(font_size) = &self.font_size {
            write!(f, "font-size:{};", font_size)?;
        }

        if let Some(text_anchor) = &self.text_anchor {
            let as_text = match text_anchor {
                TextAnchor::Start    => "start",
                TextAnchor::Middle   => "middle",
                TextAnchor::End      => "end",
            };

            write!(f, "text-anchor:{};", as_text)?;
        }

        write!(f, r#"""#)
    }
}

impl fmt::Display for ShapeTransform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.transforms.is_empty() {
            return Ok(());
        }

        write!(f, r#"transform=""#)?;

        for (idx, trans) in self.transforms.iter().rev().enumerate() {
            match trans {
                Transform::Translation((x, y)) => write!(f, "translate({},{})", x, y)?,
                Transform::Scale((x, y))       => write!(f, "scale({},{})", x, y)?,
                Transform::Rotation(angle)     => write!(f, "rotate({})", angle)?,
                Transform::RotationAroundPoint((x, y), angle) => 
                    write!(f, "rotate({},{},{})", angle, x, y)?,
            };

            if idx + 1 != self.transforms.len() {
                write!(f, " ")?;
            }
        }

        write!(f, r#"""#)
    }
}

impl fmt::Display for CombinedCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let is_absolute = matches!(self.command_type, CommandType::Absolute);

        let get_char = |ch: char| {
            if is_absolute { 
                ch.to_uppercase().next().unwrap()
            } else {
                ch.to_lowercase().next().unwrap()
            }
        };

        match self.command {
            Command::MoveTo((x, y)) => {
                write!(f, "{}{} {}", get_char('m'), x, y)?;
            },
            Command::LineTo((x, y)) => {
                if x == 0.0 {
                    write!(f, "{}{}", get_char('v'), y)?;
                } else if y == 0.0 {
                    write!(f, "{}{}", get_char('h'), x)?;
                } else {
                    write!(f, "{}{} {}", get_char('l'), x, y)?;
                }
            },
            Command::QuadCurveTo((x, y), (x1, y1)) => {
                write!(f, "{}{} {},{} {}", get_char('q'), x1, y1, x, y)?;
            },
            Command::CubicCurveTo((x, y), (x1, y1), (x2, y2)) => {
                write!(f, "{}{} {},{} {},{} {}", get_char('c'), x1, y1, x2, y2, x, y)?;
            },
            Command::SmoothQuadCurveTo((x, y)) => {
                write!(f, "{}{} {}", get_char('t'), x, y)?;
            },
            Command::SmoothCubicCurveTo((x, y), (x1, y1)) => {
                write!(f, "{}{} {},{} {}", get_char('s'), x1, y1, x, y)?;
            },
            Command::ClosePath => {
                write!(f, "Z")?;
            },
        };

        Ok(())
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.commands.is_empty() {
            write!(f, r#"<path d=""#)?;

            for (idx, command) in self.commands.iter().enumerate() {
                write!(f, "{}", command)?;

                if idx + 1 != self.commands.len() {
                    write!(f, " ")?;
                }
            }

            writeln!(f, r#"" />"#)?;
        }

        Ok(())
    }
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Shape::Rect((x, y), (w, h)) => {
                writeln!(f, r#"<rect x="{}" y="{}" width="{}" height="{}" />"#, x, y, w, h)
            },
            Shape::RoundRect((x, y), (w, h), (rx, ry)) => {
                writeln!(f, r#"<rect x="{}" y="{}" width="{}" height="{}" rx="{}" ry="{}" />"#,
                    x, y, w, h, rx, ry)
            },
            Shape::Circle((cx, cy), r) => {
                writeln!(f, r#"<circle cx="{}" cy="{}" r="{}" />"#, cx, cy, r)
            },
            Shape::Ellipse((cx, cy), (rx, ry)) => {
                writeln!(f, r#"<ellipse cx="{}" cy="{}" rx="{}" ry="{}" />"#, cx, cy, rx, ry)
            },
            Shape::Line((x1, y1), (x2, y2)) => {
                writeln!(f, r#"<line x1="{}" y1="{}" x2="{}" y2="{}" />"#, x1, y1, x2, y2)
            },
            Shape::Polyline(points) | Shape::Polygon (points) => {
                write_poly(f, matches!(self, Shape::Polyline(..)), points)
            },
            Shape::Text((x, y), text) => {
                write_text(f, (*x, *y), text)
            },
            Shape::Complex(subshapes) => {
                for shape in subshapes { 
                    write!(f, "{}", shape)?;
                }
                Ok(())
            },
            Shape::Ref(shape) => {
                write!(f, "{}", **shape)
            },
            Shape::Path(path) => {
                write!(f, "{}", **path)
            },
            Shape::StyledTransformed(shape, style, transform) => {
                write_styled_transformed(f, shape, style, transform)
            }
        }
    }
}

impl fmt::Display for SVG {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"#,
            self.size.0, self.size.1)?;

        for shape in &self.shapes {
            write!(f, "{}", shape)?;
        }

        writeln!(f, "</svg>")
    }
}
