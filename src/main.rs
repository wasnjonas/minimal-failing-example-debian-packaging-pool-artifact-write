use std::fs;

use debian_packaging::repository::{
    builder::{InMemoryDebFile, NO_SIGNING_KEY, RepositoryBuilder},
    filesystem::{FilesystemRepositoryReader, FilesystemRepositoryWriter},
};

#[tokio::main]
async fn main() {
    let filename = "data/vim_2%3a9.0.1378-2+deb12u2_arm64.deb";
    let data = fs::read(filename).expect("There should be the file");

    let mut builder = RepositoryBuilder::new_recommended(
        ["arm64".to_string()].iter(),
        ["vim".to_string()].iter(),
        "suite",
        "codename",
    );

    let path = builder
        .add_binary_deb(
            "vim",
            &InMemoryDebFile::new(filename.to_string(), data.clone()),
        )
        .expect("The file {filename} should be a valid a valid DebFile");

    println!("Will be written to {path}");

    let reader = FilesystemRepositoryReader::new("./apt_repo");
    let writer = FilesystemRepositoryWriter::new("./apt_repo");

    match builder
        .publish(
            &writer,
            &reader,
            "bookworm",
            8,
            &Some(|event| eprintln!("{event}")),
            NO_SIGNING_KEY,
        )
        .await
    {
        Ok(()) => println!("The dummy repo was written!"),
        Err(e) => println!("Looks like an error to me: {e}"),
    };
}
