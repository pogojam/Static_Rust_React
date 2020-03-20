
use rocket::response::NamedFile;
use std::{io,path};
use path::{Path,PathBuf};

#[get("/")]
fn index()  -> Result<NamedFile,io::Error> {
 NamedFile::open("./client/build/index.html")
}

#[get("/static/<file..>")]
pub fn static_files(file: PathBuf) -> Option<NamedFile> {
    let file_path = Path::new("./client/build/static/").join(file);
    println!("{:?}",file_path);
    NamedFile::open(file_path).ok()
}

pub fn init() {
    rocket::ignite().mount("/", routes![index,static_files]).launch();
}