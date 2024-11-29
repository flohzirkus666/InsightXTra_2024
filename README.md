# Insight X Tra tech demo

Insight X-Tra tech demo, presented at Insight X Tra in 2024.
LoD used in this presentation: [Lab on Demand](https://labondemand.netapp.com/node/860)

## Installation & usage 🪛

### Ansible

Please follow the installation guidline on the [Ansible Documentation](https://docs.ansible.com/ansible/latest/installation_guide/intro_installation.html)

### Python (REST-endpoint)

My code was written in Python 3.12.7. This codebase should work with Python >=3.10. If your system doesn't offer Python 3.10,
you can go with Pyenv and install a more recent version of the python interpreter.

#### Installing packages

Installing packages by using pip:

```bash
cd python-rest_endpoint
python3 -m pip install -r requirements.txt
```

Or by using uv:

```bash
cd python-rest_endpoint
uv pip sync requirements.txt
```

Utilizing a virtual environment is highly recommended.

### Rust 🦀 (REST-endpoint)

To be able to compile this Rust binary, you'll need a copy of rust.
You can install Rust, by using rustup: [Rustup.rs](https://rustup.rs/)

#### Compilation

```bash
cd rust-rest_endpoint
cargo build --target
```

In case you're encountering compilation problems, please add this line to your Cargo.toml under dependencies:

```toml
[dependencies]
# -- snip --
openssl = { version = "0.10", features = ["vendored"] }
```

This will force Rust to use their own OpenSSL library, instead using your local machine one's.

### Vue (Web frontend)

Just click and run. You can place the files inside your wwwroot of your webserver. There are no additional steps needed.

## Dockerizing content 🐳

In case you have the need to run this environment containerized. You should be already set.
