

# Quickstart (wsl2 Ubuntu)

```bash
# first installation
sudo apt-get update
sudo apt-get install libssl-dev pkg-config

cargo install trunk wasm-bindgen-cli
rustup target add wasm32-unknown-unknown

# run
trunk serve
```


# Quickstart (docker)

[Docker](https://www.docker.com/get-started) required.

```bash
./docker/setup.sh # first installation

./docker/run-exposing-port.sh trunk serve
```

# Release (wsl2)

```bash
trunk build --release
```

# Release (docker)

```bash
./run.sh trunk build --release
```

or

```bash
./run.sh
trunk build --release
```
