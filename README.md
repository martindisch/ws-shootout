# ws-shootout

Testing the boundaries of WebSockets and server-sent events.

## Hints

- The benchers may run into the limit of file descriptors. Use
  `ulimit -n 500000` to set a higher soft limit.
- Opening lots of connections will also exhaust the default port range. When
  running locally adjust it with
  `sudo sysctl -w net.ipv4.ip_local_port_range="1024 60999"`.

## Running locally

Start any of the servers, such as `with-tungstenite` or `with-warp`, then run
the matching bencher (`ws-bencher` or `sse-bencher`). You can adjust the number
of connections opened in the source.

## Running with Docker

Change into either the `ws` or `sse` directory, then you can scale the number
of bencher instances with a single command such as:
`docker compose up --build --scale sse-bencher=16`

## Unscientific results

Running one server and several clients (10'000 connections each) tied together
with docker compose works quite well. The main problem seems to be that
Docker's internal name resolution starts to buckle under the load when too many
connections are opened at a time, so they have to be spaced out over some
period. After that we're hitting a limit around the magic number 2^18 (262'144
connections), regardless of the implementation. They do differ in the amount of
resources consumed, but not enough to definitively exclude any one as
inadequate.

## License

Licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT license](LICENSE-MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
