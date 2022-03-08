use std::error::Error;
use std::fs::File;
use std::io::copy;
use std::path::Path;

pub fn encode(input: &Path, output: &Path) -> Result<(), Box<dyn Error>> {
    let mut input = File::open(input)?;
    let mut output = File::create(output)?;

    let mut encoder = base64::write::EncoderWriter::new(&mut output, base64::STANDARD);
    let _ = copy(&mut input, &mut encoder)?;

    Ok(())
}

pub fn decode(input: &Path, output: &Path) -> Result<(), Box<dyn Error>> {
    let mut input = File::open(input)?;
    let mut output = File::create(output)?;

    let mut decoder = base64::read::DecoderReader::new(&mut input, base64::STANDARD);
    let _ = copy(&mut decoder, &mut output)?;

    Ok(())
}
