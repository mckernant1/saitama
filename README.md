# Saitama
Load testing tool written in rust for fun. Possibly extensible, but I have not made a crate yet.


# Install

---

## Homebrew

Install via brew
```bash
brew install mckernant1/tap/saitama
```

# Punch

---

## Example

Sample Command
```bash
saitama punch --url http://localhost:8000\
	--rps 20\
	--duration 20\
	--duration-unit second\
	--method get
```

## User Manual

```
saitama punch --help                                            
saitama-punch 0.0.1
Load test against an endpoint

USAGE:
    saitama punch [OPTIONS] --url <URL> --rps <RPS> --duration <DURATION>

OPTIONS:
    -b, --body <BODY>
            Body of the request [default: ]

    -d, --duration <DURATION>
            How long to run the test. Used with duration_unit

    -h, --help
            Print help information

    -H, --headers <HEADERS>
            Headers on the request

    -m, --method <METHOD>
            HTTP method [default: get] [possible values: options, get, post, put, delete, head,
            trace, connect, patch]

    -n, --duration-unit <DURATION_UNIT>
            What unit to run with the test [default: minute] [possible values: second, minute, hour]

    -q, --quiet
            Less output per occurrence

    -r, --rps <RPS>
            How many RPS to drive. This can max out on certain devices

    -t, --thread-count <THREAD_COUNT>
            How many worker threads to start [default: 10]

    -u, --url <URL>
            Target URL

    -v, --verbose
            More output per occurrence

    -V, --version
            Print version information
```
