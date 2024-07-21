# fitff
CLI for automating changes to device, and manufacturer fields on fit files.

# Goal
Be able to download fit files from Zwift, and convert them to show being recorded by my Garmin device so they are counted for challenges when uploaded to Garmin Connect.

This is tailored to my watch and manufacturer for convenience, but there is general instruction to find your ProductID if you don't want to use enduro2.

## Usage
See the help menu for general options:
```
fitff --help
```

Convert a file using the default device and manufacturer:
```
fitff --input-file ~/Downloads/{name-of-file}.fit
```

NOTE:
To run from code replace `fitff` with `cargo run --`
example:
```
cargo run -- --input-file test.fit
```

## Install
Only supported install method is through cargo currently

```
cargo install fitff --locked
```

If you clone the repo you can use:
```
cargo install --path . --locked --profile release
```
