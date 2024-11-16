use anyhow::Result;

use std::{fs::File, path::PathBuf};

use reqwest::blocking::multipart;
use serde::Deserialize;

use clap::Parser;
use clap_verbosity_flag::InfoLevel;

/// Usage: this that and the other thing
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(flatten)]
    pub verbosity: clap_verbosity_flag::Verbosity<InfoLevel>,

    /// The ProductID of your device to change the file to
    ///
    /// Export an activity recorded with your device as TCX
    /// from Garmin Connect Search for ProductID.
    ///
    /// ex for enduro2:
    /// <ProductID>4341</ProductID>
    #[arg(short, long, default_value = "4341")]
    devicetype: u64,

    /// garmin = 1
    #[arg(short, long, env = "MFG", default_value = "1")]
    manufacturer: u64,

    /// Location of the fit file(s) that you would like to update.
    #[arg(short, long, required = true)]
    input_files: Vec<String>,

    /// ex: `fit_file` will be downloaded to `fit_file_{input_file_name}_{uuid}.fit`
    #[arg(long)]
    output_prefix: Option<String>,

    /// ex: `/tmp/fit_file/` will be downloaded to `/tmp/fit_file/{file_path}.fit`
    /// If this is not provided the input_file parent dir will be used.
    #[arg(long)]
    output_dir: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    env_logger::builder()
        .filter_level(args.verbosity.log_level_filter())
        .parse_default_env()
        .init();

    log::debug!("{:#?}", args.input_files);

    let fitfiletools_api_uri = format!(
        "https://www.fitfiletools.com/tools/devicechanger?devicetype={}&mfgr={}",
        args.devicetype.clone(),
        args.manufacturer.clone()
    );

    log::debug!("{fitfiletools_api_uri}");

    for file in args.input_files {
        log::debug!("{file:#?}");
        let inputfile_path = std::path::Path::new(&file);
        log::trace!("check if file exists at: {}", &file);
        assert!(inputfile_path.exists(), "input file path needs to exist");

        let client = reqwest::blocking::Client::new();
        let res = client
            .post(&fitfiletools_api_uri)
            .multipart(multipart::Form::new().file("file", file.clone()).unwrap())
            .send()
            .unwrap();
        let json_body: FixedFitFileApiResponse =
            serde_json::from_str(&res.text().unwrap()).unwrap();
        log::debug!("body = {:#?}", json_body);

        let output_file_path = get_output_file_path(
            get_output_dir(args.output_dir.clone(), file.clone()),
            args.output_prefix.clone(),
            PathBuf::from(file)
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string(),
            uuid::Uuid::new_v4().to_string(),
        );
        log::debug!("out_path: {:#?}", output_file_path,);

        let mut res = client.get(json_body.file).send().unwrap();
        let mut file = File::create(&output_file_path).expect("file should be able to be created");
        std::io::copy(&mut res, &mut file).expect("file was copied");
        log::info!(
            "file was downloaded to: {}",
            &output_file_path.to_string_lossy().to_string()
        );
    }

    Ok(())
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct FixedFitFileApiResponse {
    file: String,
    _id: String,
    _message: String,
    _ext_data: Option<String>,
}

fn get_output_file_path(
    dir: PathBuf,
    prefix: Option<String>,
    file_name: String,
    id: String,
) -> PathBuf {
    dir.join(format!(
        "{}{}_{}.fit",
        prefix.map(|p| format!("{p}_")).unwrap_or_default(),
        file_name.replace(".fit", ""),
        id
    ))
}

fn get_output_dir(output_dir: Option<String>, input_file: String) -> PathBuf {
    let dir = match output_dir {
        Some(o_dir) => PathBuf::from(o_dir),
        None => PathBuf::from(PathBuf::from(input_file).parent().unwrap()),
    };
    log::debug!("output_dir: {:#?}", dir);
    dir
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod get_output_dir {
        use super::super::*;

        #[test]
        fn should_use_output_dir_if_output_dir_is_provided() {
            // Arrange / Act / Assert
            assert_eq!(
                get_output_dir(
                    Some("path".to_string()),
                    "/input/file/path/file.fit".to_string(),
                ),
                PathBuf::from("path")
            )
        }

        #[test]
        fn should_use_input_file_parent_dir_if_output_dir_is_not_provided() {
            // Arrange / Act / Assert
            assert_eq!(
                get_output_dir(None, "/input/file/path/file.fit".to_string(),),
                PathBuf::from("/input/file/path/")
            )
        }
    }

    #[cfg(test)]
    mod get_output_file_path {
        use super::super::*;

        #[test]
        fn should_correctly_build_output_file_path_when_prefix_is_provied() {
            // Arrange / Act / Assert
            assert_eq!(
                get_output_file_path(
                    PathBuf::from("path"),
                    Some("prefix".to_string()),
                    "file.fit".to_string(),
                    "123abc".to_string()
                ),
                PathBuf::from("path").join("prefix_file_123abc.fit")
            )
        }

        #[test]
        fn should_correctly_build_output_file_path_when_prefix_is_not_provied() {
            // Arrange / Act / Assert
            assert_eq!(
                get_output_file_path(
                    PathBuf::from("path"),
                    None,
                    "file.fit".to_string(),
                    "123abc".to_string()
                ),
                PathBuf::from("path").join("file_123abc.fit")
            )
        }
    }
}
