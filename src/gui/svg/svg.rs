use rust_graphics::{
    color::COLOR_BLACK,
    draw_command::{DrawCommand, Stroke},
    path_builder::{Path, PathBuilder},
    rect::Rect,
    vec::Vec2,
};
use svgtypes::{PathParser, PathSegment};
use xml::{reader::XmlEvent, EventReader};

use crate::{error::Error, prelude::Result};

#[derive(Debug, Clone)]
enum SvgElement {
    Path(Path),
}

#[derive(Debug, Clone)]
pub struct Svg {
    segments: Vec<SvgElement>,
    size: Vec2,
}

impl Svg {
    pub fn get_size(&self) -> Vec2 {
        self.size
    }

    pub fn generate_draw_commands(&self, rect: Rect) -> Vec<DrawCommand> {
        let mut res = Vec::new();
        for elem in self.segments.iter() {
            match elem {
                SvgElement::Path(path) => {
                    res.push(DrawCommand::Path(path.clone(), self.size, rect))
                }
            }
        }
        res
    }

    /// Loads Svg from a string.
    /// The string should only countain one <svg> tag and all its children.
    pub fn load_from_str(_s: &str) -> Result<Svg> {
        let reader = EventReader::from_str(_s);
        let mut result = None;
        for event in reader {
            match event.map_err(|e| Error::SvgParseError(e.to_string()))? {
                XmlEvent::StartElement {
                    name, attributes, ..
                } => match name.local_name.as_str() {
                    "svg" => {
                        if result.is_some() {
                            return Err(Error::SvgParseError(
                                "Svg should only contain one <svg> tag".into(),
                            ));
                        }

                        let width = attributes
                            .iter()
                            .find(|a| a.name.local_name == "width")
                            .map(|a| {
                                a.value.parse::<f32>().map_err(|e| {
                                    Error::SvgParseError(format!(
                                        "Could not parse width from \"{}\"",
                                        e.to_string()
                                    ))
                                })
                            })
                            .ok_or(Error::SvgParseError(
                                "Svg should have a width attribute".into(),
                            ))??;

                        let height = attributes
                            .iter()
                            .find(|a| a.name.local_name == "height")
                            .map(|a: &xml::attribute::OwnedAttribute| {
                                a.value.parse::<f32>().map_err(|e| {
                                    Error::SvgParseError(format!(
                                        "Could not parse height from \"{}\"",
                                        e.to_string()
                                    ))
                                })
                            })
                            .ok_or(Error::SvgParseError(
                                "Svg should have a height attribute".into(),
                            ))??;

                        result = Some(Svg {
                            size: Vec2::new(width, height),
                            segments: Vec::new(),
                        });
                    }
                    "path" => {
                        let result = result.as_mut().ok_or(Error::SvgParseError(
                            "You need to specify an <svg> tag before defining a path!".into(),
                        ))?;
                        let d = attributes
                            .iter()
                            .find(|a| a.name.local_name == "d")
                            .map(|a| a.value.clone())
                            .ok_or(Error::SvgParseError(
                                "Path should have a d attribute".into(),
                            ))?;
                        let mut path_builder = PathBuilder::new();
                        path_builder.stroke(Some(Stroke::new(COLOR_BLACK, 4.0)));
                        //path_builder.fill(Some(Fill::new(COLOR_BLUE)));
                        for seg in PathParser::from(d.as_str()) {
                            if let Ok(seg) = seg {
                                match seg {
                                    PathSegment::MoveTo { abs, x, y } => {
                                        if abs {
                                            path_builder.move_to(Vec2::new(x as f32, y as f32));
                                        } else {
                                            path_builder.move_to_rel(Vec2::new(x as f32, y as f32));
                                            //path_builder.move_to(Vec2::new(x as f32, y as f32));
                                        }
                                    }
                                    PathSegment::LineTo { abs, x, y } => {
                                        if abs {
                                            path_builder.line_to(Vec2::new(x as f32, y as f32));
                                        } else {
                                            path_builder.line_to_rel(Vec2::new(x as f32, y as f32));
                                        }
                                    }
                                    PathSegment::VerticalLineTo { abs, y } => {
                                        if abs {
                                            path_builder.vert(y as f32);
                                        } else {
                                            path_builder.vert_rel(y as f32);
                                        }
                                    }
                                    PathSegment::HorizontalLineTo { abs, x } => {
                                        if abs {
                                            path_builder.horiz(x as f32);
                                        } else {
                                            path_builder.horiz_rel(x as f32);
                                        }
                                    }
                                    PathSegment::CurveTo {
                                        abs,
                                        x1,
                                        y1,
                                        x2,
                                        y2,
                                        x,
                                        y,
                                    } => {
                                        if abs {
                                            path_builder.cubic_to(
                                                Vec2::new(x1 as f32, y1 as f32),
                                                Vec2::new(x2 as f32, y2 as f32),
                                                Vec2::new(x as f32, y as f32),
                                            );
                                        } else {
                                            path_builder.cubic_to_rel(
                                                Vec2::new(x1 as f32, y1 as f32),
                                                Vec2::new(x2 as f32, y2 as f32),
                                                Vec2::new(x as f32, y as f32),
                                            );
                                        }
                                    }
                                    PathSegment::SmoothCurveTo { abs, x2, y2, x, y } => {
                                        if abs {
                                            path_builder.smooth_cubic_to(
                                                (x2 as f32, y2 as f32),
                                                (x as f32, y as f32),
                                            );
                                        } else {
                                            path_builder.smooth_cubic_to_rel(
                                                (x2 as f32, y2 as f32),
                                                (x as f32, y as f32),
                                            );
                                        }
                                    }
                                    PathSegment::Quadratic { abs, x1, y1, x, y } => {
                                        if abs {
                                            path_builder.quad_to(
                                                Vec2::new(x1 as f32, y1 as f32),
                                                Vec2::new(x as f32, y as f32),
                                            );
                                        } else {
                                            path_builder.quad_to_rel(
                                                Vec2::new(x1 as f32, y1 as f32),
                                                Vec2::new(x as f32, y as f32),
                                            );
                                        }
                                    }
                                    PathSegment::SmoothQuadratic { abs, x, y } => {
                                        if abs {
                                            path_builder
                                                .smooth_quad_to(Vec2::new(x as f32, y as f32));
                                        } else {
                                            path_builder
                                                .smooth_quad_to_rel(Vec2::new(x as f32, y as f32));
                                        }
                                    }
                                    PathSegment::EllipticalArc {
                                        abs,
                                        rx,
                                        ry,
                                        x_axis_rotation,
                                        large_arc,
                                        sweep,
                                        x,
                                        y,
                                    } => {
                                        if abs {
                                            path_builder.arc_to(
                                                (x as f32, y as f32),
                                                (rx as f32, ry as f32),
                                                x_axis_rotation as f32,
                                                large_arc,
                                                sweep,
                                            )
                                        } else {
                                            path_builder.arc_to_rel(
                                                (x as f32, y as f32),
                                                (rx as f32, ry as f32),
                                                x_axis_rotation as f32,
                                                large_arc,
                                                sweep,
                                            )
                                        }
                                    }
                                    PathSegment::ClosePath { .. } => {
                                        path_builder.close();
                                    }
                                }
                            }
                        }
                        result.segments.push(SvgElement::Path(path_builder.build()));
                    }
                    elem => {
                        println!("Unknown svg element: {}", elem);
                    }
                },
                XmlEvent::EndElement { name } => {
                    if name.local_name == "svg" {
                        break;
                    }
                }
                _ => {}
            }
        }
        result.ok_or(Error::SvgParseError("No svg tag found!".into()))
    }
}
