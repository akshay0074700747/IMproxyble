use std::{error,fs,io,path::Path};

pub fn read_file(file_path: &str) -> Result<String, Box<dyn error::Error>> {

    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}

pub async fn copy_config_file<P: AsRef<str>>(source_path: P, destination_path: P) -> io::Result<()> {

    let source = Path::new(source_path.as_ref());
    let file_name = source.file_name().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid Input"))?;
    let dest = Path::new(destination_path.as_ref()).join(file_name);
    fs::copy(&source,&dest)?;
    Ok(())
}