# JSON API Benchmark

## Environment setup

Verify that Docker, and Docker Compose are available by executing the following commands:
```
docker --version
docker-compose --version
```

The PostgreSQL database used to run the benchmarks runs inside of a Docker container. Starting the container will automatically load all of the test data as well.
```
docker-compose up -d
```

Once the container is running, please read the instructions for each implementation (e.g. `rust`) for further instructions on how to run the benchmarks.

## Benchmarks
After starting one of the implementations, running a benchmark is fairly straight forward. Personally, I use [wrk](https://github.com/wg/wrk) to run benchmarks but you're obviously free to use whatever suits you.
```
wrk -H 'Accept: application/json,text/html' -H 'Connection: keep-alive' --latency -d 15 -c 64 --timeout 8 -t 3 "http://localhost:8080/species"
```
Runs a benchmark for 15 seconds, using 3 threads, and keeping 64 HTTP connections open.
```
Running 30s test @ http://localhost:8080/species
  3 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     9.24ms    1.97ms  38.10ms   74.07%
    Req/Sec     3.59k   206.56     4.08k    79.67%
  321393 requests in 30.05s, 59.46MB read
Requests/sec:  10695.38
Transfer/sec:      1.98MB
```
