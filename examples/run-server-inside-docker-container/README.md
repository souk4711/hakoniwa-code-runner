# Run server inside docker container

## Server

```sh
$ docker build . -t hcr-example-run-server-inside-docker-container
$ docker run --privileged --rm -it --stop-signal SIGINT -p 8080:8080 hcr-example-run-server-inside-docker-container
2022-09-05T10:27:48.867769Z  INFO hcr::server: listening on 0.0.0.0:8080
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
  "startTime": "2022-09-05T10:28:14.595426234Z",
  "realTime": {
    "nanos": 5233800
  },
  "systemTime": {

  },
  "userTime": {
    "nanos": 1197000
  },
  "maxRss": "3852"
}
```
