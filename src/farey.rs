//! For generating sunbursts from the Farey sequences as per
//! https://en.wikipedia.org/wiki/Farey_sequence
//!
//! A farey sequence for a given n is all the completely reduced fractions between 0 to 1
//! which have denominators less than or equal to n
//! e.g. for n = 5 this would be
//! 0/1, 1/5, 1/4, 1/3, 2/5, 1/2, 3/5, 2/3, 3/4, 4/5, 1/1

use std::f64;

use csscolorparser::Color;
use image::{DynamicImage, Rgba, RgbaImage};
use imageproc::{
    drawing::{Canvas, draw_filled_circle_mut, draw_polygon_mut},
    point::Point,
};

const SIZE: u32 = 1024;
const LINE_THICKNESS: i32 = 6;
const CIRCLE_SIZE: i32 = 20;

pub fn generate_farey_sunburst(color: Color, n: i32) -> DynamicImage {
    let mut image = RgbaImage::new(SIZE, SIZE);

    let scale = SIZE as i32 / n / 2 - 20;

    let color = Rgba(color.to_rgba8());

    let centre = ((SIZE / 2) as i32, (SIZE / 2) as i32);
    let top_right_position = |x, y| (centre.0 + x * scale, centre.1 - y * scale);
    let bottom_right_position = |x, y| (centre.0 + x * scale, centre.1 + y * scale);
    let bottom_left_position = |x, y| (centre.0 - x * scale, centre.1 + y * scale);
    let top_left_position = |x, y| (centre.0 - x * scale, centre.1 - y * scale);

    draw_farey_octet(&mut image, top_right_position, false, n, color);
    draw_farey_octet(&mut image, top_right_position, true, n, color);
    draw_farey_octet(&mut image, bottom_right_position, true, n, color);
    draw_farey_octet(&mut image, bottom_right_position, false, n, color);
    draw_farey_octet(&mut image, bottom_left_position, false, n, color);
    draw_farey_octet(&mut image, bottom_left_position, true, n, color);
    draw_farey_octet(&mut image, top_left_position, false, n, color);
    draw_farey_octet(&mut image, top_left_position, true, n, color);

    DynamicImage::ImageRgba8(image)
}

fn draw_farey_octet<F, C>(image: &mut C, position_func: F, swap: bool, n: i32, color: C::Pixel)
where
    F: Fn(i32, i32) -> (i32, i32),
    C: Canvas,
{
    let farey_iterator = if swap {
        FareyIterator::new_descending(n)
    } else {
        FareyIterator::new(n)
    };
    let mut last: Option<(i32, i32)> = None;
    for (mut x, mut y) in farey_iterator {
        if swap {
            std::mem::swap(&mut x, &mut y);
        }
        let position = position_func(x, y);
        draw_filled_circle_mut(image, position, CIRCLE_SIZE, color);
        if let Some(last) = last {
            // draw line between last and this one
            draw_thick_line(
                image,
                color,
                Point::new(last.0, last.1),
                Point::new(position.0, position.1),
                LINE_THICKNESS,
            );
        }
        last = Some(position);
    }
}

fn draw_thick_line<C, P>(
    canvas: &mut C,
    color: P,
    point1: Point<i32>,
    point2: Point<i32>,
    thickness: i32,
) where
    C: Canvas<Pixel = P>,
{
    let angle = f64::atan2(
        point2.y as f64 - point1.y as f64,
        point2.x as f64 - point1.x as f64,
    );

    let perpedicular_angle_1 = angle + f64::consts::PI / 2.0;
    let perpedicular_angle_2 = angle - f64::consts::PI / 2.0;

    let point1_1 = add_point_distance(point1, perpedicular_angle_1, thickness);
    let point1_2 = add_point_distance(point1, perpedicular_angle_2, thickness);

    let point2_1 = add_point_distance(point2, perpedicular_angle_1, thickness);
    let point2_2 = add_point_distance(point2, perpedicular_angle_2, thickness);

    draw_polygon_mut(canvas, &[point1_1, point1_2, point2_2, point2_1], color);
}

fn add_point_distance(point: Point<i32>, angle: f64, distance: i32) -> Point<i32> {
    let distance = distance as f64;
    let x = angle.cos() * distance;
    let y = angle.sin() * distance;
    Point::new(point.x + x as i32, point.y + y as i32)
}

type Fraction = (i32, i32);

fn reduce_fraction(frac: Fraction) -> Fraction {
    let mut fraction = frac;

    // check for numerator = 0
    if fraction.0 == 0 {
        return (0, 1);
    }

    loop {
        let gcd_value = gcd(fraction.0, fraction.1);
        if gcd_value == 1 {
            return fraction;
        }

        fraction = (fraction.0 / gcd_value, fraction.1 / gcd_value);
    }
}

