## Installing Go
If Go not already installed on your system, follow the [installation instructions](https://go.dev/doc/install) to get the latest version.
## Running the benchmark
Compile an optimized version of the code and run the server
```
cd [PROJECT ROOT]/go
go run main
```

In a separate console window, run the benchmarking tool. For example
```
wrk -H 'Accept: application/json,text/html' -H 'Connection: keep-alive' --latency -d 15 -c 64 --timeout 8 -t 3 "http://localhost:8080/species"
```
