# Aeron Media Driver

## Overview

This project includes a Media Driver binary, which can be built and executed using Rust and Cargo. The Media Driver is designed to interact with the [Aeron](https://github.com/real-logic/aeron) messaging system.

## Prerequisites

Before you can run the Media Driver, you need to have Rust and Cargo installed. Please follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to set up your environment.

## Building the Project

To build the project, execute the following command in the root directory of the project:

```sh
cargo build --release
```

## Running the Media Driver

After successfully building the project, you can run the Media Driver using the following command:

```sh
cargo run --release --bin media-driver
```

## Configuration

You might need to configure certain aspects of the Media Driver for your specific use case. This typically involves setting environment variables or command-line arguments. Please refer to the Aeron documentation for more details on configuring the Media Driver.

## Libaeron-sys

This project uses a fork of the `libaeron-sys` crate to ensure version compatibility. Make sure to clone and build the appropriate version of the `libaeron-sys` fork that matches your project requirements.

## License

This project is licensed under the terms of the MIT license. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

We would like to thank the Aeron community for their continuous support and contributions.