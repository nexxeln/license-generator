<h1 align="center">License Generator</h1>

 add gif here later
 
### Overview

This is a blazing fast ⚡, command line license generator for your open source projects written in Rust.

I know that GitHub has a great GUI to add licenses to projects but I always found myself doing too much work. First you have to go to GitHub, create a file, type 'LICENSE', pick a license, push it and then pull it locally. With this you can just generate the license locally and push it to GitHub.

I had not written some Rust code in a while so I made this simple project to brush up my Rust skills a bit (still very beginner).


### Prerequisites

- A Rust development environment
- I may release pre-compiled binaries in the future

### Installation

- Clone the repository
  ```bash
  git clone https://github.com/nexxeln/license-generator.git
  ```
- Install the executable
  ```bash
  cd license-generator
  cargo install --path .
  ```

### Usage

- For Windows
   ```shell
   license_generator.exe
   ```
- For Linux and macOS
  ```bash
  license_generator
  ```

### Contributing
- Fork the repository
- Create a branch
  ```bash
  git checkout -b fix/amazingFix
  ```
- Commit your changes and push to your branch
  ```bash
  git commit -m "made an amazingFix"
  git push origin fix/amazingFix
  ```
- Open a pull request