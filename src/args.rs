extern crate argwerk;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum End {
    Server,
    Client,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Par {
    Seq,
    Thread,
    Async,
}

argwerk::define! {
    pub struct Args {
        pub end: Option<End>,
        pub par: Option<Par>,
        pub n: usize = 100_000,
        pub m: usize = 100,
    }
    /// Server mode.
    ["-s" | "--server"] => {
        end = Some(End::Server);
    }
    /// Client mode.
    ["-c" | "--client"] => {
        end = Some(End::Client);
    }
    /// No parallelism.
    ["--seq" | "--sequential"] => {
        par = Some(Par::Seq);
    }
    /// Thread parallelelism.
    ["-t" | "--thread"] => {
        par = Some(Par::Thread);
    }
    /// Async parallelism.
    ["-a" | "--async"] => {
        par = Some(Par::Async);
    }
    /// Number of transactions.
    ["-n", transactions] => {
        n = str::parse(&transactions)?;
    }
    /// Maximum concurrent transactions.
    ["-m", transactions] => {
        m = str::parse(&transactions)?;
    }
}

pub fn get_args() -> Args {
    let args = Args::args().unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });
    if args.end.is_none() {
        eprintln!("must specify client or server");
        std::process::exit(1);
    }
    if args.par.is_none() {
        eprintln!("must specify seq or thread par or async");
        std::process::exit(1);
    }
    args
}
