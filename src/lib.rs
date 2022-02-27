extern crate argwerk;

argwerk::define! {
    /// Countserver binary.
    pub struct Args {
        pub n: usize = 100_000,
        pub m: usize = 100,
    }
    /// Number of transactions.
    ["-n"] => {
    }
    /// Maximum concurrent transactions.
    ["-m"] => {
    }
}

pub fn get_args() -> Args {
    Args::args().unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    })
}
