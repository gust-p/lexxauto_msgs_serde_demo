# Zenoh Chatter Demo

This project demonstrates integration between ROS2, Zenoh, Rust, and Python for service communication using Zenoh as a middleware bridge. It includes:

- A Rust-based ROS2 service (`service/`)
- A Python client (`client/`) that calls the service via Zenoh
- Example configuration files for Zenoh router and bridge

## Project Structure

```
├── client/                # Python client for calling the service
│   └── main.py
├── service/               # Rust ROS2 service implementation
│   ├── src/
│   │   └── main.rs
│   ├── Cargo.toml
│   └── service.Dockerfile
├── src/                   # Rust library for Python bindings (PyO3)
│   └── lib.rs
├── bridge.json5           # Zenoh bridge configuration
├── router.json5           # Zenoh router configuration
├── docker-compose.yml     # Compose file to run router, bridge, and services
├── ros.Dockerfile         # Dockerfile for ROS2/Zenoh environment
├── Cargo.toml             # Rust workspace manifest
├── pyproject.toml         # Python project manifest
```

## Components

### 1. Rust ROS2 Service (`service/`)
- Implements a ROS2 service `/demo/set_bool` using the `r2r` crate.
- Handles boolean requests and responds with success and a message.
- Built and run via Docker or natively.

### 2. Python Client (`client/`)
- Uses the `zenoh` Python library and the Rust-based Python bindings with `lexxauto_msgs_rs` (`zenoh_chatter_demo`) to call the service.

### 3. Zenoh Router & Bridge
- `router.json5` and `bridge.json5` provide example configurations.
- The bridge connects ROS2 DDS and Zenoh networks.

## Usage

### Prerequisites
- Docker and Docker Compose
- (Optional) Native Rust and Python toolchains for development
### SSH Agent Setup (Required for Private Git Dependencies) - Python Client

If you are using private git dependencies (such as in `Cargo.toml`), you must ensure your SSH agent is running and your key is added before running Docker Compose:

```sh
eval "$(ssh-agent -s)"
ssh-add /path/to/your/private/key
```
Replace `/path/to/your/private/key` with the path to your SSH private key (e.g., `~/.ssh/id_rsa`).

This step is required so Docker can access private repositories during the build process.

### Build and Run with Docker Compose

```sh
docker compose up --build
```
This will start the Zenoh router, bridge, and (if enabled) the ROS2 service container.

### Running the Python Client

1. Install dependencies (Check `maturin` docs if you prefer to use `uv`):
   ```sh
   python -m venv ~/virtualenvs/chatter
   source ~/virtualenvs/chatter/bin/activate
   maturin develop
   ```
2. Run the client:
   ```sh
   python client/main.py
   ```

## Configuration
- Edit `router.json5` and `bridge.json5` to customize Zenoh and bridge behavior.
- The service and client communicate using the key `demo/set_bool`.

## References
- [Zenoh](https://zenoh.io/)
- [ROS2](https://docs.ros.org/en/humble/index.html)
- [r2r Rust ROS2 client](https://github.com/sequenceplanner/r2r)
- [PyO3](https://pyo3.rs/)

---

*This is a proof-of-concept demo for bridging ROS2 and Zenoh using Rust and Python.*
