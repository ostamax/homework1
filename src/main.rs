use figure_calculation::area_calculation::square_calculation;
use figure_calculation::Figure;

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        let message = String::from(
            "Arguments should be provided!\nCircle radius_value or\n
            Quadrate side_length_value or\n
            Rectangle side_one_length_value side_two_length_value or\n
            Triangle side_one_length_value side_two_length_value side_three_length_value",
        );
        print_message_and_exit(message);
    }

    let subcommand = args.remove(0);
    println!("{:?}", args);
    let figure = match subcommand.as_str() {
        "Circle" => {
            if args.len() == 1 {
                Figure::Circle(args[0].parse::<f32>().unwrap())
            } else {
                print_message_and_exit(String::from("A radius of the circle must be provided!"))
            }
        }
        "Quadrate" => {
            if args.len() == 1 {
                Figure::Quadrate(args[0].parse::<f32>().unwrap())
            } else {
                print_message_and_exit(String::from("A side of the quadrate must be provided!"))
            }
        }
        "Rectangle" => {
            if args.len() == 2 {
                Figure::Rectangle(
                    args[0].parse::<f32>().unwrap(),
                    args[1].parse::<f32>().unwrap(),
                )
            } else {
                print_message_and_exit(String::from("Two sides of the rectangle must be provided!"))
            }
        }
        "Triangle" => {
            if args.len() == 3 {
                Figure::Triangle(
                    args[0].parse::<f32>().unwrap(),
                    args[1].parse::<f32>().unwrap(),
                    args[2].parse::<f32>().unwrap(),
                )
            } else {
                print_message_and_exit(String::from("Three sides of a rectangle must be provided!"))
            }
        }
        _ => print_message_and_exit(String::from(
            "Supported figures are: Circle, Quadrate, Rectangle and Triangle!",
        )),
    };
    println!("{}: {:?}", subcommand, square_calculation(figure));
}

fn print_message_and_exit(message: String) -> Figure {
    println!("{}", message);
    std::process::exit(-1);
}
