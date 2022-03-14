# countserver: demo of Rust parallel/async
Bart Massey 2020

This is a little demo that provides client and server
implementations of a counter: concurrent clients grab unique
counter values from the server.

You will need a recent nightly Rust toolchain to build and
run this. It is known to work on `x86_64` with the channel
in the `rust-toolchain.toml` file. Just `cargo build
--release`.  You will probably want to run the resulting
binary as `target/release/countserver` for ease of arguments
and for timing.

Supplied modes:

* Count Servers (`--server`)

    * `--seq`: Sequential count server.

    * `--thread`: Concurrent count server using many threads that
      access an atomic counter.

    * `--async`: Concurrent count server using async accesses to
      an atomic counter.

* Count Clients (`--client`)

    * `--seq`: Sequential count client.

    * `--thread`: Concurrent count client using many threads that
      access an atomic counter.

    * `--async`: Concurrent count client using async accesses to
      collect counts.

    The `-n` flag specifies the total number of transactions
    to complete. The default is 100K.

The `-m` thread specifies a target level of parallelism for
some clients and servers. The default is the number of
parallel threads Rust believes is available: typically the
number of threads on the host machine.

Async client and server will use Tokio by default. To use
`async-std`, use the `--alt` argument.

Threaded server will use standard threading by default. To
use a scoped server, use the `--alt` argument.

On my modern Linux box I need to run these things as root to
get decent performance. I also occasionally need to

    sysctl net.ipv4.tcp_syncookies=1

to get SYN cookies turned off on localhost. (Sigh. Working
on reporting this.) Don't forget to undo this when you're done!

## Acknowledgements

Thanks to Josh Triplett for the scoped-threads server and
for other help with this.

## License

This work is made available under the "Copyleft Next v0.3.1
license." Please see the file `LICENSE.txt` in this
distribution for license terms.
