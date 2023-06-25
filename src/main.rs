
use std::fs::File;

use reqwest::{blocking::multipart, Error};
use serde::Deserialize;
use url;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct FixedFitFileResponse {
    message: String,
    file: String,
    id: String,
    ext_data: Option<String>,
}

fn main() -> Result<(), Error>{
    // enduro2 = 4341, garmin = 1
    let fitfiletools = format!("https://www.fitfiletools.com/tools/devicechanger?devicetype={}&mfgr={}", 4341, 1);

    let client = reqwest::blocking::Client::new();
    let res = client.post(fitfiletools).multipart(multipart::Form::new().file("file", "/home/davidpi/Downloads/ff.fit").unwrap()).send()?;
    let json_body: FixedFitFileResponse = serde_json::from_str(&res.text().unwrap()).unwrap();

    println!("body = {:#?}", json_body);

    let mut res = client.get(json_body.file).send()?;
    let mut file = File::create(json_body.id).expect("file should be able to get created");
    std::io::copy(&mut res, &mut file).expect("file was copied");

    println!("res = {res:#?}");

    Ok(())
}
