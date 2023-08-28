# Run server inside docker container (multiple programming languages)

## Server

Build docker image:

```sh
$ ./scripts/dockerbuild.sh all
$ docker build . -t hcr-example-multi
```

Run server:

```sh
$ docker run --privileged --group-add keep-groups --rm -it --stop-signal SIGINT -p 8080:8080 hcr-example-multi
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
$ grpcurl -d '{ "lang": "go", "files": [{ "name": "main.go", "content": "package main\nimport \"fmt\"\nfunc main() { fmt.Println(\"こんにちは世界。\") }" }] }' --plaintext 127.0.0.1:8080 runs.Runs/Create
{
  "status": "OK",
  "stdout": "こんにちは世界。\n",
  "exitCode": 0,
  "startTime": "2023-08-28T13:35:47.809065474Z",
  "realTime": {
    "nanos": 1748005
  },
  "systemTime": {

  },
  "userTime": {
    "nanos": 1576000
  },
  "maxRss": "3416"
}
```

Run Python code:

```sh
$ grpcurl -d '{ "lang": "python", "files": [{ "name": "main.py", "content": "print(\"你好，世界！\")" }] }' --plaintext 127.0.0.1:8080 runs.Runs/Create
{
  "status": "OK",
  "stdout": "你好，世界！\n",
  "exitCode": 0,
  "startTime": "2023-08-28T13:36:15.806599898Z",
  "realTime": {
    "nanos": 34565068
  },
  "systemTime": {

  },
  "userTime": {
    "nanos": 10640000
  },
  "maxRss": "8320"
}
```
