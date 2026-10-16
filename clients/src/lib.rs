use anyhow::{Context, Ok, Result};
use reqwest::{self, blocking::RequestBuilder};
use serde_json::Value;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Weather {
    temperature: f64,
    summary: String,
}

impl Display for Weather {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:.1}ºC", self.summary, self.temperature)
    }
}

pub fn get_weather(location: &str, api_key: &str) -> Result<Weather> {
    let resp = request(location, api_key).send()?;
    let weather = deserialize(&resp.text()?)?;
    Ok(weather)
}

fn request(location: &str, api_key: &str) -> RequestBuilder {
    reqwest::blocking::Client::new()
        .get("https://api.weatherstack.com/current")
        .query(&[("query", location), ("access_key", api_key)])
}

fn deserialize(json: &str) -> Result<Weather> {
    let val: Value = serde_json::from_str(json)?;
    let temperature = val
        .pointer("/current/temperature")
        .and_then(Value::as_f64)
        .with_context(|| format!("bad response: {val}"))?;

    let summary = val
        .pointer("/current/weather_descriptions/0")
        .and_then(Value::as_str)
        .with_context(|| format!("bad response: {val}"))?
        .to_string();

    Ok(Weather {
        temperature,
        summary,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use url::Host::Domain;

    #[test]
    fn request_builds_correct_request() {
        let req = request("London, UK", "dummy API key");
        let req = req.build().unwrap();
        assert_eq!(req.method(), "GET", "wrong method");
        let url = req.url();
        assert_eq!(
            url.host(),
            Some(Domain("api.weatherstack.com")),
            "wrong host"
        );
        assert_eq!(url.path(), "/current", "wrong path");
        let params: Vec<(_, _)> = url.query_pairs().collect();
        assert_eq!(
            params,
            vec![
                ("query".into(), "London, UK".into()),
                ("access_key".into(), "dummy API key".into())
            ],
            "wrong params"
        );
    }

    #[test]
    fn deserialize_extracts_correct_weather_from_json() {
        let json = fs::read_to_string("demo.json").unwrap();
        let weather = deserialize(&json).unwrap();

        assert_eq!(
            weather,
            Weather {
                temperature: 15.0,
                summary: "Light rain".into()
            },
            "wrong weather"
        )
    }
}
