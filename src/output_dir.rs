use cap_std::{
    AmbientAuthority,
    fs_utf8::{Dir, camino::Utf8Path as Path},
};

use std::{fs::remove_dir_all, io::Result};

pub(crate) fn create_output_dir(
    output_dir_path: &Path,
    ambient_authority: AmbientAuthority,
) -> Result<Dir> {
    remove_dir_all(output_dir_path)?;
    Dir::create_ambient_dir_all(output_dir_path, ambient_authority)?;
    let output_dir = Dir::open_ambient_dir(output_dir_path, ambient_authority)?;
    Ok(output_dir)
}