// using euclidean algorithm to get gcd
// https://en.wikipedia.org/wiki/Euclidean_algorithm
fn gcd(num1: i32, num2: i32) -> i32 {
    let mut num1 = num1;
    let mut num2 = num2;
    // always put larger one in num1
    if num2 > num1 {
        std::mem::swap(&mut num1, &mut num2);
    }

    loop {
        let remainder = num1 % num2;

        if remainder == 0 {
            return num2;
        }

        // set num1 to num2 which will be the higher of the two
        num1 = num2;
        num2 = remainder;
    }
}

struct FareyIterator {
    n: i32,
    descending: bool,
    last_fraction: Option<Fraction>,
    last_fraction_2: Option<Fraction>,
}

impl FareyIterator {
    fn new(n: i32) -> Self {
        Self {
            n,
            descending: false,
            last_fraction: None,
            last_fraction_2: None,
        }
    }

    fn new_descending(n: i32) -> Self {
        Self {
            n,
            descending: true,
            last_fraction: None,
            last_fraction_2: None,
        }
    }
}

impl Iterator for FareyIterator {
    type Item = Fraction;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(last_frac) = self.last_fraction {
            if !self.descending && last_frac.0 == 1 && last_frac.1 == 1 {
                // terminate at 1 / 1
                return None;
            }
            if self.descending && last_frac.0 == 0 {
                // terminate at 0 / 1
                return None;
            }
            if let Some(last_frac_2) = self.last_fraction_2 {
                // https://en.wikipedia.org/wiki/Farey_sequence#Next_term
                // using the relation
                // (h + h')/(k + k') = h''/k''
                // where they are in order, h/k, h''/k'', h'/k'
                // so e.g. for n = 5 if h/k = 1/4 and h''/k'' = 1/3
                // then 1/3 = (1 + h')/(4 + k')
                // and we can find some integer q such that
                // q * 1 = 1 + h' and q * 3 = 4 + k' =>
                // h' = q * 1 - 1 and k' = q * 3 - 4
                // want to check the highest value for q which is (n + k)/k''
                // so q = (5 + 4) / 3 = 3 then
                // h' = 3 * 1 - 1 = 2 and k' = 3 * 3 - 4 = 5 so h'/k' = 2/5
                let multiple = (self.n + last_frac_2.1) / last_frac.1;
                let numerator = multiple * last_frac.0 - last_frac_2.0;
                let denominator = multiple * last_frac.1 - last_frac_2.1;

                let new_frac = reduce_fraction((numerator, denominator));
                self.last_fraction_2 = self.last_fraction;
                self.last_fraction = Some(new_frac);
                return Some(new_frac);
            } else {
                let second_frac = if !self.descending {
                    (1, self.n)
                } else {
                    (self.n - 1, self.n)
                };
                self.last_fraction_2 = self.last_fraction;
                self.last_fraction = Some(second_frac);
                return Some(second_frac);
            }
        } else {
            let first_frac = if !self.descending { (0, 1) } else { (1, 1) };
            self.last_fraction = Some(first_frac);
            return Some(first_frac);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        let value = gcd(100, 45);
        assert_eq!(value, 5);

        let value = gcd(1, 3);
        assert_eq!(value, 1);

        let value = gcd(84, 36);
        assert_eq!(value, 12);
    }

    #[test]
    fn test_farey() {
        let mut farey_iterator = FareyIterator::new(5);
        assert_eq!(Some((0, 1)), farey_iterator.next());
        assert_eq!(Some((1, 5)), farey_iterator.next());
        assert_eq!(Some((1, 4)), farey_iterator.next());
        assert_eq!(Some((1, 3)), farey_iterator.next());
        assert_eq!(Some((2, 5)), farey_iterator.next());
        assert_eq!(Some((1, 2)), farey_iterator.next());
        assert_eq!(Some((3, 5)), farey_iterator.next());
        assert_eq!(Some((2, 3)), farey_iterator.next());
        assert_eq!(Some((3, 4)), farey_iterator.next());
        assert_eq!(Some((4, 5)), farey_iterator.next());
        assert_eq!(Some((1, 1)), farey_iterator.next());
        assert_eq!(None, farey_iterator.next());
    }

    #[test]
    fn test_farey_descending() {
        let mut farey_iterator = FareyIterator::new_descending(5);
        assert_eq!(Some((1, 1)), farey_iterator.next());
        assert_eq!(Some((4, 5)), farey_iterator.next());
        assert_eq!(Some((3, 4)), farey_iterator.next());
        assert_eq!(Some((2, 3)), farey_iterator.next());
        assert_eq!(Some((3, 5)), farey_iterator.next());
        assert_eq!(Some((1, 2)), farey_iterator.next());
        assert_eq!(Some((2, 5)), farey_iterator.next());
        assert_eq!(Some((1, 3)), farey_iterator.next());
        assert_eq!(Some((1, 4)), farey_iterator.next());
        assert_eq!(Some((1, 5)), farey_iterator.next());
        assert_eq!(Some((0, 1)), farey_iterator.next());
        assert_eq!(None, farey_iterator.next());
    }
}
