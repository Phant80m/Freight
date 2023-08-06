use super::FreightCompress;
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use owo_colors::OwoColorize;
use std::thread;
use std::{fs::File, io, path::Path};
use tar::Archive;
use walkdir::WalkDir;
use zip::{read::ZipArchive, write::FileOptions};

impl FreightCompress {
    pub fn into_tar(input: Vec<String>, output: String) -> Result<(), io::Error> {
        let handle = thread::spawn(move || {
            let tar_gz = File::create(output)?;
            let enc = GzEncoder::new(tar_gz, Compression::best());
            let mut tar = tar::Builder::new(enc);

            for items in input {
                let path = Path::new(&items);
                let relative_path = path.strip_prefix(".").unwrap_or(path);

                if path.is_file() {
                    let mut file = File::open(&items)?;
                    tar.append_file(relative_path, &mut file)?;
                } else if path.is_dir() {
                    tar.append_dir_all(relative_path, &items)?;
                }
            }
            Ok(())
        });
        match handle.join() {
            Ok(result) => {
                println!("{}", "File(s) compressed into a tarball!".bold().green());
                result
            }
            Err(e) => {
                eprintln!("Error occurred in the thread: {:?}", e);
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Thread panicked or returned an error",
                ))
            }
        }
    }
    pub fn from_tar(input: String, output: String) -> Result<(), io::Error> {
        let handle = thread::spawn(move || {
            let tar_gz = File::open(input)?;
            let tar = GzDecoder::new(tar_gz);
            let mut archive = Archive::new(tar);
            archive.unpack(output)?;

            Ok(())
        });
        match handle.join() {
            Ok(result) => {
                println!("{}", "File(s) decompressed from a tarball!".bold().green());
                result
            }
            Err(e) => {
                eprintln!("Error occurred in the thread: {:?}", e);
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Thread panicked or returned an error",
                ))
            }
        }
    }
    pub fn into_zip(input: Vec<String>, output: String) -> Result<(), io::Error> {
        let handle = thread::spawn(move || {
            let zip_file = File::create(format!("{}", output))?;
            let options = FileOptions::default()
                .compression_method(zip::CompressionMethod::Stored)
                .unix_permissions(0o755); // Optional: Set the file permissions for the stored files

            let mut zip = zip::write::ZipWriter::new(zip_file);

            for items in &input {
                let path = Path::new(items);
                if path.is_file() {
                    zip_file_into_zip(&mut zip, path, options)?;
                } else if path.is_dir() {
                    for entry in WalkDir::new(items) {
                        let entry = entry?;
                        let path = entry.path();
                        zip_file_into_zip(&mut zip, path, options)?;
                    }
                }
            }
            Ok(())
        });
        match handle.join() {
            Ok(result) => {
                println!("{}", "File(s) compressed into a zip!".bold().green());
                result
            }
            Err(e) => {
                eprintln!("Error occurred in the thread: {:?}", e);
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Thread panicked or returned an error",
                ))
            }
        }
    }
    pub fn from_zip(input: String, output: String) -> Result<(), io::Error> {
        let handle = thread::spawn(move || {
            let zip_file = File::open(input)?;
            let mut archive = ZipArchive::new(zip_file)?;

            for i in 0..archive.len() {
                let mut file = archive.by_index(i)?;

                let outpath = match file.enclosed_name() {
                    Some(path) => Path::new(output.as_str()).join(path),
                    None => continue,
                };

                if (*file.name()).ends_with('/') {
                    // Create directory if it doesn't exist
                    std::fs::create_dir_all(&outpath)?;
                } else {
                    if let Some(parent) = outpath.parent() {
                        if !parent.exists() {
                            std::fs::create_dir_all(parent)?;
                        }
                    }
                    let mut outfile = File::create(&outpath)?;
                    io::copy(&mut file, &mut outfile)?;
                }
            }

            Ok(())
        });
        match handle.join() {
            Ok(result) => {
                println!("{}", "File(s) decompressed from a zip!".bold().green());
                result
            }
            Err(e) => {
                eprintln!("Error occurred in the thread: {:?}", e);
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Thread panicked or returned an error",
                ))
            }
        }
    }
}
fn zip_file_into_zip<W>(
    zip: &mut zip::write::ZipWriter<W>,
    file_path: &Path,
    options: FileOptions,
) -> Result<(), io::Error>
where
    W: io::Write + io::Seek,
{
    if file_path.is_file() {
        let relative_path = file_path.strip_prefix(".").unwrap_or(file_path);
        zip.start_file(relative_path.to_str().unwrap(), options)?;
        let mut file = File::open(file_path)?;
        io::copy(&mut file, zip)?;
    }
    Ok(())
}
