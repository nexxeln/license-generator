<h1 align="center">License Generator</h1>

![demo](https://us-east-1.tixte.net/uploads/nexxel.needs.rest/idea64_rZlJLu450P.gif)

<p align="center">
  <img src="https://img.shields.io/crates/d/gen-license?color=%23b7410e" />
  <img src="https://img.shields.io/crates/v/gen-license" />
</p>

### Overview

This is a blazing fast ⚡, command line license generator for your open source projects written in Rust.

I know that GitHub has a great GUI to add licenses to projects but I always found myself doing too much work. First you have to go to GitHub, create a file, type 'LICENSE', pick a license, push it and then pull it locally. With this you can just generate the license locally and push it to GitHub.

I had not written some Rust code in a while so I made this simple project to brush up my Rust skills a bit (still very beginner).

### Installation

#### Prebuilt binaries

Prebuilt binaries are available on the [release](https://github.com/nexxeln/license-generator/releases/latest) page.

```bash
wget https://github.com/nexxeln/license-generator/releases/download/v0.3.0/gen-license-v0.3.0-x86_64-unknown-linux-musl.tar.gz
tar -xvf gen-license-v0.3.0-x86_64-unknown-linux-musl.tar.gz
mv gen-license ~/.local/bin
```

#### Using Cargo

```bash
cargo install gen-license
```

#### Using AUR (Arch/Arch-based Linux Distros)

```
yay -S gen-license
```

#### From source

```bash
git clone https://github.com/nexxeln/license-generator.git
cd license-generator
cargo install --path .
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

#### Creating a release

- Extensively test your changes and get them on `main`
- In a commit with only one change, update the version in `Cargo.toml`
- Tag the commit with the version number
  ```bash
  git tag -a v0.3.0 -m "v0.3.0"
  ```
- Push the tag
  ```bash
  git push --tags origin main
  ```
- Observe progress on the [Actions tab](https://github.com/nexxeln/license-generator/actions)
