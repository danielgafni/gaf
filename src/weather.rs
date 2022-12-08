use serde::*;
use std::collections::HashMap;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct Sys {
    #[serde(rename = "type")]
    pub r#type: i64,
    pub id: i64,
    pub country: String,
    pub sunrise: i64,
    pub sunset: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Clouds {
    pub all: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wind {
    pub speed: f64,
    pub deg: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Main {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: i64,
    pub humidity: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    pub id: i64,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Coord {
    pub lon: f64,
    pub lat: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherInfo {
    pub coord: Coord,
    pub weather: Vec<Weather>,
    pub base: String,
    pub main: Main,
    pub visibility: i64,
    pub wind: Wind,
    pub clouds: Clouds,
    pub dt: i64,
    pub sys: Sys,
    pub timezone: i64,
    pub id: i64,
    pub name: String,
    pub cod: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherPrettyInfo {
    pub icon: String,
    pub quote: String,
    pub advice: String,
    pub temp: f64,
    pub hex: String,
    pub description: String,
}

impl WeatherPrettyInfo {
    pub fn from_weather_info(weather_info: WeatherInfo) -> Box<WeatherPrettyInfo> {
        let weather_codes_map: HashMap<&str, (&str, &str, &str, &str)> = HashMap::from([
            (
                "50d",
                (
                    " ",
                    "#84afdb",
                    "Forecast says it's misty",
                    "Make sure you don't get lost on your way...",
                ),
            ),
            (
                "50n",
                (
                    " ",
                    "#84afdb",
                    "Forecast says it's a misty night",
                    "Don't go anywhere tonight or you might get lost...",
                ),
            ),
            (
                "01d",
                (
                    " ",
                    "#ffd86b",
                    "It's a sunny day, gonna be fun!",
                    "Don't go wandering all by yourself though...",
                ),
            ),
            (
                "01n",
                (
                    "",
                    "#fcdcf6",
                    "It's a clear night",
                    "You might want to take a evening stroll to relax...",
                ),
            ),
            (
                "02d",
                (
                    "",
                    "#adadff",
                    "It's  cloudy, sort of gloomy",
                    "You'd better get a book to read...",
                ),
            ),
            (
                "02n",
                (
                    "",
                    "#adadff",
                    "It's a cloudy night",
                    "How about some hot chocolate and a warm bed?",
                ),
            ),
            (
                "03d",
                (
                    "",
                    "#adadff",
                    "It's  cloudy, sort of gloomy",
                    "You'd better get a book to read...",
                ),
            ),
            (
                "03n",
                (
                    "",
                    "#adadff",
                    "It's a cloudy night",
                    "How about some hot chocolate and a warm bed?",
                ),
            ),
            (
                "04d",
                (
                    "",
                    "#adadff",
                    "It's  cloudy, sort of gloomy",
                    "You'd better get a book to read...",
                ),
            ),
            (
                "04n",
                (
                    "",
                    "#adadff",
                    "It's a cloudy night",
                    "How about some hot chocolate and a warm bed?",
                ),
            ),
            (
                "09d",
                (
                    "",
                    "#6b95ff",
                    "It's rainy, it's a great day!",
                    "Get some ramen and watch as the rain falls...",
                ),
            ),
            (
                "09n",
                (
                    "",
                    "#6b95ff",
                    " It's gonna rain tonight it seems",
                    "Make sure your clothes aren't still outside...",
                ),
            ),
            (
                "10d",
                (
                    "",
                    "#6b95ff",
                    "It's rainy, it's a great day!",
                    "Get some ramen and watch as the rain falls...",
                ),
            ),
            (
                "10n",
                (
                    "",
                    "#6b95ff",
                    " It's gonna rain tonight it seems",
                    "Make sure your clothes aren't still outside...",
                ),
            ),
            (
                "11d",
                (
                    "",
                    "#ffeb57",
                    "There's storm for forecast today",
                    "Make sure you don't get blown away...",
                ),
            ),
            (
                "11n",
                (
                    "",
                    "#ffeb57",
                    "There's gonna be storms tonight",
                    "Make sure you're warm in bed and the windows are shut...",
                ),
            ),
            (
                "13d",
                (
                    "",
                    "#e3e6fc",
                    "It's gonna snow today",
                    "You'd better wear thick clothes and make a snowman as well!",
                ),
            ),
            (
                "13n",
                (
                    "",
                    "#e3e6fc",
                    "It's gonna snow tonight",
                    "Make sure you get up early tomorrow to see the sights...",
                ),
            ),
            (
                "40d",
                (
                    "",
                    "#84afdb",
                    "Forecast says it's misty",
                    "Make sure you don't get lost on your way...",
                ),
            ),
            (
                "40n",
                (
                    "",
                    "#84afdb",
                    "Forecast says it's a misty night",
                    "Don't go anywhere tonight or you might get lost...",
                ),
            ),
        ]);

        let weather_icon = weather_info.weather[0].icon.as_str();
        let description = weather_info.weather[0].description.clone();

        let unknown_weather_id_quote = format!(
            "Sort of odd, I don't know what to forecast for {}...",
            weather_icon
        );

        let (icon, hex, quote, advice) = weather_codes_map
            .get(weather_icon)
            .unwrap_or(&(
                "",
                "#84afdb",
                unknown_weather_id_quote.as_str(),
                "Make sure you have a good time!",
            ))
            .clone();

        return Box::new(WeatherPrettyInfo {
            icon: String::from(icon),
            quote: String::from(quote),
            advice: String::from(advice),
            temp: weather_info.main.temp - 273.15, // to Celsius
            hex: String::from(hex),
            description,
        });
    }
}

pub fn get_current_weather() -> String {
    let lat = env::var("LAT").unwrap().parse::<f32>().unwrap();
    let lon = env::var("LON").unwrap().parse::<f32>().unwrap();
    let openweather_key = env::var("OPENWEATHER_KEY").unwrap();

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}",
        lat, lon, openweather_key
    );
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    let weather_info: WeatherInfo = serde_json::from_str(&body.as_str()).unwrap();
    // let pretty_weather_info = *;
    serde_json::to_string(&WeatherPrettyInfo::from_weather_info(weather_info)).unwrap()
}
