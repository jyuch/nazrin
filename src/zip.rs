use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub fn compress(input: &Path, output: &Path) -> anyhow::Result<()> {
    let zip_file = File::create(&output)?;
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
                zip.write_all(&*buf)?;
                buf.clear();
            } else {
                zip.add_directory(name, Default::default())?;
            }
        }
    }

    zip.finish()?;
    Ok(())
}
