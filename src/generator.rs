use std::{collections::HashMap, path::Path};

use crate::{config::Config, parser::parse};

pub struct Generator {
    type_map: HashMap<(), ()>,
    conversions: HashMap<(), ()>,
}

impl Generator {
    fn new(_config: Config) -> Self {
        Generator {
            type_map: HashMap::new(),
            conversions: HashMap::new(),
        }
    }
}

pub fn generate() {
    let functions = parse(Path::new("C:/Users/sven/Desktop/ffi_stub/src/test/test.h"));

    println!("{:#?}", functions);
}
