use clap::Parser;
use owo_colors::OwoColorize;
use std::{fmt::Error, fs, str::FromStr};

#[derive(Clone, Debug)]
enum ComponentType {
    BUTTON,
}

impl FromStr for ComponentType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "button" => Ok(ComponentType::BUTTON),
            _ => panic!(),
        }
    }
}

struct Component {
    _type: ComponentType,
    name: String,
    path: String,
}

impl Component {
    fn new(component_type: &ComponentType) -> Component {
        match component_type {
            ComponentType::BUTTON => Component {
                _type: ComponentType::BUTTON,
                name: "button".to_string(),
                path: "resources/button.tsx".to_string(),
            },
        }
    }
}

/// Simple component extension CLI
#[derive(Parser, Debug)]
struct Args {
    /// Component of choice
    #[arg(short)]
    component: ComponentType,

    /// Output file,
    /// it assumes that you're running this script from the root of the project
    #[arg(short, long, default_value_t = String::from("components"))]
    output: String,

    /// Source folder,
    /// Whether you're using a SRC folder or not
    #[arg(short, default_value_t = true)]
    source_folder: bool,
}

fn main() {
    let args = Args::parse();

    let mut component = Component::new(&args.component);
    let mut output_path = format!("{}/{}.tsx", args.output, component.name);
    let mut component_folder_path = "components";

    if args.source_folder {
        component.path = format!("src/{}", component.path);
        output_path = format!("src/{}", output_path);
        component_folder_path = "src/components";
    }

    let component_file = match fs::read(&component.path) {
        Ok(value) => value,
        Err(_) => {
            panic!(
                "{} {}",
                "Unable to find the component, trying to map the following:".red(),
                &component.path
            )
        }
    };

    match fs::read_dir(component_folder_path) {
        Ok(_) => (),
        Err(_) => {
            println!(
                "{}, {}",
                "Components folder not found".red(),
                "creating folder as a default behaviour".dimmed()
            );
            fs::create_dir(component_folder_path).unwrap()
        }
    };

    match fs::write(&output_path, component_file) {
        Ok(_) => println!(
            "Component {} created on {}",
            component.name.bright_cyan(),
            &output_path.bright_cyan()
        ),
        Err(err) => panic!(
            "{} {}",
            "Unable to create component, an error ocurred\n Trace:".red(),
            err
        ),
    }
}
