use crate::{error, ok, warn};
use async_zip::tokio::write::ZipFileWriter;
use async_zip::{Compression, ZipEntryBuilder};
use std::path::{Path, PathBuf};
use tokio::fs::read_dir;
use tokio::io::AsyncReadExt;
use tokio::{fs::File, io::AsyncWriteExt};
use tokio_util::compat::TokioAsyncReadCompatExt;

use super::{
    cliinput::{Command, CompressType},
    types::{PCHStdError, CONFIG},
};

pub async fn save_file<T: AsRef<Path>, U: AsRef<[u8]>>(
    filename: T,
    data: &U,
) -> Result<(), PCHStdError> {
    let mut file = File::create(filename).await?;
    file.write_all(data.as_ref()).await?;
    Ok(())
}

pub async fn rename(
    _opt: &Command,
    input: &PathBuf,
    output: &PathBuf,
    compress_type: &Option<CompressType>,
) -> Result<String, PCHStdError> {
    let mut input_files_reader = tokio::fs::read_dir(input).await?;
    // check output directory is or not existing
    if !output.exists() {
        tokio::fs::create_dir_all(output).await?;
    }
    let mut output_files_reader = tokio::fs::read_dir(output).await?;
    let mut input_files = Vec::new();
    loop {
        match input_files_reader.next_entry().await {
            Ok(entry) => match entry {
                Some(entry) => input_files.push(entry.path()),
                None => break,
            },
            Err(_) => break,
        }
    }
    let mut output_files = Vec::new();
    loop {
        match output_files_reader.next_entry().await {
            Ok(entry) => match entry {
                Some(entry) => output_files.push(entry.path()),
                None => break,
            },
            Err(_) => break,
        }
    }
    input_files.sort_by(|a, b| a.cmp(&b));
    output_files.sort_by(|a, b| a.cmp(&b));

    for output_file in &output_files {
        // delete output files
        tokio::fs::remove_file(output_file).await?;
    }
    if input_files.len() != CONFIG.lock().await.file_names.len() {
        let input_files_len = input_files.len();
        let file_names_len = CONFIG.lock().await.file_names.len();
        if input_files_len > file_names_len {
            warn!(
                "Rename warn:",
                input_files_len, "input_files", file_names_len, "file_names"
            );

            warn!(
                "Rename warn:",
                input_files_len - file_names_len,
                "input files will be ignored"
            );

            // random remove input files from input_files
            for _ in 0..(input_files_len - file_names_len) {
                let index = rand::random::<usize>() % input_files.len();
                input_files.remove(index);
            }
        } else {
            warn!(
                "Rename warn:",
                input_files_len, "input_files", file_names_len, "file_names"
            );

            warn!(
                "Rename warn:",
                input_files_len - file_names_len,
                "input files will be duplicated"
            );

            // random add input files to input_files
            for _ in 0..(file_names_len - input_files_len) {
                let index = rand::random::<usize>() % input_files.len();
                input_files.push(input_files[index].clone());
            }
        }
    }
    for inputs2outputs in input_files
        .iter()
        .zip(CONFIG.lock().await.file_names.iter())
    {
        // get extension name
        let extension_name = inputs2outputs
            .0
            .extension()
            .ok_or(PCHStdError::NoExtension)?;

        // create output files
        let output_file = output.join(inputs2outputs.1).with_extension(extension_name);
        tokio::fs::copy(inputs2outputs.0, &output_file).await?;
        ok!(
            "Renamed:",
            &inputs2outputs
                .0
                .file_name()
                .ok_or(PCHStdError::FileNameExtractFailed)?
                .to_str()
                .ok_or(PCHStdError::FileNameExtractFailed)?,
            "->",
            &output_file
                .file_name()
                .ok_or(PCHStdError::FileNameExtractFailed)?
                .to_str()
                .ok_or(PCHStdError::FileNameExtractFailed)?
        );
    }

    // Compress output files to zip file
    {
        use std::env;
        match compress_type {
            Some(compress_type) => match compress_type {
                CompressType::Zip => {
                    error!("Rename error:", "Zip compress method not available");
                }
                CompressType::_7Zip => {
                    let temp_dir = env::temp_dir();
                    sevenz_rust::compress_to_path(
                        output,
                        temp_dir.join("auto_rename_screenshots").join("output.7z"),
                    )
                    .map_err(|_err| PCHStdError::CompressionFailed)?;
                    // move temp file to output directory
                    tokio::fs::copy(
                        temp_dir.join("auto_rename_screenshots").join("output.7z"),
                        output.join("output.7z"),
                    )
                    .await?;
                    // delete temp file
                    tokio::fs::remove_file(
                        temp_dir.join("auto_rename_screenshots").join("output.7z"),
                    )
                    .await?;
                    ok!("Compressed:", "output.7z");
                }
                CompressType::NoCompress => {
                    warn!("Rename warn:", "No compress method available");
                }
            },
            None => {}
        }
    }

    Ok("Finished".to_string())
}

