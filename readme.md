# rust_web_fullstack

Demo project for [this bilibili video(BV1RP4y1G7KF)](https://www.bilibili.com/video/BV1RP4y1G7KF)

**FOR STUDY ONLY**

Database: PostgreSQL

## Demo

Download from [release](https://github.com/rexcape/rust_web_fullstack/releases)

### Prepare

- install PostgreSQL
- create database tutorial
- create tables using tables.sql
- install http-server with `npm install -g http-server`

### Run

Change working directory to release folder, and run follow commands:

**Replace {USERNAME} {PASSWORD} with yours**

Windows Powershell

```powershell
$env:DATABASE_URL="postgres://{USERNAME}:{PASSWORD}@127.0.0.1:5432/tutorial"; `
Start-Process ".\teacher-service.exe"

$env:HOST_PORT="localhost:8080";Start-Process ".\svr.exe"

http-server dist
```

\*nix

```shell
DATABASE_URL="postgres://{USERNAME}:{PASSWORD}@127.0.0.1:5432/tutorial" \
./teacher-service
HOST_PORT="localhost:8080";./svr
```

## Build

```shell
git clone https://github.com/rexcape/rust_web_fullstack.git
cd rust_web_fullstack
```

### webservice

```shell
# directory rust_web_fullstack
cd webservice
cargo build --bin teacher-service --release
```

**Replace {USERNAME} {PASSWORD} with yours**

Windows Powershell

```powershell
$env:DATABASE_URL="postgres://{USERNAME}:{PASSWORD}@127.0.0.1:5432/tutorial";`
.\target\release\teacher-service.exe
```

\*nix

```shell
DATABASE_URL="postgres://{USERNAME}:{PASSWORD}@127.0.0.1:5432/tutorial" \
./target/release/teacher-service
```

### webapp

```shell
# directory rust_web_fullstack
cd webapp
cargo build --bin svr --release
```

Windows Powershell

```powershell
$env:HOST_PORT="localhost:8080"; `
.\target\release\svr.exe
```

\*nix

```shell
HOST_PORT="localhost:8080" \
target/release/svr
```

### wasm-client

```shell
# directory rust_web_fullstack
cd wasm-client
wasm-pack build

cd www
npm run build

npm install -g http-server
http-server dist
```

## Develop

```shell
git clone https://github.com/rexcape/rust_web_fullstack.git
cd rust_web_fullstack
```

### webservice

**Update the .env file under webservice folder**

```shell
# directory rust_web_fullstack
cd webservice
cargo run --bin teacher-service
```

### webapp

```shell
# directory rust_web_fullstack
cd webapp
cargo run --bin svr
```

### wasm-client

```shell
# directory rust_web_fullstack
cd wasm-client
wasm-pack build
cd www
npm run start
```
