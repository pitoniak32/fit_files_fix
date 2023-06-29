use anyhow::Result;
use clap::Parser;
use glob::glob;
use reqwest::blocking::multipart;
use serde::Deserialize;
use std::{fs::File, path::PathBuf, vec};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    env_logger::builder()
        .filter_level(args.verbosity.log_level_filter())
        .parse_default_env()
        .init();

    let fitfiletools_api_uri = format!(
        "https://www.fitfiletools.com/tools/devicechanger?devicetype={}&mfgr={}",
        args.devicetype.clone(),
        args.manufacturer.clone()
    );

    let files = match get_search_strategy(args.input_file, args.search_dir, args.glob_pattern)
        .expect("This should not be possible, you must provide one parameter.")
    {
        SearchStrategy::Glob(glob_path) => expand_paths(glob_path),
        SearchStrategy::File(file_path) => vec![file_path],
    };

    for file in files {
        log::debug!("{file:#?}");
        // let uri = fitfiletools_api_uri.clone();
        // let output = args.output_file.clone();
        // tokio::spawn(async move {
        //     let inputfile_path = std::path::Path::new(&file);
        //     log::trace!("check if file exists at: {}", &file);
        //     assert!(inputfile_path.exists(), "input file path needs to exist");
        //
        //     let client = reqwest::blocking::Client::new();
        //     let res = client
        //         .post(uri.clone())
        //         .multipart(multipart::Form::new().file("file", file).unwrap())
        //         .send()
        //         .unwrap();
        //     let json_body: FixedFitFileApiResponse =
        //         serde_json::from_str(&res.text().unwrap()).unwrap();
        //     log::debug!("body = {:#?}", json_body);
        //
        //

        let output_file_path = get_output_file_path(
            get_output_dir(args.output_dir.clone(), file.clone()),
            args.output_prefix.clone(),
            PathBuf::from(file)
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string(),
            "123abc".to_string(),
        );
        log::debug!("out_path: {:#?}", output_file_path,);

        //
        //     let mut res = client.get(json_body.file).send().unwrap();
        //     let mut file = File::create(&output_file).expect("file should be able to be created");
        //     std::io::copy(&mut res, &mut file).expect("file was copied");
        //     log::info!("file was downloaded to: {}", &output_file);
        // });
    }

    Ok(())
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct FixedFitFileApiResponse {
    file: String,
    id: String,
    _message: String,
    _ext_data: Option<String>,
}

/// Usage: this that and the other thing
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(flatten)]
    pub verbosity: clap_verbosity_flag::Verbosity,

    /// enduro2 = 4341
    #[arg(short, long, default_value = "4341")]
    devicetype: u64,

    /// garmin = 1
    #[arg(short, long, default_value = "1")]
    manufacturer: u64,

    /// Location of the fit file that you would like to update.
    #[arg(short, long)]
    input_file: Option<String>,

    /// Glob pattern that matches all of the fit files that you would like to update.
    #[arg(short, long, requires = "search_dir", conflicts_with = "input_file")]
    glob_pattern: Option<String>,

    /// Glob pattern that matches all of the fit files that you would like to update.
    #[arg(short, long, requires = "glob_pattern", conflicts_with = "input_file")]
    search_dir: Option<String>,

    /// ex: `fit_file` will be downloaded to `fit_file_{input_file_name}_{uuid}.fit`
    #[arg(long)]
    output_prefix: Option<String>,

    /// ex: `/tmp/fit_file/` will be downloaded to `/tmp/fit_file/{file_path}.fit`
    /// If this is not provided the input_file parent dir will be used.
    #[arg(long)]
    output_dir: Option<String>,
}

fn expand_paths(search_glob_path: String) -> Vec<String> {
    log::trace!("{:#?}", search_glob_path);
    let files: Vec<String> = glob(&search_glob_path)
        .unwrap()
        .map(|r| r.unwrap().to_str().unwrap().to_string())
        .collect();
    log::trace!("files: {files:#?}");
    files
}

enum SearchStrategy {
    Glob(String),
    File(String),
}

fn get_search_strategy(
    input_file: Option<String>,
    search_dir: Option<String>,
    glob_pattern: Option<String>,
) -> Option<SearchStrategy> {
    match (input_file, search_dir, glob_pattern) {
        (None, Some(dir), Some(glob)) => Some(SearchStrategy::Glob(
            PathBuf::from(dir).join(glob).to_string_lossy().to_string(),
        )),
        (Some(file), None, None) => Some(SearchStrategy::File(file)),
        (None, None, None) => None,
        _ => None,
    }
}

fn get_output_file_path(
    dir: PathBuf,
    prefix: Option<String>,
    file_name: String,
    id: String,
) -> PathBuf {
    dir.join(format!(
        "{}{}_{}.fit",
        prefix
            .and_then(|p| Some(format!("{p}_")))
            .unwrap_or_default(),
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
    mod get_search_strategy {
        use super::super::*;

        #[test]
        fn should_return_none_when_all_params_are_none() {
            // Arrange / Act / Assert
            assert!(get_search_strategy(None, None, None).is_none())
        }

        #[test]
        fn should_return_file_when_dir_and_glob_are_some() {
            // Arrange / Act / Assert
            assert!(
                get_search_strategy(Some("/path/search/dir".to_string()), None, None)
                    .is_some_and(|r| { matches!(r, SearchStrategy::File(..)) })
            )
        }

        #[test]
        fn should_return_glob_when_dir_and_glob_are_some() {
            // Arrange / Act / Assert
            assert!(get_search_strategy(
                None,
                Some("/path/search/dir".to_string()),
                Some("glob_*.toml".to_string())
            )
            .is_some_and(|r| { matches!(r, SearchStrategy::Glob(..)) }))
        }
    }

    #[cfg(test)]
    mod expand_paths {
        use super::super::*;

        #[test]
        fn should_expand_glob() {
            // Arrange
            let cwd = std::env::current_dir().expect("should be able to get cwd");

            // Act
            let result = expand_paths(format!(
                "{}{}*.toml",
                cwd.to_str().expect("should be able to be converted to str"),
                std::path::MAIN_SEPARATOR_STR,
            ));

            // Assert
            assert_eq!(result, [cwd.join("Cargo.toml").to_str().unwrap()]);
        }
    }

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
