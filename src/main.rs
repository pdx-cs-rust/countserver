mod args;
mod counter;

mod ccasync;
mod ccseq;
mod ccthread;

mod csasync;
mod csseq;
mod csthread;

fn main() {
    let args = args::get_args();
    let p = || usize::from(std::thread::available_parallelism().unwrap()) / 2 + 1;
    let n = args.n.unwrap_or(100_000);
    match args.end.unwrap() {
        args::End::Client => match args.par.unwrap() {
            args::Par::Seq => {
                if args.m.is_some() {
                    args::fail("no -m for seq client");
                }
                if args.alt {
                    args::fail("no alt seq client");
                }
                ccseq::send(n);
            }
            args::Par::Thread => {
                if args.alt {
                    args::fail("no alt thread client");
                }
                let m = args.m.unwrap_or_else(p);
                ccthread::send(n, m);
            }
            args::Par::Async => {
                let m = args.m.unwrap_or_else(p);
                if args.alt {
                    ccasync::rt_async_std::start(n, m);
                } else {
                    ccasync::rt_tokio::start(n, m);
                }
            }
        },
        args::End::Server => {
            if args.n.is_some() {
                args::fail("no -n for server");
            }
            match args.par.unwrap() {
                args::Par::Seq => {
                    if args.m.is_some() {
                        args::fail("no -m for seq server");
                    }
                    if args.alt {
                        csseq::simple::start();
                    } else {
                        csseq::fast::start();
                    }
                }
                args::Par::Thread => {
                    let m = args.m.unwrap_or_else(p);
                    if args.alt {
                        csthread::simple::start(m);
                    } else {
                        csthread::fast::start(m);
                    }
                }
                args::Par::Async => {
                    if args.m.is_some() {
                        args::fail("no -m for async server");
                    }
                    if args.alt {
                        csasync::rt_async_std::start();
                    } else {
                        csasync::rt_tokio::start();
                    }
                }
            }
        }
    }
}
