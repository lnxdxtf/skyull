# SKYULL CLI README
[![Crates.io](https://img.shields.io/crates/v/skyull?style=for-the-badge)](https://crates.io/crates/skyull)
[![Docs.rs](https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K)](https://lnxdxtf.github.io/skyull/skyull/index.html)

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

The crate is available on <a href="https://crates.io/crates/skyull">crates.io</a> so you can install it easily running:
```bash
cargo install skyull
```

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
