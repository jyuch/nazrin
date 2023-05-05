use std::fs::{create_dir_all, File};
use std::io::{copy, Read, Write};
use std::path::Path;

pub fn compress(input: &Path, output: &Path) -> anyhow::Result<()> {
    let zip_file = File::create(output)?;
    let mut zip = zip::ZipWriter::new(zip_file);
    let mut buf = Vec::new();
    let walk_dir = walkdir::WalkDir::new(input);

    for it in walk_dir.into_iter() {
        let it = it?;
        let inner_path = it.path().strip_prefix(input)?;

        if let Some(name) = inner_path.as_os_str().to_str() {
            if it.path().is_file() {
                zip.start_file(name, Default::default())?;
                let mut f = File::open(it.path())?;
                f.read_to_end(&mut buf)?;
                zip.write_all(&buf)?;
                buf.clear();
            } else {
                zip.add_directory(name, Default::default())?;
            }
        }
    }

    zip.finish()?;
    Ok(())
}

pub fn expand(input: &Path, output: &Path) -> anyhow::Result<()> {
    let file = File::open(input)?;
    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let local_path = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        if (*file.name()).ends_with('/') {
            let directory_path = output.join(local_path);
            create_dir_all(&directory_path)?;
        } else {
            let file_path = output.join(local_path);
            if let Some(parent) = file_path.parent() {
                if !parent.exists() {
                    create_dir_all(parent)?;
                }
            }

            let mut f = File::create(&file_path)?;
            copy(&mut file, &mut f)?;
        }
    }

    Ok(())
}
