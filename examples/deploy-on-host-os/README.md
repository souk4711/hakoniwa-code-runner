# Deploy on host os

## Server

```sh
$ hakoniwa-code-runner start -c app.toml
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
  "startTime": "2022-09-05T03:35:03.703822352Z",
  "realTime": {
    "nanos": 586339
  },
  "systemTime": {

  },
  "userTime": {
    "nanos": 558000
  },
  "maxRss": "2716"
}
```
