use std::{env, error::Error, io::{self, Write}, thread, time::Duration};
use reqwest::blocking::get;
use serde::Deserialize;

// Structs to handle openweather json response
#[derive(Deserialize)]
struct Weather {
    main: Main,
    weather: Vec<WeatherDesc>,
}

#[derive(Deserialize)]
struct Main {
    temp: f64,
}

#[derive(Deserialize)]
struct WeatherDesc {
    main: String,
    icon: String,
}

// Map conditions with emojis from Font Awesome (or any other polybar-friendly font)
fn weather_icon(main: &str, icon_code: &str) -> &'static str {
    let night = icon_code.ends_with('n');

    match main {
        "Clear" => {
            if night {
                "" // nf-fa-moon
            } else {
                "" // nf-fa-sun
            }
        }
        "Clouds" => {
            if night {
                "󰼱" // nf-fa-cloud
            } else {
                "" // nf-fa-sun_o
            }
        }
        "Drizzle" => "",      // nf-weather-showers
        "Rain" => "",         // nf-fa-tint
        "Thunderstorm" => "", // nf-fa-bolt
        "Snow" => "",         // nf-fa-snowflake_o
        "Mist" | "Fog" => "", // nf-weather-fog
        "Smoke" => "",        // nf-weather-smoke
        "Haze" => "",         // nf-weather-fog
        "Dust" | "Sand" => "", // nf-weather-dust
        "Ash" => "🌋",          // fallback emoji or alt icon
        "Squall" | "Tornado" => "", // nf-weather-tornado
        _ => "",              // nf-fa-question
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let api_key = env::var("OPENWEATHER_API_KEY")
        .expect("OPENWEATHER_API_KEY must be set");
    let city = "Markham";

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );

    let resp = reqwest::blocking::get(&url)?.json::<Weather>()?;

    let temp = resp.main.temp;
    let desc = &resp.weather[0].main;
    let icon = weather_icon(&resp.weather[0].main, &resp.weather[0].icon);
    
    // This handles how polybar displays the weather
    // println!("{} {:.1}°C - {}", icon, temp, desc);    
    println!("{} {:.1}°C", icon, temp);

    Ok(())
}

