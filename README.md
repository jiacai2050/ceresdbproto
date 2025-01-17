# Protocol of [CeresDB](https://github.com/CeresDB/ceresdb)

[![Crates.io](https://img.shields.io/crates/v/ceresdbproto.svg)](https://crates.io/crates/ceresdbproto)
[![Go Reference](https://pkg.go.dev/badge/github.com/CeresDB/ceresdbproto.svg)](https://pkg.go.dev/github.com/CeresDB/ceresdbproto)

Protocol buffer files for CeresDB. Projects that manage generated code of different languages are also in this repository. They acts as underlying dependency of client, server and meta.

---

## User Guide

### Rust

```sh
cargo add ceresdbproto
```

### Golang

```sh
go get github.com/CeresDB/ceresdbproto/golang
```

### Java

Add a maven dependency to your project.

```xml
<dependency>
    <groupId>io.ceresdb</groupId>
    <artifactId>ceresdb-proto-internal</artifactId>
    <version>1.0.0</version>
</dependency>
```

---

## Developer Guide
After modifying the proto files, something else for different programming languages should be done.

### Java
1. Name the maven project to a new version.
2. Rebuild the maven project lies in the `java` directory (During the build process, java code for the latest proto will be generated).
2. Publish the build result to the central maven repository.

### Rust
As for the Rust projects that depends on this project, everything will be generated during the build process of themselves. So nothing else needs to be done for `Rust`.

### Golang
1. Install the prerequisites:
* Install [Protocol Buffers (above 3.20.1)](https://github.com/protocolbuffers/protobuf/releases) compiler.
2. Execute `make build`.
