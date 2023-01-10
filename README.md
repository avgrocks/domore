# domore

domore is used to do more, to parallelize something. Execute any command in parallel.

## Installation

## Examples

Execute `curl` 40 times in parallel and repeat that `3` times.

```bash
domore -t 40 -r 3 "curl 127.0.0.1:8899"
```

To list all options hit `domore --help`.
