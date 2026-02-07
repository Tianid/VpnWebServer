use std::{fs, io, path::Path};

pub fn read_bytes_quietly<T>(path: T) -> Option<Vec<u8>>
where
    T: AsRef<Path>,
{
    fs::read(path).ok()
}

pub fn read_string_quietly<T>(path: T) -> Option<String>
where
    T: AsRef<Path>,
{
    fs::read_to_string(path).ok()
}

pub fn read_bytes<T>(path: T) -> io::Result<Vec<u8>>
where
    T: AsRef<Path>,
{
    fs::read(path)
}

pub fn read_string<T>(path: T) -> io::Result<String>
where
    T: AsRef<Path>,
{
    fs::read_to_string(path)
}
