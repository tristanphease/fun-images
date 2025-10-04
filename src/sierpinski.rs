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
    // for some reason i thought the triangles switched direction each iteration
    // which did generate some cool images so leaving this here in case i want to use it
    // for something in the future
    #[allow(unused)]
    fn other(self) -> Self {
        match self {
            TriangleDirection::Up => TriangleDirection::Down,
            TriangleDirection::Down => TriangleDirection::Up,
        }
    }
}

fn draw_triangle_mut<I>(image: &mut I, color: I::Pixel, triangle: Triangle)
where
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
    let pos2 = (
        centre.0 + factor * height / 2.0,
        centre.1 + factor * height / 2.0,
    );
    let pos3 = (
        centre.0 - factor * height / 2.0,
        centre.1 + factor * height / 2.0,
    );

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
    let sierpinski_image = generate_sierpinski_image_with_zoom(color, size, 0.0);

    DynamicImage::ImageRgba8(sierpinski_image)
}

fn lerp(point1: f32, point2: f32, amount: f32) -> f32 {
    point1 * (1.0 - amount) + point2 * amount
}

fn generate_sierpinski_image_with_zoom(color: Color, size: u32, zoom: f32) -> RgbaImage {
    let mut image = RgbaImage::new(size, size);

    let centre = size as f32 / 2.0;
    let main_triangle_height = centre * 2.0;
    let mut triangles = vec![];
    // zoom towards bottom left
    let zoom_point = (
        centre - -1.0 * main_triangle_height / 2.0,
        centre + -1.0 * main_triangle_height / 2.0,
    );

    let new_centre = (
        lerp(centre, zoom_point.0, zoom),
        lerp(centre, zoom_point.1, zoom),
    );
    let new_height = lerp(main_triangle_height, main_triangle_height * 2.0, zoom);
    triangles.push(Triangle {
        centre: new_centre,
        height: new_height,
        direction: TriangleDirection::Down,
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
                centre: (
                    triangle.centre.0 - factor * new_height / 2.0,
                    triangle.centre.1 - factor * new_height / 2.0,
                ),
                height: new_height,
                direction: triangle.direction,
            };
            let triangle2 = Triangle {
                centre: (
                    triangle.centre.0 + factor * new_height / 2.0,
                    triangle.centre.1 - factor * new_height / 2.0,
                ),
                height: new_height,
                direction: triangle.direction,
            };
            let triangle3 = Triangle {
                centre: (
                    triangle.centre.0,
                    triangle.centre.1 + factor * new_height / 2.0,
                ),
                height: new_height,
                direction: triangle.direction,
            };

            triangles.push(triangle1);
            triangles.push(triangle2);
            triangles.push(triangle3);
        }
    }

    image
}

pub fn generate_sierpinski_zoom_images(color: Color, size: u32) -> Vec<RgbaImage> {
    let mut images = Vec::new();

    for i in 0..=20 {
        let zoom = i as f32 / 20.0;
        let image = generate_sierpinski_image_with_zoom(color.clone(), size, zoom);
        images.push(image);
    }

    images
}
