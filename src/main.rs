mod svg;
use svg::prelude::*;

fn main() {
    let mut svg = SVG::new((3000, 3000));
    
    let mut triangle = Shape::Polygon(vec![(0.5, 0.0), (0.0, 1.0), (1.0, 1.0)]).make_ref();
    
    for _ in 0..7 {
        triangle = Shape::Complex(vec![
            triangle.clone(),
            triangle.clone().translate((-0.5, 1.0)),
            triangle.clone().translate((0.5, 1.0)),
        ]).scale((0.5, 0.5)).make_ref();
    }

    svg.add(
        triangle
            .scale((400.0, 400.0))
            .translate((800.0, 800.0))
            .fill((255, 0, 0))
    );
    
    /*
    svg.add(
        Shape::Line(0.0, 0.0, 100.0, 200.0)
            .stroke((0, 0, 255))
            .stroke_width(5.0)
            .stroke_opacity(0.3)
    );

    svg.add(
        Shape::RoundRect(100.0, 100.0, 500.0, 400.0, 50.0, 50.0)
            .fill((0, 255, 0))
    );

    svg.add(
        Shape::Text(0.0, 0.0, "Hello world >>> &&XD".to_string())
            .font_size(80)
            .font_family("Arial")
            .text_anchor(svg::TextAnchor::Middle)
            .rotate(90.0)
            .translate(400.0, 400.0)
    );
    */

    //let mut path = Path::new();
    //path.move_to(Absolute, 200.0, 300.0);
    //path.line_to(Relative, 100.0, 100.0);

    /*
    path.move_to(Absolute, 200.0, 200.0);
    path.line_to(Relative, -100.0, 0.0);
    path.line_to(Relative, 0.0, -100.0);

    path.move_to(Absolute, 200.0, 200.0);
    path.line_to(Relative, 100.0, 0.0);
    path.line_to(Relative, 0.0, 100.0);

    path.move_to(Absolute, 200.0, 200.0);
    path.line_to(Relative, 0.0, -100.0);
    path.line_to(Relative, 100.0, 0.0);

    path.move_to(Absolute, 200.0, 200.0);
    path.line_to(Relative, 0.0, 100.0);
    path.line_to(Relative, -100.0, 0.0);
    */

    let path = Path::new()
        .move_to(Absolute, (500.0, 500.0))
        .quad_curve_to(Relative, (1000.0, 0.0), (500.0, 400.0))
        .cont_quad_curve_to(Relative, (1000.0, 0.0))
        .cont_quad_curve_to(Relative, (200.0, 300.0))
        .shape()
        .no_fill()
        .stroke((255, 0, 0))
        .stroke_width(5.0);

    svg.add(path);

    use std::io::Write;

    let mut file = std::fs::File::create("result.svg").unwrap();
    write!(file, "{}", svg).unwrap();

}
