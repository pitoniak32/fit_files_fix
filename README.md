# fitff
CLI for interacting with the fitfiletools API `https://www.fitfiletools.com/tools/devicechanger?devicetype={}&mfgr={}` to automate changes to device, and manufacturer fields on fit files.

# Goal
To download fit files from Zwift, and convert them to show being recorded by my Garmin device so they are counted for challenges when uploaded to Garmin Connect.

This is tailored to my watch and manufacturer for convenience, but there is general instruction to find your ProductID if you don't want to use enduro2.

# Demo
https://github.com/user-attachments/assets/80fe6071-324e-4171-be79-7f9b8ac554f8

## Usage
See the help menu for general options:
```
fitff --help
```

If you are converting Zwift this is my current recommended way (See Demo Video):
```
fitff --input-location ~/Downloads/ --cleanup
```
(This assumes the file name is the date of the activity `YYYY-MM-DD-*.fit`, but its my only usecase)

Convert file(s):
```
fitff --devicetype 4341 --manufacturer 1 --input-file ~/Downloads/{name-of-file}.fit --input-file ~/Downloads/{other-file}.fit
```

NOTE:
To run from code replace `fitff` with `cargo run --`
example:
```
cargo run -- --input-file test.fit
```

## Install

### Releases

See releases section.

### Cargo

```
cargo install fitff --locked
```

If you clone the repo you can use:
```
cargo install --path . --locked --profile release
```
