use std::env;
#[allow(unused)]
use std::path::PathBuf;

fn main() {
    #[cfg(any(feature = "rt"))]
    let crate_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());

    #[allow(unused)]
    let chip_name = match env::vars()
        .map(|(a, _)| a)
        .filter(|x| {
            x.starts_with("CARGO_FEATURE_MIMXRT")
                || x.starts_with("CARGO_FEATURE_MCX")
                || x.starts_with("CARGO_FEATURE_LPC55S69")
        })
        .get_one()
    {
        Ok(x) => x,
        Err(GetOneError::None) => panic!("No mimxrt/mcx/lpc Cargo feature enabled"),
        Err(GetOneError::Multiple) => panic!("Multiple mimxrt/mcx/lpc Cargo features enabled"),
    }
    .strip_prefix("CARGO_FEATURE_")
    .unwrap()
    .to_ascii_lowercase();
    // .replace('_', "-");

    #[cfg(any(feature = "rt"))]
    println!(
        "cargo:rustc-link-search={}/src/chips/{}",
        crate_dir.display(),
        chip_name,
    );

    println!("cargo:rerun-if-changed=build.rs");
}

#[allow(unused)]
enum GetOneError {
    None,
    Multiple,
}

#[allow(unused)]
trait IteratorExt: Iterator {
    fn get_one(self) -> Result<Self::Item, GetOneError>;
}

impl<T: Iterator> IteratorExt for T {
    fn get_one(mut self) -> Result<Self::Item, GetOneError> {
        match self.next() {
            None => Err(GetOneError::None),
            Some(res) => match self.next() {
                Some(_) => Err(GetOneError::Multiple),
                None => Ok(res),
            },
        }
    }
}
