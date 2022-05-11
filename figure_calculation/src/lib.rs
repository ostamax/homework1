pub enum Figure {
    Circle(f32),             // radius of a circle
    Quadrate(f32),           // one side of a quadrate
    Rectangle(f32, f32),     // sides of a rectangle
    Triangle(f32, f32, f32), // sides of a triangle
}

pub mod area_calculation {
    use crate::Figure;
    use std::f32::consts::PI;
    

    pub fn square_calculation(figure: Figure) -> f32 {
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
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
