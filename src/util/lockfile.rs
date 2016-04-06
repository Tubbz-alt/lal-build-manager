use rustc_serialize::json;
use std::path::Path;
use std::fs::File;
use std::io::prelude::Write;

use std::collections::HashMap;

use errors::LalResult;
use util::input;

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct Dependency {
    pub name: String,
    pub version: String,
    // TODO: other stash data if using a stashed build
    pub dependencies: HashMap<String, Dependency>,
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct Lock {
    pub name: String,
    pub config: String,
    // pub date: String,
    pub version: String,
    pub dependencies: HashMap<String, Dependency>,
}

impl Lock {
    pub fn new(n: &str, v: Option<&str>, cfg: &str) -> Lock {
        Lock {
            name: n.to_string(),
            version: v.unwrap_or("experimental").to_string(),
            config: cfg.to_string(),
            dependencies: HashMap::new(),
        }
    }
    pub fn populate_from_input(mut self) -> LalResult<Self> {
        let deps = input::analyze();
        Ok(self)
    }
    pub fn write(&self, pth: &Path) -> LalResult<()> {
        let encoded = json::as_pretty_json(self);
        let mut f = try!(File::create(pth));
        try!(write!(f, "{}\n", encoded));
        info!("Wrote lockfile {}: \n{}", pth.display(), encoded);
        Ok(())
    }
}
