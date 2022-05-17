use figure_calculation::area_calculation::area_calculation;
use figure_calculation::volume_calculation::volume_calculation;
use figure_calculation::Figures;

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        println!(
            "Arguments should be provided!\n
            Circle radius_value or\n
            Quadrate side_length_value or\n
            Rectangle side_one_length_value side_two_length_value or\n
            Triangle side_one_length_value side_two_length_value side_three_length_value or\n
            Sphere radius_value or\n
            Cube edge_length_value or\n
            Cylinder base_radius_value height or\n
            Cuboid edge_one_length_value edge_two_length_value edge_three_length_value",
        );
    }

    let figure_arg = args.remove(0);

    let parsed_args: Vec<f64> = args.iter().filter_map(|s| s.parse().ok()).collect();

    if parsed_args.iter().any(|&x| x <= 0f64) {
        panic!("Figure's dimensions must be greater than zero!")
    }

    let figure = match figure_arg.as_str() {
        "Circle" => {
            if args.len() == 1 {
                Figures::Circle(parsed_args[0])
            } else {
                println!("A radius of the circle must be provided!");
                return;
            }
        }
        "Quadrate" => {
            if args.len() == 1 {
                Figures::Quadrate(parsed_args[0])
            } else {
                println!("A side of the quadrate must be provided!");
                return;
            }
        }
        "Rectangle" => {
            if args.len() == 2 {
                Figures::Rectangle(parsed_args[0], parsed_args[1])
            } else {
                println!("Two sides of the rectangle must be provided!");
                return;
            }
        }
        "Triangle" => {
            if args.len() == 3 {
                Figures::Triangle(parsed_args[0], parsed_args[1], parsed_args[2])
            } else {
                println!("Three sides of a rectangle must be provided!");
                return;
            }
        }
        "Sphere" => {
            if args.len() == 1 {
                Figures::Sphere(parsed_args[0])
            } else {
                println!("A radius of the sphere must be provided!r");
                return;
            }
        }
        "Cube" => {
            if args.len() == 1 {
                Figures::Cube(parsed_args[0])
            } else {
                println!("An edge of the cube must be provided!");
                return;
            }
        }
        "Cylinder" => {
            if args.len() == 2 {
                Figures::Cylinder(parsed_args[0], parsed_args[1])
            } else {
                println!("Radius and height of the cylinder must be provided!");
                return;
            }
        }
        "Cuboid" => {
            if args.len() == 3 {
                Figures::Cuboid(parsed_args[0], parsed_args[1], parsed_args[2])
            } else {
                println!("Edges of the cuboid must be provided!");
                return;
            }
        }
        _ => {
            println!("Supported figures are: Circle, Quadrate, Rectangle, Triangle, Sphere, Cude, Cylinder and Cuboid!");
            return;
        }
    };

    println!(
        "Figure: {}, Area: {:?}, Volume: {:?}",
        figure_arg,
        area_calculation(&figure),
        volume_calculation(&figure)
    );
}
