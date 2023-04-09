use serde::{Deserialize, Serialize};
use reqwest::blocking::get;
use std::error::Error;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
struct Route {
    CMSID: String,
    Name: String,
    ZoomLevel: Option<i32>,
    OffsetY: Option<String>,
    OffsetX: Option<String>,
}

// https://md.hecke.rs/m87XCW1ZT6i4GaVW--eaog#

fn main() -> Result<(), Box<dyn Error>> {

    let route_list_url: &str = "https://maps.amtrak.com/rttl/js/RoutesList.json";
    let resp = get(route_list_url)?.text()?;

    let route_list: Vec<Route> = serde_json::from_str(&resp).unwrap();


    let mut zoom_total: i32 = 0;

    for route in route_list {

        println!("{:?}", route);

        zoom_total += route.ZoomLevel.unwrap_or(0);
    }

    println!("Zoom Total: {zoom_total}");

    Ok(())
}
