# countserver: demo of Rust parallel/async
Bart Massey 2020

This is a little demo that provides client and server
implementations of a counter: concurrent clients grab unique
counter values from the server.

To build, specify either `--features=tokio-rt` or
`--features=async-std-rt` to get an async client and server
built with the specified async runtime.

Supplied programs:

* `csthread`: Concurrent count server using threads that
  access an `Arc`-protected atomic counter.

* `csasync`: Concurrent count server using async accesses to
  an `Arc`-protected atomic counter.

* `ccasync`: Concurrent count client that spawns many connections
  before collecting all counts.
