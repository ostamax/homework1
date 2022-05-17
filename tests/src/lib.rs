#[cfg(test)]
mod tests {
    use figure_calculation::Figures;
    use figure_calculation::area_calculation::area_calculation;
    use figure_calculation::volume_calculation::volume_calculation;

    #[test]
    fn quadrate_area_calculation() {
        assert_eq!(area_calculation(&Figures::Quadrate(2f64)), 4f64);
    }

    #[test]
    fn quadrate_volume_calculation() {
        assert_eq!(volume_calculation(&Figures::Quadrate(2f64)), 0f64);
    }

    #[test]
    fn circle_area_calculation() {
        assert_eq!(f64::trunc(area_calculation(&Figures::Circle(5f64)) * 10f64) / 10f64, 78.5);
    }

    #[test]
    fn circle_volume_calculation() {
        assert_eq!(volume_calculation(&Figures::Circle(2f64)), 0f64);
    }

    #[test]
    fn rectangle_area_calculation() {
        assert_eq!(area_calculation(&Figures::Rectangle(3f64, 4f64)), 12f64);
    }

    #[test]
    fn rectangle_volume_calculation() {
        assert_eq!(volume_calculation(&Figures::Rectangle(3f64, 4f64)), 0f64);
    }

    #[test]
    fn triangle_area_calculation() {
        assert_eq!(area_calculation(&Figures::Triangle(3f64, 4f64, 5f64)), 6f64);
    }

    #[test]
    fn triangle_volume_calculation() {
        assert_eq!(volume_calculation(&Figures::Triangle(3f64, 4f64, 5f64)), 0f64);
    }

    #[test]
    fn sphere_area_calculation() {
        assert_eq!(area_calculation(&Figures::Sphere(2f64)), 0f64);
    }

    #[test]
    fn sphere_volume_calculation() {
        assert_eq!(f64::trunc(volume_calculation(&Figures::Sphere(2f64)) * 10f64) / 10f64, 33.5)
    }

    #[test]
    fn cube_area_calculation() {
        assert_eq!(area_calculation(&Figures::Cube(2f64)), 0f64);
    }

    #[test]
    fn cube_volume_calculation() {
        assert_eq!(volume_calculation(&Figures::Cube(2f64)), 8f64);
    }

    #[test]
    fn cylinder_area_calculation() {
        assert_eq!(area_calculation(&Figures::Cylinder(5f64, 2f64)), 0f64)
    }

    #[test]
    fn cylinder_volume_calculation() {
        assert_eq!(f64::trunc(volume_calculation(&Figures::Cylinder(5f64, 2f64)) * 10f64) / 10f64, 157f64)
    }

    #[test]
    fn cuboid_area_calculation() {
        assert_eq!(area_calculation(&Figures::Cuboid(1f64, 2f64, 3f64)), 0f64)
    }

    #[test]
    fn cuboid_volume_calculation() {
        assert_eq!(volume_calculation(&Figures::Cuboid(1f64, 2f64, 3f64)), 6f64)
    }
}
