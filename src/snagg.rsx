// use clap::{CommandFactory, Parser, Subcommand};
// use prettytable::format::consts::FORMAT_CLEAN;
// use prettytable::{cell, row, Table};
// use reqwest::blocking::Client;
// use serde::Deserialize;
// use std::error::Error;
// use std::io::Write;
// use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

// #[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
// enum Args {
//     /// Name of the person to greet
//     args: Vec<String>,

//     /// Number of times to greet
//     #[arg(short, long)]
//     api_key: u8,
// }

// // Replace 'YOUR_API_KEY' with your actual USDA FoodData Central API key
// const API_KEY: &str = "uwLUcab8yUVedyyxI3tdxEvodNjZaCnF89Tipih8";
// const FOOD_ITEM: &str = "apple";

// // Endpoint for searching food items with pageSize parameter
// const SEARCH_URL: &str = "https://api.nal.usda.gov/fdc/v1/food/search";

// #[derive(Deserialize, Debug)]
// struct Nutrient {
//     nutrientName: String,
//     value: f64,
//     unitName: String,
// }

// #[derive(Deserialize, Debug)]
// struct Food {
//     description: String,
//     fdcId: u32,
//     foodNutrients: Vec<Nutrient>,
// }

// #[derive(Deserialize, Debug)]
// struct FoodResponse {
//     foods: Vec<Food>,
// }

// fn main() -> Result<(), Box<dyn Error>> {
//     let args = Args::parse();

//     let client = Client::new();

//     // Include the pageSize parameter to limit results to 1
//     let request_url = format!(
//         "{}?query={}&pageSize=1&api_key={}",
//         SEARCH_URL, FOOD_ITEM, API_KEY
//     );
//     let response = client.get(&request_url).send()?;

//     // Parse the JSON response
//     let food_response: FoodResponse = response.json()?;

//     if let Some(first_food) = food_response.foods.into_iter().next() {
//         let mut stdout = StandardStream::stdout(ColorChoice::Always);

//         // Print description and FDC ID with color
//         // stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)).set_bold(true))?;

//         // Create a table for nutrients
//         let mut table = Table::new();
//         // table.set_format(*FORMAT_CLEAN);
//         table.add_row(row!["Nutrient", "Amount", "Unit"]);

//         for nutrient in first_food.foodNutrients {
//             table.add_row(row![
//                 Fg->nutrient.nutrientName,
//                 Fg->nutrient.value,
//                 Fg->nutrient.unitName
//             ]);
//         }

//         // Print the table
//         table.printstd();
//     } else {
//         println!("No food items found.");
//     }

//     Ok(())
// }
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// track your macros
    Track(TrackArgs),
    /// Set FoodData Central API key
    SetApiKey(TrackArgs),
}

#[derive(Args)]
struct TrackArgs {
    #[arg(value_parser = parse_foods)]
    foods: Vec<String>,
}

#[derive(Args)]
struct KeyArgs {
    key: Option<String>,
}

fn parse_foods(s: &str) -> Result<Vec<String>, String> {
    println!("{}", s);
    // Ok(Vec::from([s.to_string()]))
    Err("hello".to_string())
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Track(name) => {
            println!("'myapp add' was used, name is: {:?}", name.foods)
        }
        Commands::SetApiKey(_) => todo!(),
    }
}
