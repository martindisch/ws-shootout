# ws-shootout

Testing the limits of WebSockets.

## Hints

- The bencher may run into the limit of file descriptors. Use `ulimit -n 500000`
  to set a higher soft limit.
- Opening lots of connections it will also exhaust the default port range. When
  running locally adjust it with
  `sudo sysctl -w net.ipv4.ip_local_port_range="1024 60999"`.

## Running locally

Start either `with-tungstenite` or `with-ws` to get a server up and running,
then run `bencher`. You can adjust the number of connections opened in its
source.

## Running with Docker

In just one command you can scale the number of bencher instances:
`docker compose up --build --scale bencher=6`

## License

Licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT license](LICENSE-MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
