use colored::*;
use serde::Deserialize;
use std::{io};

// creating a struct to deserialize the json response from openweather map api
#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

// struct to rep weather description
#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}
// struct to represent the main weather parameters
#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: f64,
    pressure: f64,
}
//struct to rep wind information
#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}
//function to get weather information from openweathermap api
fn get_weather_info(
    city: &str,
    country_code: &str,
    api_key: &str,
) -> Result<WeatherResponse, reqwest::Error> {
    let url: String = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}",
        city, country_code, api_key
    );

    let response = reqwest::blocking::get(&url)?;
    let response_json: WeatherResponse = response.json::<WeatherResponse>()?;
    Ok(response_json)
}

//function to display weather info
fn display_weather_info(response: &WeatherResponse) {
    let description: &str = &response.weather[0].description;
    let temperature: f64 = response.main.temp;
    let humidity: f64 = response.main.humidity;
    let pressure: f64 = response.main.pressure;
    let wind_speed: f64 = response.wind.speed;

    let weather_text: String = format!(
        "Weather in {}: {} \
        \n>Temperature: {} {:.1}°C, \
        \n>Humidity: {:.1}%, \
        \n>Pressure: {:.1} hPa, \
        \n>Wind Speed: {:.1} m/s",
        response.name,
        description,
        get_temp_emoji(temperature),
        temperature,
        humidity,
        pressure,
        wind_speed,
    );

    let weather_text_colored = match description {
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => {
            weather_text.dimmed()
        }
        "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
        _ => weather_text.normal(),
    };

    println!("{}", weather_text_colored);
}

fn get_temp_emoji(temperature: f64) -> &'static str {
    if temperature < 0.0 {
        "❄️"
    } else if temperature >= 0.0 && temperature < 10.0 {
        "☁️"
    } else if temperature >= 10.0 && temperature < 20.0 {
        "🌥️"
    } else if temperature >= 20.0 && temperature < 30.0 {
        "🌤️"
    } else {
        "🌞"
    }
}

fn main() {
    println!("{}", "Welcome to the weather Station!".bright_yellow());
    loop {
        println!("{}", "please enter the name of the city:".bright_green());
        let mut city = String::new();
        io::stdin().read_line(&mut city).expect("failed to read!");
        let city: &str = city.trim();

        println!(
            "{}",
            "please enter the country code (e.g., US for United States)".bright_green()
        );
        let mut country_code = String::new();
        io::stdin()
            .read_line(&mut country_code)
            .expect("failed to read input");
        let country_code = country_code.trim();

        let api_key = "b422958aa055173bb0e30776c21d88f7";
        match get_weather_info(city, country_code, api_key) {
            Ok(response) => {
                display_weather_info(&response);
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
        println!(
            "{}",
            "Do you want to search for weather in another city? (yes/no):".bright_green()
        );
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim().to_lowercase();

        if input != "yes" {
            println!("Thank you for using our software!");
            break;
        }
    }
}
