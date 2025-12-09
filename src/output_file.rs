use cap_std::fs_utf8::{Dir, camino::Utf8Path as Path};

use std::io::Result as IOResult;

macro_rules! static_file {
    ($file_name:expr) => {
        OutputFile {
            output_path: cap_std::fs_utf8::camino::Utf8Path::new($file_name),
            contents: include_str!(concat!("../static/", $file_name)),
        }
    };
}

macro_rules! static_byte_file {
    ($file_name:expr) => {
        OutputFile {
            output_path: cap_std::fs_utf8::camino::Utf8Path::new($file_name),
            contents: include_bytes!(concat!("../static/", $file_name)),
        }
    };
}

pub(crate) use static_byte_file;
pub(crate) use static_file;

#[derive(Debug)]
pub(crate) struct OutputFile<'data, Contents> {
    pub output_path: &'data Path,
    pub contents: Contents,
}

impl<'data, Contents> OutputFile<'data, Contents> {
    pub fn write(self, output_dir: &Dir) -> IOResult<()>
    where
        Contents: AsRef<[u8]>,
    {
        output_dir.write(self.output_path, self.contents)
    }
}
