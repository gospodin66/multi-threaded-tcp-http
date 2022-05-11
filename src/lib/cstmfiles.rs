use std::fs;
use std::fs::{OpenOptions};
use std::io::{BufReader, Read, Write};
use std::os::unix::fs::OpenOptionsExt;
//use std::path::Path;
/**
 * Since many things can go wrong when doing file I/O,
 * all the File methods return the io::Result<T> type,
 * which is an alias for Result<T, io::Error>
 * 
 * '?' operator is shorthand for e.g: .expect("Unable to open file")
 * 'Result<()>' is shorthand for e.g: Result<T,io::Error>
 */
pub fn create(path: &String) -> std::io::Result<()>{
    if fs::metadata(path).is_ok() {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "File already exists"))
    }
    let f = OpenOptions::new()
            .create(true)
            .write(true)
            .mode(0o664)
            .open(&path)?;
    let perms = f.metadata()?.permissions();
    //perms.set_readonly(true);
    //f.set_permissions(perms)?;
    println!("File permissions: {:?}", perms);
    Ok(())
}
/**
 * .sync_all() - attempts to sync all OS-internal metadata to disk.
 * .flush()    - flush this output stream, ensuring that all intermediately
 *               buffered contents reach their destination
 */
pub fn write(path : &String, fcontents: String) -> std::io::Result<()> {
    let fc_with_nl : String = fcontents + "\r\n";
    let mut f = OpenOptions::new()
                .write(true)
                .append(true)
                .open(&path)?;
    //f.set_len(5)?;
    f.write_all(fc_with_nl.as_bytes())?;
    f.sync_all()?;
    f.flush()?;
    Ok(())
}
#[allow(dead_code)]
pub fn read(path : &String) -> std::io::Result<String> {
    let f = OpenOptions::new()
            .read(true)
            .open(&path)?;
    let mut buf_reader = BufReader::new(f);
    let mut fcontents : String = String::new();
    buf_reader.read_to_string(&mut fcontents)?;
    Ok(fcontents)
}
#[allow(dead_code)]
pub fn remove(path : &String) -> std::io::Result<()> {
    fs::remove_file(path)?;
    Ok(())
}
#[allow(dead_code)]
pub fn get_f_len(path : &String) -> std::io::Result<u64> {
    let f = OpenOptions::new()
            .read(true)
            .open(&path)?;
    let len = f.metadata().unwrap().len();
    Ok(len)
}
