pub enum Figure {
    Circle(f32),             // radius of a circle
    Quadrate(f32),           // one side of a quadrate
    Rectangle(f32, f32),     // sides of a rectangle
    Triangle(f32, f32, f32), // sides of a triangle
}

pub mod area_calculation {
    use crate::Figure;
    use std::f32::consts::PI;
    

    pub fn area_calculation(figure: Figure) -> f32 {
        match figure {
            Figure::Quadrate(side) => side * side,
            Figure::Circle(radius) => f32::powf(radius, 2f32) * PI,
            Figure::Rectangle(side_one, side_two) => side_one * side_two,
            Figure::Triangle(side_one, side_two, side_three) => ((side_one + side_two + side_three)
                / 2f32
                * ((side_one + side_two + side_three) / 2f32 - side_one)
                * ((side_one + side_two + side_three) / 2f32 - side_two)
                * ((side_one + side_two + side_three) / 2f32 - side_three))
                .sqrt(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::area_calculation::area_calculation;
    use crate::Figure;

    #[test]
    fn quadrate_area_calculation() {
        assert_eq!(area_calculation(Figure::Quadrate(2f32)), 4f32);
        assert_ne!(area_calculation(Figure::Quadrate(2f32)), 0f32)
    }

    #[test]
    fn circle_area_calculation() {
        assert_eq!(area_calculation(Figure::Circle(5f32)), 78.53982);
    }

    #[test]
    fn rectangle_area_calculation() {
        assert_eq!(area_calculation(Figure::Rectangle(3f32, 4f32)), 12f32);
    }

    #[test]
    fn triangle_area_calculation() {
        assert_eq!(area_calculation(Figure::Triangle(3f32, 4f32, 5f32)), 6f32);
    }
}
