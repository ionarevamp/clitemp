extern crate reqwest as rq;
extern crate serde_json;
extern crate serde;
extern crate clap;

use clap::Parser;

use std::fs;
//use std::io::Write;
//use std::str::FromStr;
use std::error::Error;

use crate::serde::Deserialize;

#[derive(Debug, Deserialize)]
struct FieldCondition {
	text: String,
}
#[derive(Debug, Deserialize)]
struct FieldAirQuality {
	co: f64,
	no2: f64,
	o3: f64,
	so2: f64,
	pm2_5: f64,
	pm10: f64,
	#[serde(rename = "us-epa-index")]
	us_epa_index: f64,
	#[serde(rename = "gb-defra-index")]
	gb_defra_index: f64,
}

#[derive(Debug, Deserialize)]
struct FieldCurrent {
	temp_c: f64,
	temp_f: f64,
	feelslike_c: f64,
	feelslike_f: f64,
	wind_mph: f64,
	wind_kph: f64,
	cloud: f64,
	uv: f64,
	condition: FieldCondition,
	air_quality: FieldAirQuality,
}

#[derive(Debug, Deserialize)]
struct Data {
	current: FieldCurrent,
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Check weather for this zip code
    #[arg(short, long, required(false))]
    zip: Option<String>,
    /// Check weather using this key
    #[arg(short, long, required(false))]
    key: Option<String>,
}
/*
pub fn convert(mode: &mut char, temp: f64) -> f64 {
    return match *mode {
		'f' => (temp - 32.0) * (5.0/9.0),
		'c' => (temp * (9.0/5.0))+ 32.0,
		_ => f64::MAX,
    }
}
*/
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let exe_path = std::env::current_exe()
        .expect("Could not get path of running executable.");
    let parent_path = exe_path
        .parent()
        .unwrap();

    let key_path = parent_path.join("WeatherAPI.key");
    let zip_path = parent_path.join("zipcode.txt");
   
    let clapargs = Args::parse();
    
    let key = if let Some(val) = clapargs.key { val } else {
        fs::read_to_string(std::path::Path::new(&key_path)).unwrap_or_else( |_|
	    	panic!("Unable to read key from `WeatherAPI.key`. Exiting\n\r")
	    )
    };
    let zip = if let Some(val) = clapargs.zip { val } else {
        fs::read_to_string(std::path::Path::new(&zip_path)).unwrap_or_else( |_|
            panic!("Unable to read zip code from `zipcode.txt` Exiting\n\r")
        )
    };
	 
	// Get weather data as json from WeatherAPI.com using key and zip code
	let resp = rq::get(
		format!("http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=yes", key, zip).as_str())
        .await?
        .json::<Data>()
        .await?;
    
	let weather: &FieldCurrent = &resp.current;
	let air_quality: &FieldAirQuality = &weather.air_quality;
	let sky: &String = &weather.condition.text;

	print!("Weather is {}",sky);
	match weather.cloud as usize {
		0 => print!(""),
		(1..=50) => print!(" with some clouds"),
		(51..=100) => print!(" with significant clouds"),
		_ => print!(" with cloud value {}", weather.cloud),
	}
	println!(".");
	
	println!("Temperature in Celsius: {:.1} (Feels like {:.1})", weather.temp_c, weather.feelslike_c);
	println!("Temperature in Fahrenheit: {:.1} (Feels like {:.1})", weather.temp_f, weather.feelslike_f);
	println!("UV strength: {}", weather.uv);
	println!("Wind Speed: {}mph, {}kph", weather.wind_mph, weather.wind_kph );
	println!("Air Quality: \n\tCO: {:.1}\n\tNO2: {:.1}\n\tO3: {:.1}\n\tSM2: {:.1}\n\tPM2.5: {:.1}\n\tPM10: {:.1}\n\t\
			 US EPA Index: {}\n\tUK DEFRA index: {}",
			air_quality.co, air_quality.no2, air_quality.o3, air_quality.so2, air_quality.pm2_5,
			air_quality.pm10, air_quality.us_epa_index, air_quality.gb_defra_index );

    Ok(())
}


//Formula -- (32°F − 32) × 5/9 = °C
