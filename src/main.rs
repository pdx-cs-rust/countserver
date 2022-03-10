#![feature(thread_is_running)]
#![feature(scoped_threads)]

mod args;
mod ccthread;
mod ccasync;
mod csthread;
mod csthreadscoped;
mod csasync;

fn main() {
    let args = args::get_args();
    let p = || usize::from(std::thread::available_parallelism().unwrap());
    let n = args.n.unwrap_or(100_000);
    match args.end.unwrap() {
        args::End::Client => {
            let m = args.m.unwrap_or_else(p);
            match args.par.unwrap() {
                args::Par::Seq => {
                    if args.alt {
                        args::fail("no alt seq client");
                    }
                    todo!()
                }
                args::Par::Thread => {
                    if args.alt {
                        args::fail("no alt thread client");
                    }
                    ccthread::send(n, m);
                }
                args::Par::Async => {
                    if args.alt {
                        args::fail("no alt async client");
                    }
                    ccasync::start(n, m);
                }
            }
        }
        args::End::Server => {
            if args.n.is_some() {
                args::fail("no -n for server");
            }
            match args.par.unwrap() {
                args::Par::Seq => {
                    if args.alt {
                        args::fail("no alt seq server");
                    }
                    todo!()
                }
                args::Par::Thread => {
                    let m = args.m.unwrap_or_else(p);
                    if args.alt {
                        csthreadscoped::send(m);
                    } else {
                        csthread::send(m);
                    }
                }
                args::Par::Async => {
                    if args.m.is_some() {
                        args::fail("no -m for async server");
                    }
                    if args.alt {
                        args::fail("no alt async server");
                    }
                    csasync::start();
                }
            }
        }
    }
}
