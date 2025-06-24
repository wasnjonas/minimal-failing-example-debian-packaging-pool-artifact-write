use std::{collections::HashMap, fs, pin::Pin};

use async_trait::async_trait;
use debian_packaging::{
    io::DataResolver,
    repository::{
        builder::{InMemoryDebFile, NO_SIGNING_KEY, RepositoryBuilder},
        filesystem::FilesystemRepositoryWriter,
    },
};
use futures::AsyncRead;

struct Resolver {
    pub packages: HashMap<String, Vec<u8>>,
}

impl Resolver {
    fn new() -> Resolver {
        Resolver {
            packages: HashMap::new(),
        }
    }

    fn add_package(&mut self, path: String, package: Vec<u8>) {
        // HACK: This should be handled gracefully
        let _ = self.packages.insert(path, package);
    }
}

#[async_trait]
impl DataResolver for Resolver {
    async fn get_path(
        &self,
        path: &str,
    ) -> debian_packaging::error::Result<Pin<Box<dyn AsyncRead + Send>>> {
        Ok(Box::pin(futures::io::Cursor::new(
            self.packages
                .get(path)
                .expect("There should be the file")
                .clone(),
        )))
    }
}

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

    let mut resolver = Resolver::new();
    resolver.add_package(path, data);

    let writer = FilesystemRepositoryWriter::new("./apt_repo");

    match builder
        .publish(
            &writer,
            &resolver,
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
