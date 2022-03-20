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
wrk -t3 -c100 -d30s http://localhost:8080/species
```
