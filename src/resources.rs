use std::ffi;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    FailedToGetExePath,
    FileContainsNil,
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::IO(other)
    }
}

pub struct Resources {
    root_path: PathBuf,
}

impl Resources {
    pub fn from_relative_exe_path(rel_path: &str) -> Result<Resources, Error> {
        let exe_file_path = ::std::env::current_exe()
            .map_err(|_| Error::FailedToGetExePath)?;
        let exe_path = exe_file_path.parent()
            .ok_or(Error::FailedToGetExePath)?;

        Ok(Resources {
            root_path: exe_path.join(rel_path),
        })
    }

    pub fn load_cstring(&self, res_name: &str) -> Result<ffi::CString, Error> {
        let mut file = fs::File::open(self.res_name_to_path(res_name))?;

        // allocate buffer of the same size as file
        let mut buffer: Vec<u8> = Vec::with_capacity(
            file.metadata()?.len() as usize + 1
        );
        file.read_to_end(&mut buffer);

        // check for nul byte
        if buffer.iter().find(|i| **i == 0).is_some() {
            return Err(Error::FileContainsNil);
        }

        Ok(unsafe { ffi::CString::from_vec_unchecked(buffer) })
    }

    fn res_name_to_path(&self, location: &str) -> PathBuf {
        let mut path: PathBuf = self.root_path.into();

        for part in location.split("/") {
            path = path.join(part)
        }

        path
    }
}