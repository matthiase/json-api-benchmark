## Installing Node
If Node not already installed on your system, follow the [installation instructions](https://nodejs.org/en/download/) to get the latest version.
## Running the benchmark
Install the dependencies
```
yarn install
```

Start the server in production mode
```
yarn start
```

In a separate console window, run the benchmarking tool. For example
```
wrk -H 'Accept: application/json,text/html' -H 'Connection: keep-alive' --latency -d 15 -c 64 --timeout 8 -t 3 "http://localhost:8080/species"
```
