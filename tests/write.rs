extern crate wrap_archive;

use std::error::Error;
use std::fs::File as StdFile;
use std::io::{
    Read,
    Write,
};

use wrap_archive::blake3::Hasher;
use wrap_archive::{
    Compression,
    Writer,
};

#[test]
fn test_writer_image_compressed() -> Result<(), Box<dyn Error>> {
    let file = StdFile::create("tests/out/image_c.wrap")?;
    let mut writer = Writer::new(file)?;
    let mut var_file = writer.write_file("marbles.bmp", Compression::default())?;
    let mut input_file = StdFile::open("tests/files/marbles.bmp")?;
    std::io::copy(&mut input_file, &mut var_file)?;
    Ok(())
}

#[test]
fn test_writer_image_uncompressed() -> Result<(), Box<dyn Error>> {
    let file = StdFile::create("tests/out/image_u.wrap")?;
    let mut writer = Writer::new(file)?;
    let mut var_file = writer.write_file("marbles.bmp", Compression::None)?;
    let mut input_file = StdFile::open("tests/files/marbles.bmp")?;
    std::io::copy(&mut input_file, &mut var_file)?;
    Ok(())
}

#[test]
fn test_writer_text_compressed() -> Result<(), Box<dyn Error>> {
    let file = StdFile::create("tests/out/text_c.wrap")?;
    let mut writer = Writer::new(file)?;
    let mut var_file = writer.write_file("bsd.md", Compression::default())?;
    let mut input_file = StdFile::open("tests/files/bsd.md")?;
    std::io::copy(&mut input_file, &mut var_file)?;
    Ok(())
}

#[test]
fn test_writer_text_uncompressed() -> Result<(), Box<dyn Error>> {
    let file = StdFile::create("tests/out/text_u.wrap")?;
    let mut writer = Writer::new(file)?;
    let mut var_file = writer.write_file("bsd.md", Compression::None)?;
    let mut input_file = StdFile::open("tests/files/bsd.md")?;
    std::io::copy(&mut input_file, &mut var_file)?;
    Ok(())
}

#[test]
fn test_writer_csv_compressed() -> Result<(), Box<dyn Error>> {
    let file = StdFile::create("tests/out/csv_c.wrap")?;
    let mut writer = Writer::new(file)?;
    let mut var_file = writer.write_file("sales_records.csv", Compression::default())?;
    let mut input_file = StdFile::open("tests/files/sales_records.csv")?;
    std::io::copy(&mut input_file, &mut var_file)?;
    Ok(())
}

#[test]
fn test_writer_csv_uncompressed() -> Result<(), Box<dyn Error>> {
    let file = StdFile::create("tests/out/csv_u.wrap")?;
    let mut writer = Writer::new(file)?;
    let mut var_file = writer.write_file("sales_records.csv", Compression::None)?;
    let mut input_file = StdFile::open("tests/files/sales_records.csv")?;
    std::io::copy(&mut input_file, &mut var_file)?;
    Ok(())
}

#[test]
fn test_writer_mixed_compressed() -> Result<(), Box<dyn Error>> {
    let file = StdFile::create("tests/out/mixed_c.wrap")?;
    let mut writer = Writer::new(file)?;
    {
        let mut var_file = writer.write_file("sales_records.csv", Compression::default())?;
        let mut input_file = StdFile::open("tests/files/sales_records.csv")?;
        std::io::copy(&mut input_file, &mut var_file)?;
    }
    {
        let mut var_file = writer.write_file("bsd.md", Compression::default())?;
        let mut input_file = StdFile::open("tests/files/bsd.md")?;
        std::io::copy(&mut input_file, &mut var_file)?;
    }
    {
        let mut var_file = writer.write_file("marbles.bmp", Compression::default())?;
        let mut input_file = StdFile::open("tests/files/marbles.bmp")?;
        std::io::copy(&mut input_file, &mut var_file)?;
    }
    Ok(())
}

#[test]
fn test_writer_mixed_uncompressed() -> Result<(), Box<dyn Error>> {
    let file = StdFile::create("tests/out/mixed_u.wrap")?;
    let mut writer = Writer::new(file)?;
    {
        let mut var_file = writer.write_file("sales_records.csv", Compression::None)?;
        let mut input_file = StdFile::open("tests/files/sales_records.csv")?;
        std::io::copy(&mut input_file, &mut var_file)?;
    }
    {
        let mut var_file = writer.write_file("bsd.md", Compression::None)?;
        let mut input_file = StdFile::open("tests/files/bsd.md")?;
        std::io::copy(&mut input_file, &mut var_file)?;
    }
    {
        let mut var_file = writer.write_file("marbles.bmp", Compression::None)?;
        let mut input_file = StdFile::open("tests/files/marbles.bmp")?;
        std::io::copy(&mut input_file, &mut var_file)?;
    }
    Ok(())
}
