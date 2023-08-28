# Run server inside docker container (multiple programming languages)

## Server

Build docker image:

```sh
$ ./scripts/dockerbuild.sh all
$ docker build . -t hcr-example-multi-server
```

Run server:

```sh
$ docker run --privileged --group-add keep-groups --rm -it --stop-signal SIGINT -p 8080:8080 hcr-example-multi-server
2023-08-28T10:05:49.857389Z  INFO hcr::server: listening on 0.0.0.0:8080
```

## Client

Query supported programming languages:

```sh
$ grpcurl --plaintext 127.0.0.1:8080 languages.Languages/Index
{
  "languages": [
    {
      "id": "c",
      "name": "C (GCC 10.2.1)"
    },
    {
      "id": "go",
      "name": "Golang (1.19)"
    },
    {
      "id": "python",
      "name": "Python (3.10.6)"
    }
  ]
}
```

Run C code:

```sh
$ grpcurl -d '{ "lang": "c", "files": [{ "name": "main.c", "content": "#include <stdio.h>\nint main() { printf(\"Hello, World!\"); return 0; }" }] }' --plaintext 127.0.0.1:8080 runs.Runs/Create
{
  "status": "OK",
  "stdout": "Hello, World!",
  "exitCode": 0,
  "startTime": "2023-08-28T10:06:20.497337948Z",
  "realTime": {
    "nanos": 729977
  },
  "systemTime": {
    "nanos": 697000
  },
  "userTime": {

  },
  "maxRss": "3416"
}
```

Run Go code:

```sh
$ grpcurl -d '{ "lang": "go", "files": [{ "name": "main.go", "content": "package main\nimport \"fmt\"\nfunc main() { fmt.Println(\"Hello, World!\") }" }] }' --plaintext 127.0.0.1:8080 runs.Runs/Create
{
  "status": "OK",
  "stdout": "Hello, World!\n",
  "exitCode": 0,
  "startTime": "2023-08-28T12:56:35.841592951Z",
  "realTime": {
    "nanos": 1886227
  },
  "systemTime": {

  },
  "userTime": {
    "nanos": 1784000
  },
  "maxRss": "3640"
}
```

Run Python code:

```sh
$ grpcurl -d '{ "lang": "python", "files": [{ "name": "main.py", "content": "print(\"Hello, World!\")" }] }' --plaintext 127.0.0.1:8080 runs.Runs/Create
{
  "status": "OK",
  "stdout": "Hello, World!\n",
  "exitCode": 0,
  "startTime": "2023-08-28T10:07:33.412847401Z",
  "realTime": {
    "nanos": 33416373
  },
  "systemTime": {
    "nanos": 2662000
  },
  "userTime": {
    "nanos": 7996000
  },
  "maxRss": "8320"
}
```
