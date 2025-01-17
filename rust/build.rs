// Copyright 2022 CeresDB Project Authors. Licensed under Apache-2.0.

use std::{io::Write, path::Path};
use walkdir::WalkDir;

struct Builder {
    out_dir: String,
    proto_dir: String,
    proto_paths: Vec<String>,
    module_names: Vec<String>,
}

impl Builder {
    fn new(out_dir: String, proto_dir: String) -> Result<Self, Box<dyn std::error::Error>> {
        // Get each proto's path and related module name.
        let proto_dir = Path::new(proto_dir.as_str()).to_str().unwrap().to_owned();

        let mut proto_paths = Vec::new();
        let mut module_names = Vec::new();
        for entry in WalkDir::new(proto_dir.clone())
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir() && !e.file_type().is_symlink())
        {
            proto_paths.push(entry.path().display().to_string());

            let module_name = entry
                .file_name()
                .to_str()
                .unwrap()
                .split(".")
                .into_iter()
                .next()
                .unwrap()
                .to_owned();
            module_names.push(module_name);
        }

        Ok(Self {
            out_dir,
            proto_dir,
            proto_paths,
            module_names,
        })
    }

    fn generate(self) -> Result<(), Box<dyn std::error::Error>> {
        self.generate_files()?.generate_mod_file()?;

        Ok(())
    }

    fn generate_files(self) -> Result<Self, Box<dyn std::error::Error>> {
        // Compile proto files.
        tonic_build::configure()
            .compile(&self.proto_paths, &[self.proto_dir.clone()])
            .map_err(|e| Box::new(e))?;

        Ok(self)
    }

    fn generate_mod_file(self) -> Result<Self, Box<dyn std::error::Error>> {
        // Generate mod.rs file.
        let mut mod_file = std::fs::File::create(format!("{}/mod.rs", self.out_dir))?;
        for module_name in self.module_names.iter() {
            write!(mod_file, "pub mod {};\n", module_name)?;
        }

        Ok(self)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::env::var("OUT_DIR").map_err(|e| Box::new(e))?;

    let protos_dir = "protos".to_string();
    let builder = Builder::new(out_dir, protos_dir)?;
    builder.generate()
}
