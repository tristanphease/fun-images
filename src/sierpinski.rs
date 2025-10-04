//! Generates sierpinski triangle fractals
//!
//!

use csscolorparser::Color;
use image::{DynamicImage, GenericImage, Rgba, RgbaImage};
use imageproc::drawing::draw_line_segment_mut;

#[derive(Clone, Copy, Debug)]
enum TriangleDirection {
    Up,
    Down,
}

impl TriangleDirection {
    #[allow(unused)]
    fn other(self) -> Self {
        match self {
            TriangleDirection::Up => TriangleDirection::Down,
            TriangleDirection::Down => TriangleDirection::Up,
        }
    }
}

fn draw_triangle_mut<I>(
    image: &mut I,
    color: I::Pixel,
    triangle: Triangle,
) where
    I: GenericImage,
{
    let Triangle {
        direction,
        centre,
        height,
    } = triangle;
    // width of the triangle will be the same as the height
    // need to draw 3 lines
    let factor = match direction {
        TriangleDirection::Down => 1.0,
        TriangleDirection::Up => -1.0,
    };
    let pos1 = (centre.0, centre.1 - factor * height / 2.0);
    let pos2 = (centre.0 + factor * height / 2.0, centre.1 + factor * height / 2.0);
    let pos3 = (centre.0 - factor * height / 2.0, centre.1 + factor * height / 2.0);

    draw_line_segment_mut(image, pos1, pos2, color);
    draw_line_segment_mut(image, pos2, pos3, color);
    draw_line_segment_mut(image, pos3, pos1, color);
}

#[derive(Clone, Copy, Debug)]
struct Triangle {
    centre: (f32, f32),
    height: f32,
    direction: TriangleDirection,
}

pub fn generate_sierpinski_image(color: Color, size: u32) -> DynamicImage {
    let mut image = RgbaImage::new(size, size);

    let centre = size as f32 / 2.0;
    let mut triangles = vec![];
    triangles.push(Triangle {
        centre: (centre, centre),
        height: centre * 1.8,
        direction: TriangleDirection::Down
    });

    let color = color.to_rgba8();
    while let Some(triangle) = triangles.pop() {
        draw_triangle_mut(&mut image, Rgba(color), triangle);
        if triangle.height >= 10.0 {
            let factor = match triangle.direction {
                TriangleDirection::Up => 1.0,
                TriangleDirection::Down => -1.0,
            };
            let new_height = triangle.height / 2.0;
            let triangle1 = Triangle {
                centre: (triangle.centre.0 - factor * new_height / 2.0, triangle.centre.1 - factor * new_height / 2.0),
                height: new_height,
                direction: triangle.direction,
            };
            let triangle2 = Triangle {
                centre: (triangle.centre.0 + factor * new_height / 2.0, triangle.centre.1 - factor * new_height / 2.0),
                height: new_height,
                direction: triangle.direction,
            };
            let triangle3 = Triangle {
                centre: (triangle.centre.0, triangle.centre.1 + factor * new_height / 2.0),
                height: new_height,
                direction: triangle.direction,
            };

            triangles.push(triangle1);
            triangles.push(triangle2);
            triangles.push(triangle3);
        }
    }

    DynamicImage::ImageRgba8(image)
}
