## Build, Test, Run

### Cargo

```sh
$ cargo build
$ cargo test
$ [PORT=1234] cargo run
# Or all three :)
$ cargo watch -x check -x test -x run
```

### Docker

```sh
❯ docker build -t colormixerservice:latest .
...
Successfully tagged colormixerservice:latest
❯ docker run --publish 8080:8080 host colormixerservice
❯ curl http://localhost:8080/greet/you
Hello you!%
```

## Deploy

```sh
❯ gcloud builds submit --tag gcr.io/chroma-345921/colormixerservice --timeout 9000
❯ gcloud config set project chroma-345921
❯ gcloud run deploy color-mixer-service --image gcr.io/chroma-345921/colormixerservice
Deploying container to Cloud Run service [color-mixer-service] in project [chroma-345921] region [europe-west2]
...


```