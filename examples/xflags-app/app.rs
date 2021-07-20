mod flags {
    // xflags! doesn't support `:` in types
    use std::path::PathBuf;

    xflags::xflags! {
        src "./app.rs"

        cmd app
            required input: PathBuf
        {
            optional --help
            /// Sets a number
            required --number number: u32
            /// Sets an optional number
            optional --opt-number opt_number: u32
            /// Sets width
            optional --width width: u32
        }
    }

    // generated start
    // The following code is generated by `xflags` macro.
    // Run `env UPDATE_XFLAGS=1 cargo build` to regenerate.
    #[derive(Debug)]
    pub struct App {
        pub input: PathBuf,

        pub help: bool,
        pub number: u32,
        pub opt_number: Option<u32>,
        pub width: Option<u32>,
    }

    impl App {
        pub const HELP: &'static str = Self::HELP_;

        #[allow(dead_code)]
        pub fn from_env() -> xflags::Result<Self> {
            Self::from_env_()
        }

        #[allow(dead_code)]
        pub fn from_vec(args: Vec<std::ffi::OsString>) -> xflags::Result<Self> {
            Self::from_vec_(args)
        }
    }
    // generated end

    impl App {
        pub fn validate(&self) -> xflags::Result<()> {
            if let Some(width) = self.width {
                if width != 0 {
                    return Err(xflags::Error::new("width must be positive"));
                }
            }
            Ok(())
        }
    }
}

fn main() {
    let args = match flags::App::from_env() {
        Ok(args) => {
            if args.help {
                println!("{}", flags::App::HELP);
                std::process::exit(0);
            }
            args
        }
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(2);
        }
    };
    match args.validate() {
        Ok(()) => {}
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(2);
        }
    }
    println!("{:#?}", args);
}