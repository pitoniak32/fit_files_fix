
use std::{fs::File, path::PathBuf};
use clap::Parser;
use reqwest::{blocking::multipart, Error};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct FixedFitFileResponse {
    message: String,
    file: String,
    id: String,
    ext_data: Option<String>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// enduro2 = 4341
    #[arg(short, long, default_value = "4341")]
    devicetype: u64,

    /// garmin = 1
    #[arg(short, long, default_value = "1")]
    manufacturer: u64,

    /// Location of the fit file that you would like to update.
    #[arg(short, long)]
    input_file: String,

    /// Location of where you would like the fixed fit file to be downloaded.
    #[arg(short, long)]
    output_file: PathBuf,
}

fn main() -> Result<(), Error>{
    let args = Args::parse();

    // enduro2 = 4341, garmin = 1
    let fitfiletools = format!("https://www.fitfiletools.com/tools/devicechanger?devicetype={}&mfgr={}", args.devicetype, args.manufacturer);

    // TODO: add env_logger, and verbosity flags
    // TODO: do some checks on the input and output file paths to make sure they are valid.

    let client = reqwest::blocking::Client::new();
    let res = client.post(fitfiletools).multipart(multipart::Form::new().file("file", args.input_file).unwrap()).send()?;
    let json_body: FixedFitFileResponse = serde_json::from_str(&res.text().unwrap()).unwrap();

    println!("body = {:#?}", json_body);

    let mut res = client.get(json_body.file).send()?;
    let mut file = File::create(&json_body.id).expect("file should be able to get created");
    std::io::copy(&mut res, &mut file).expect("file was copied");

    println!("file was downloaded to: {}", &json_body.id);

    Ok(())
}
