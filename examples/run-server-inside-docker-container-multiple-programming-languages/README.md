# Run server inside docker container (multiple programming languages)


## Server

Build docker image:

```sh
$ ./scripts/dockerbuild.sh all
$ docker build . -t hcr-example-multi-all
```

Run server:

```sh
$ docker run --privileged --group-add keep-groups --rm -it --stop-signal SIGINT -p 8080:8080 hcr-example-run-server-inside-docker-container
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
