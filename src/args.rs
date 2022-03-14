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
    #[usage = "countserver <args ...>"]
    pub struct Args {
        pub help: bool,
        pub end: Option<End>,
        pub par: Option<Par>,
        pub alt: bool,
        pub n: Option<usize>,
        pub m: Option<usize>,
    }
    /// Help with arguments.
    ["-h" | "--help"] => {
        println!("{}", Args::help());
        help = true;
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
    /// Alternate implementation.
    ["--alt"] => {
        alt = true;
    }
    /// Number of transactions.
    ["-n", transactions] => {
        n = Some(str::parse(&transactions)?);
    }
    /// Maximum concurrent transactions.
    ["-m", transactions] => {
        m = Some(str::parse(&transactions)?);
    }
}

pub fn fail<T: std::fmt::Display>(msg: T) -> ! {
    eprintln!("{}", msg);
    std::process::exit(1);
}

pub fn get_args() -> Args {
    // XXX Closure is not redundant, as typechecking won't
    // allow cleaning it up.
    #[allow(clippy::redundant_closure)]
    let args = Args::args().unwrap_or_else(|e| fail(e));
    if args.end.is_none() {
        fail("must specify client or server");
    }
    if args.par.is_none() {
        fail("must specify seq or thread par or async");
    }
    args
}
