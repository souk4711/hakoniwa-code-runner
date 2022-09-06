# Run server on host

## Server

```sh
$ hakoniwa-code-runner start -c app.toml
2022-09-05T06:51:32.207731Z  INFO hcr::server: listening on 127.0.0.1:8080
```

## Client

```sh
$ grpcurl --plaintext 127.0.0.1:8080 languages.Languages/Index
{
  "languages": [
    {
      "id": "c",
      "name": "C (GCC)"
    }
  ]
}

$ grpcurl -d '{ "lang": "c", "files": [{ "name": "main.c", "content": "#include <stdio.h>\nint main() { printf(\"Hello, World!\"); return 0; }" }] }' --plaintext 127.0.0.1:8080 runs.Runs/Create
{
  "status": "OK",
  "stdout": "Hello, World!",
  "exitCode": 0,
  "startTime": "2022-09-05T06:52:05.728329446Z",
  "realTime": {
    "nanos": 448983
  },
  "systemTime": {
    "nanos": 461000
  },
  "userTime": {

  },
  "maxRss": "3440"
}
```
