use derive_builder::*;
use std::path::PathBuf;

#[derive(Builder)]
pub struct Config {
    header_files: Vec<PathBuf>,
    converters: Vec<&'static fn(function: &mut syn::ItemFn)>,
    #[builder(
        default = "PathBuf::from(std::env::var(\"OUT_DIR\").unwrap())",
        setter(into)
    )]
    out_dir: PathBuf,
    #[builder(default = "\"std::os::raw::\".to_string()")]
    prefix: String,
}

impl Config {
    fn add_header<T: Into<PathBuf>>(&mut self, header: T) -> &mut Self {
        self.header_files.push(header.into());
        return self;
    }

    fn add_function_converter(
        &mut self,
        function_converter: &'static fn(function: &mut syn::ItemFn),
    ) -> &mut Self {
        return self;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_test() {
        // let builder = Builder::new().header_files(vec!["sdf"]);
        // println!("cargo:WARNING= out dir = {}", builder.out_dir());
    }
}
