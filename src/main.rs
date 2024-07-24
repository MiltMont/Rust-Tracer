use std::{
    fs::File,
    io::{BufWriter, Write},
};

use rust_tracer::image::{sample_image, PPM};

fn main() -> std::io::Result<()> {
    let mut buffer = BufWriter::new(File::create("sample.ppm")?);
    let img = sample_image();
    write!(buffer, "{}", PPM(&img))?;
    buffer.flush()?;
    println!("Successfully generated the sample image.");

    Ok(())
}