async fn dirs(dir: PathBuf) -> Result<Vec<PathBuf>, PCHStdError> {
    let mut dirs = vec![dir];
    let mut files = vec![];
    while !dirs.is_empty() {
        let mut dir_iter = read_dir(dirs.remove(0)).await?;
        while let Some(entry) = dir_iter.next_entry().await? {
            let entry_path_buf = entry.path();
            if entry_path_buf.is_dir() {
                dirs.push(entry_path_buf);
            } else {
                files.push(entry_path_buf);
            }
        }
    }
    Ok(files)
}

// //压缩单个文件
// async fn zip_entry(
//     input_path: &Path,
//     file_name: &str,
//     zip_writer: &mut ZipFileWriter<File>,
// ) -> Result<(), PCHStdError> {
//     let mut input_file = File::open(input_path).await?;
//     let builder = ZipEntryBuilder::new(file_name.into(), Compression::Stored);
//     let mut zip_entry = zip_writer.start_entry(builder).await?;
//     let mut buffer = Vec::new();
//     input_file.read_to_end(&mut buffer).await?;
//     zip_entry.write_all(&buffer).await?;
//     return Ok(());
// }

// 压缩单个文件
async fn zip_entry(
    input_path: &Path,
    file_name: &str,
    zip_writer: &mut ZipFileWriter<File>,
) -> Result<(), PCHStdError> {
    let input_file = File::open(input_path).await?;
    let builder = ZipEntryBuilder::new(file_name.into(), Compression::Stored);
    let mut entry_writer = zip_writer
        .write_entry_stream(builder)
        .await
        .map_err(|_err| PCHStdError::CompressionFailed)?;
    // let mut buffer = Vec::new();
    // input_file.read_to_end(&mut buffer).await?;
    futures_lite::io::copy(&mut input_file.compat(), &mut entry_writer).await?;

    entry_writer
        .close()
        .await
        .map_err(|_err| PCHStdError::CompressionFailed)?;
    Ok(())
}

//压缩
async fn zip(input_path: &Path, out_path: &Path) -> Result<(), PCHStdError> {
    let file = File::create(out_path).await?;
    let mut writer = ZipFileWriter::with_tokio(file);
    let input_dir_str = input_path
        .as_os_str()
        .to_str()
        .ok_or(PCHStdError::CompressionFailed)?;
    if input_path.is_file() {
        let file_name = input_path
            .file_name()
            .ok_or(PCHStdError::CompressionFailed)?
            .to_string_lossy();
        zip_entry(input_path, &file_name, &mut writer).await?;
    } else {
        let entries = dirs(input_path.into()).await?;
        for entry_path_buf in entries {
            let entry_path = entry_path_buf.as_path();
            let entry_str = entry_path
                .as_os_str()
                .to_str()
                .ok_or(PCHStdError::CompressionFailed)?;
            let file_name = &entry_str[(input_dir_str.len() + 1)..];
            zip_entry(entry_path, file_name, &mut writer).await?;
        }
    }
    writer
        .close()
        .await
        .map_err(|_err| PCHStdError::CompressionFailed)?;
    Ok(())
}
