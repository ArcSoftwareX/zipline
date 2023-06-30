use flate2::{
    write::{GzDecoder, GzEncoder},
    Compression,
};
use std::{
    fs::{remove_file, File},
    io::{copy, BufReader},
};

pub fn compress_file(path: &String) -> Result<(), std::io::Error> {
    let mut reader = BufReader::new(File::open(path)?);
    let filename = transform_name(path);
    let output = File::create(filename)?;

    let mut encoder = GzEncoder::new(output, Compression::default());
    copy(&mut reader, &mut encoder)?;

    encoder.finish()?;

    remove_file(path)?;

    Ok(())
}

fn transform_name(name: &str) -> String {
    format!("{}.gz", name)
}

fn transform_decoded_name(name: &str) -> String {
    let name = name.split('.');
    name.clone()
        .take(name.count() - 1)
        .fold("".to_string(), |acc, val| format!("{acc}{val}."))
}

pub fn decompress_file(path: &String) -> Result<(), std::io::Error> {
    let mut reader = BufReader::new(File::open(path)?);
    let filename = transform_decoded_name(path);
    let output = File::create(filename)?;

    let mut decoder = GzDecoder::new(output);
    copy(&mut reader, &mut decoder)?;

    decoder.finish()?;

    remove_file(path)?;

    Ok(())
}

pub fn check_if_gz(path: &str) -> bool {
    path.ends_with(".gz")
}
