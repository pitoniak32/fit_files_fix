use anyhow::Result;
use clap::Parser;
use clap_verbosity_flag::LevelFilter;
use reqwest::blocking::multipart;
use serde::Deserialize;
use std::fs::File;

fn main() -> Result<()> {
    let args = Args::parse();

    env_logger::builder()
        .filter(Some("fitff"), LevelFilter::Info)
        .filter_level(args.verbosity.log_level_filter())
        .parse_default_env()
        .init();

    let fitfiletools = format!(
        "https://www.fitfiletools.com/tools/devicechanger?devicetype={}&mfgr={}",
        args.devicetype, args.manufacturer
    );
    log::debug!("calling: {}", fitfiletools);

    let client = reqwest::blocking::Client::new();

    let inputfile_path = std::path::Path::new(&args.input_file);
    log::debug!("checking if file exists at {}", &args.input_file);
    assert!(inputfile_path.exists(), "input file path needs to exist");

    let res = client
        .post(fitfiletools)
        .multipart(multipart::Form::new().file("file", args.input_file)?)
        .send()?;
    let json_body: FixedFitFileResponse = serde_json::from_str(&res.text()?)?;
    log::debug!("body = {:#?}", json_body);

    let output_file = format!(
        "{}_{}",
        &args.output_file.replace(".fit", ""),
        &json_body.id
    );
    log::debug!("file output path = {}", output_file);

    let mut res = client.get(json_body.file).send()?;
    let mut file = File::create(&output_file).expect("file should be able to be created");

    std::io::copy(&mut res, &mut file).expect("file was copied");
    log::info!("file was downloaded to: {}", &output_file);

    Ok(())
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct FixedFitFileResponse {
    file: String,
    id: String,
    _message: String,
    _ext_data: Option<String>,
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

    /// ex: `/tmp/fit_file` will be downloaded to `/tmp/fit_file_{uuid}.fit`
    #[arg(short, long)]
    output_file: String,

    #[clap(flatten)]
    pub verbosity: clap_verbosity_flag::Verbosity,
}
