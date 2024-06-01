use clap::{Parser, Subcommand};
use colored::Colorize;
use prettytable::{row, Table};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering::{self};
use std::error::Error;

const API_KEY: &str = "uwLUcab8yUVedyyxI3tdxEvodNjZaCnF89Tipih8";
const SEARCH_URL: &str = "https://api.nal.usda.gov/fdc/v1/foods/search";
const BASIC_NUTRIENTS: [&str; 5] = [
    "Energy",
    "Protein",
    "Total lipid (fat)",
    "Carbohydrate, by difference",
    "Fiber, total dietary",
];

#[derive(Deserialize, Debug)]
struct Nutrient {
    nutrientName: String,
    value: f64,
    unitName: String,
}

#[derive(Deserialize, Debug)]
struct Food {
    description: String,
    fdcId: u32,
    foodNutrients: Vec<Nutrient>,
}

#[derive(Deserialize, Debug)]
struct FoodResponse {
    foods: Vec<Food>,
}

/// A CLI for calculating a foods macros
#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}
#[derive(Subcommand, Debug)]
enum Commands {
    /// get the macros for a food (e.g. 100g of chicken breast)
    Food {
        // #[clap(short, long)]
        /// The name of the food
        name: String,

        /// Optionally set a quantity (e.g. 100g)
        #[clap(short, long)]
        quantity: Option<String>,

        /// If the nutrition data should be detailed
        #[clap(short, long, default_value = "false")]
        detailed: bool,
    },
    File {
        /// A file to read from with each line being a quantity of a food item (e.g. 100g chicken breast)
        #[clap(short, long)]
        file: String,

        /// If the nutrition data should be detailed
        #[clap(short, long, default_value = "false")]
        detailed: bool,
    },
    SetApiKey {
        /// The FoodData Central API key to use
        key: String,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.cmd {
        Commands::Food {
            name,
            quantity,
            detailed,
        } => {
            let key = get_key()?;
            let food = fetch_food(&key, &name).unwrap();
            print_table(food, detailed);
        }
        Commands::File { file, detailed } => {
            println!("File: {}", file);
        }
        Commands::SetApiKey { key } => {
            set_api_key(&key)?;
        }
    }
    Ok(())
}

#[derive(Default, Deserialize, Serialize)]
struct Config {
    api_key: Option<String>,
}

fn get_key() -> Result<String, Box<dyn Error>> {
    let config: Config = confy::load("macros", None)?;
    let msg = format!(
        "No API key set! Get your key on {}, and set it using {}",
        "https://fdc.nal.usda.gov/api-guide.html".green(),
        "macros set-api-key <KEY>".green()
    );

    let key = config.api_key.expect(&msg);

    Ok(key)
}

fn fetch_food(api_key: &str, food_name: &str) -> Result<Food, Box<dyn Error>> {
    let client = Client::new();

    // Include the pageSize parameter to limit results to 1
    let request_url = format!(
        "{}?query={}&pageSize=1&api_key={}&servingSize=100&servingSizeUnit=g&dataType=Foundation&description=Standard",
        SEARCH_URL, food_name, api_key
    );
    let response = client.get(&request_url).send()?;

    let food_response: FoodResponse = response.json()?;

    let food = food_response
        .foods
        .into_iter()
        .next()
        .ok_or("No food found")?;

    Ok(food)
}

fn print_table(food: Food, detailed: bool) {
    let mut table = Table::new();

    println!("Food: {}", food.description.bold().green());
    println!("FDC ID: {}", food.fdcId);
    println!();

    table.add_row(row!["Nutrient".bold(), "Value".bold(), "Unit".bold(),]);

    let mut nutriens = if detailed {
        food.foodNutrients
    } else {
        food.foodNutrients
            .into_iter()
            .filter(|nutrient| BASIC_NUTRIENTS.contains(&&nutrient.nutrientName[..]))
            .collect()
    };
    // make energy first
    nutriens.sort_by(|a, b| {
        Ordering::Less
            .then_with(|| Ordering::Greater)
            .then_with(|| Ordering::Equal)
    });

    for nutrient in nutriens {
        table.add_row(row![
            nutrient.nutrientName.blue().bold(),
            nutrient.value.to_string().green().bold(),
            nutrient.unitName,
        ]);
    }

    table.printstd();
}

fn set_api_key(api_key: &str) -> Result<(), Box<dyn Error>> {
    let config = Config {
        api_key: Some(api_key.to_string()),
    };

    confy::store("macros", None, config)?;
    println!("API key set!");
    Ok(())
}
