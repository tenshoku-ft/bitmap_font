use std::env::var;
use std::fs::File;
use std::fs::read_dir;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;


fn main()->std::io::Result<()>{
	println!("cargo:rerun-if-changed=build.rs");

	let glyph_dir=Path::new("glyph");

	let out_dir=&var("OUT_DIR").unwrap();
	let out_dir=Path::new(out_dir);

	let mut ascii_bin=PathBuf::new();
	ascii_bin.push(out_dir);
	ascii_bin.push("ascii.bin");

	let mut fallback_glyph=PathBuf::new();
	fallback_glyph.push(glyph_dir);
	fallback_glyph.push("fallback.bin");
	let mut fallback_glyph=File::open(fallback_glyph)?;
	let mut fallback_bin=[0u8;16];
	fallback_glyph.read_exact(&mut fallback_bin[..]);

	let mut ascii_bin=File::create(ascii_bin.as_path())?;

	for i in 0..=127 {
		let filename=format!("{:02x}.bin",i);
		let mut filename={
			let mut a=PathBuf::new();
			a.push(glyph_dir);
			a.push(filename);
			a
		};
		if filename.as_path().exists() {
			println!("cargo:rerun-if-changed={}",filename.as_path().display());
			let mut file=File::open(filename)?;
			let mut glyph=[0u8;16];
			file.read_exact(&mut glyph)?;
			ascii_bin.write_all(&glyph)?;
		}else{
			ascii_bin.write_all(&fallback_bin)?;
		}
	}

	Ok(())
}
