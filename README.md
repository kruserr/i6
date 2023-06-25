<p align="center">
  <a href="https://github.com/kruserr/i6" target="_blank">
    <img width="300" src="https://raw.githubusercontent.com/kruserr/i6/main/assets/logo/logo.svg">
  </a>
  <br/>
  <br/>
  <a href="https://github.com/kruserr/i6/releases" target="_blank">
    <img src="https://img.shields.io/github/v/release/kruserr/i6?sort=semver&logo=GitHub&logoColor=white">
  </a>
  <a href="https://crates.io/crates/i6" target="_blank">
    <img src="https://img.shields.io/crates/v/i6?logo=Rust&logoColor=white"/> 
  </a>
  <br/>
  <a href="https://hub.docker.com/r/kruserr/i6" target="_blank">
    <img src="https://img.shields.io/docker/v/kruserr/i6?sort=semver&logo=docker&logoColor=white"/> 
  </a>
  <a href="https://codecov.io/gh/kruserr/i6" target="_blank"> 
    <img src="https://img.shields.io/codecov/c/gh/kruserr/i6?logo=Codecov&logoColor=white"/> 
  </a>
</p>

# i6
A collection of tools

## Documentation
Visit the [Documentation](https://docs.rs/i6).

## Getting Started
### Docker
Run database with docker
```bash
docker run -dit --rm --name i6 kruserr/i6:0.1
```

### Git and cargo
Clone the repo and build the database from source
```bash
git clone https://github.com/kruserr/i6.git
cd i6
cargo run --release
```
