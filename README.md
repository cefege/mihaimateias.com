# mihaimateias.com

Personal site of Mihai Mateias. A Dioxus 0.7 fullstack app, server-rendered and served as a container.

The Harry Potter 2 browser game is **not** built here. It is a separate project that publishes the image `ghcr.io/cefege/hp2-web:<tag>`. This site only links to it at `https://hp2.mihaimateias.com/`.

## Local development

Requires Rust, the wasm target and dioxus-cli 0.7.10:

```sh
rustup target add wasm32-unknown-unknown
cargo binstall dioxus-cli@0.7.10 -y
dx serve
```

Check the server build alone:

```sh
cargo check --features server
```

## Container

```sh
docker build -t site .
docker run --rm -p 8080:8080 site
```

The image runs `/usr/local/app/server` as UID 65532 on port 8080.

## Deployment

Pushes to `main` (outside `deploy/`) run `.github/workflows/image.yml`. It builds and pushes `ghcr.io/cefege/mihaimateias.com:sha-<12>` and commits that tag into `deploy/production/kustomization.yaml`. Flux (on the OVH k3s cluster) syncs `deploy/production` from `main`.

`deploy/production/kustomization.yaml` pins two images:

- `ghcr.io/cefege/mihaimateias.com`: set automatically by CI.
- `ghcr.io/cefege/hp2-web`: the game version. To upgrade the game, change its `newTag` to a new `hp2-web` tag and push.

`deploy/flux/bootstrap.yaml` is applied once by hand; Flux does not sync it.
