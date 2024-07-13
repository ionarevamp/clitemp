extern crate reqwest as rq;
extern crate serde_json;
extern crate serde;

use std::fs;
use std::env;
use std::io::Write;
use std::str::FromStr;
use std::error::Error;

use std::collections::HashMap;
use crate::serde::{Deserialize};

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

pub fn convert(mode: &mut char, temp: f64) -> f64 {
    return match *mode {
		'f' => (temp - 32.0) * (5.0/9.0),
		'c' => (temp * (9.0/5.0))+ 32.0,
		_ => f64::MAX,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

	let key = fs::read_to_string("WeatherAPI.key").unwrap_or_else( |_|
		panic!("Unable to read key from `WeatherAPI.key`. Exiting\n")
	 );
	
	let mut args: Vec<String> = env::args().collect();
	//dbg!(&args);

	if args.len() < 2 { args.push("f".to_string()); }
    let mut modechar = args[1].trim().to_lowercase().chars().nth(0).unwrap_or('f');
    if modechar != 'f' && modechar != 'c' {
		println!("Invalid option. Assuming Fahrenheit.");
		modechar = 'f';
    }  
    	// ^ takes first character of input string or defaults to 'f' if invalid
    /*
    let mut temp: f64 = match f64::from_str(args[2].as_str().trim()) {
		Ok(num) => num,
		Err(_) => f64::MAX,
    };
	*/

	// Get weather data as json from WeatherAPI.com using key
	let resp = rq::get(
		format!("http://api.weatherapi.com/v1/current.json?key={}&q=83702&aqi=yes", key).as_str())
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
		_ => print!(" with cloud value {}", weather.cloud as usize),
	}
	println!(".");
	
	println!("Temperature in Celsius: {:.1} (Feels like {:.1})", weather.temp_c, weather.feelslike_c);
	println!("Temperature in Fahrenheit: {:.1} (Feels like {:.1})", weather.temp_f, weather.feelslike_f);
	println!("UV strength: {:.1}", weather.uv);
	println!("Wind Speed: {}mph, {}kph", weather.wind_mph, weather.wind_kph );
	println!("Air Quality: \n\tCO: {:.1}\n\tNO2: {:.1}\n\tO3: {:.1}\n\tSM2: {:.1}\n\tPM2.5: {:.1}\n\tPM10: {:.1}\n\t\
			 US EPA Index: {:.1}\n\tUK DEFRA index: {:.1}",
			air_quality.co, air_quality.no2, air_quality.o3, air_quality.so2, air_quality.pm2_5,
			air_quality.pm10, air_quality.us_epa_index, air_quality.gb_defra_index );

    Ok(())
    //println!("Temperature in {} is {:.1}°",temp_type,convert(&mut modechar, &mut temp));
    
}


//Formula -- (32°F − 32) × 5/9 = °C