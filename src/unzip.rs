use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::copy;
use std::path::Path;

pub fn expand(input: &Path, output: &Path) -> Result<(), Box<dyn Error>> {
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
                    create_dir_all(&parent)?;
                }
            }

            let mut f = File::create(&file_path)?;
            copy(&mut file, &mut f)?;
        }
    }

    Ok(())
}
