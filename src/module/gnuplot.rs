use anyhow::Context as _;
use std::fs;
use std::process::*;

pub fn write_png<P>(dir_img: P, png_name: P, csv_name: P) -> anyhow::Result<()>
where
    P: AsRef<str>, // as_ref() -> &str
{
    fs::create_dir_all(dir_img.as_ref()).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    Command::new("gnuplot")
        .arg("-e")
        .arg(r#"set terminal png;"#)
        .arg("-e")
        .arg(r#"set datafile separator ",""#)
        .arg("-e")
        .arg(r#"set ticslevel 0;"#)
        .arg("-e")
        .arg(r#"set dgrid3d 100,100;"#)
        .arg("-e")
        .arg(format!(r#"set zrange [{}:{}]"#, -1.0, 1.0))
        .arg("-e")
        .arg(format!(r#"set output "{}""#, png_name.as_ref()))
        .arg("-e")
        // csv plot
        .arg(format!(r#"splot "{}" u 1:2:3 with lines;"#, csv_name.as_ref()))
        .output()
        // mp4
        .context("failed to start `ffmpeg`")?;

    Ok(())
}
