## Simple load balancer in Rust

Part of [coding challenges by John Crickett](https://codingchallenges.fyi/challenges/challenge-load-balancer/)

1. Round-robin algorithm
2. api servers in node (10 of them)

### Docker
1. build the images on `./Dockerfile` and `./js-server/Dockerfile`
2. run `docker compose up`

### TODO
- [ ] use 'time elapsed' to check bottleneck
- [ ] third implementation: use [Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html) on reqwest http client in order to remove
the currently bottleneck (in theory).
- [X] setup everything in docker
- [ ] improve docker setup (one server by container)
- [X] push to github


