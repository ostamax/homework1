pub enum Figures<T> {
    Circle(T),         // radius of a circle
    Quadrate(T),       // one side of a quadrate
    Rectangle(T, T),   // sides of a rectangle
    Triangle(T, T, T), // sides of a triangle
    Sphere(T),         // radius of a sphere
    Cube(T),           // one side of a cube
    Cylinder(T, T),    // radius of a cylinder's base and haight
    Cuboid(T, T, T),   // edges of a cuboid
}

pub mod area_calculation {
    use crate::Figures;
    use std::f64::consts::PI;
    pub fn area_calculation<T>(figure: &Figures<T>) -> f64
    where
        T: Clone + Into<f64> + Copy,
    {
        match *figure {
            Figures::Circle(circle_radius) => f64::powf(circle_radius.into(), 2f64) * PI,
            Figures::Quadrate(side) => f64::powf(side.into(), 2f64),
            Figures::Rectangle(side_one, side_two) => side_one.into() * side_two.into(),
            Figures::Triangle(side_one, side_two, side_three) => {
                let semi_perimeter = (side_one.into() + side_two.into() + side_three.into()) / 2f64;
                (semi_perimeter
                    * (semi_perimeter - side_one.into())
                    * (semi_perimeter - side_two.into())
                    * (semi_perimeter - side_three.into()))
                .sqrt()
            }
            _ => 0.0,
        }
    }
}

pub mod volume_calculation {
    use crate::Figures;
    use std::f64::consts::PI;
    pub fn volume_calculation<T>(figure: &Figures<T>) -> f64
    where
        T: Into<f64> + Copy,
    {
        match *figure {
            Figures::Sphere(sphere_radius) => {
                4f64 / 3f64 * PI * f64::powf(sphere_radius.into(), 3f64)
            }
            Figures::Cube(edge) => f64::powf(edge.into(), 3f64),
            Figures::Cylinder(radius, height) => {
                PI * f64::powf(radius.into(), 2f64) * height.into()
            }
            Figures::Cuboid(e1, e2, e3) => e1.into() * e2.into() * e3.into(),
            _ => 0.0,
        }
    }
}