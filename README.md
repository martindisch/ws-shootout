# ws-shootout

Testing the limits of WebSockets.

## Hints

- The bencher may run into the limit of file descriptors. Use `ulimit -n 80000`
  to set a higher soft limit.
- Opening lots of connections it will also exhaust the default port range.
  Adjust it with `sudo sysctl -w net.ipv4.ip_local_port_range="1024 60999"`.
