# SKYULL CLI README
  <p>
    <a href="https://github.com/lnxdxtf/skyull/actions/workflows/skyull_tests_and_build.yml">
      <img  src="https://github.com/lnxdxtf/skyull/actions/workflows/skyull_tests_and_build.yml/badge.svg?branch=main"/></a>
    <a href="https://github.com/lnxdxtf/skyull/actions/workflows/skyull_docs.yml">
      <img  src="https://github.com/lnxdxtf/skyull/actions/workflows/skyull_docs.yml/badge.svg?branch=main"/>
    </a>
  </p>

SKYULL is a command-line interface (CLI) in development that creates REST API project structure templates with the aim of making it easy and fast to start a new project. With just a few primary configurations, such as project name, you can get started quickly.

## Default configuration for web-api options
- Rocket v0.5-rc
- Actix (actix-web 4)

## Other configuration options
- Redis
- MongoDB
- MySQL

## Installation

Before installing the SKYULL CLI on your OS, you need to have Rust and Cargo installed on your system. If you haven't already installed Rust and Cargo, please follow the installation instructions available on the official website: https://www.rust-lang.org/tools/install.

As the project is not yet published, you need to clone the repository and build the project in release mode using the following command:

```bash
cargo build --release
```
After building the project successfully, you can install the CLI using the following command:
```bash
cargo install --path .
```
Finally, restart your terminal and run the skyull command to start using the CLI.

## Usage
To use SKYULL CLI, simply run the following command:
```bash
skyull
```
This will prompt you to enter the project name and other necessary details. SKYULL will then generate a REST API project structure template for you, making it easy and fast to get started with your new project.

## License
This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing
Contributions to this project are welcome. If you find any issues or have any suggestions, please feel free to create a pull request or open an issue.

## Acknowledgments
Thanks to all the contributors who have helped to make this project a success.
