use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

use crate::NX;
use crate::NY;

// pub fn write_csv(dir_path: String, file_path: String, u: [[f64; 100]; 100]) -> Result<(), Box<dyn error::Error>> {
pub fn write_csv<P>(dir_path: P, file_path: P, u: [[f64; 100]; 100]) -> anyhow::Result<()>
where
    P: AsRef<str>,
{
    // create directory
    fs::create_dir_all(dir_path.as_ref()).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    let file = File::create(file_path.as_ref())?;

    log::debug!("{:?}", file);
    // dbg!(&file);

    let mut w = BufWriter::new(file);
    write!(w, "x,y,u\n")?;

    for i in 0..NX {
        for j in 0..NY {
            let s = format!("{},{},{}\n", i, j, u[i][j],);
            write!(w, "{}", s)?;
        }
    }
    w.flush()?;
    Ok(())
}
