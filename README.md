<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->

<a name="readme-top"></a>

<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star!
*** Thanks again! Now go create something AMAZING! :D
-->

<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <!-- <a href="https://github.com/deepanchal/dnote-tui"> -->
  <!--   <img src="images/logo.png" alt="Logo" width="80" height="80"> -->
  <!-- </a> -->

<h3 align="center">Dnote TUI (Work In Progress)</h3>

  <p align="center">
    TUI for dnote (https://www.getdnote.com/)
    <br />
    <a href="https://docs.rs/dnote-tui"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/deepanchal/dnote-tui">View Demo</a>
    ·
    <a href="https://github.com/deepanchal/dnote-tui/issues">Report Bug</a>
    ·
    <a href="https://github.com/deepanchal/dnote-tui/issues">Request Feature</a>
  </p>
</div>

<!-- ABOUT THE PROJECT -->

## 🌟 About The Project

> Note: This is still a **WORK IN PROGRESS**

![Demo](./.github/static/demo.gif)

A TUI (Terminal User Interface) for [`dnote`](https://www.getdnote.com/).

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### 🏗️ Built With

- [`rust`](https://www.rust-lang.org/)
- [`ratatui`](https://github.com/ratatui-org/ratatui)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- GETTING STARTED -->

## 🚀 Getting Started

This is an example of how you may give instructions on setting up your project locally.
To get a local copy up and running follow these simple example steps.

### 🛠️ Prerequisites

1. Install rust: https://www.rust-lang.org/
2. Install dnote: https://github.com/dnote/dnote#installation

### 📥 Installing

Install this project with `cargo`

```bash
cargo install dnote-tui
```

If you wish to install from the main branch directly, use

```bash
cargo install --git https://github.com/deepanchal/dnote-tui
```

Once it's installed, simply run

```bash
dnote-tui
```

I will offer additional installation methods once the project reaches a more stable state.

### 🏃 Running project locally

1. Clone the repo.

```bash
git clone https://github.com/deepanchal/dnote-tui.git
```

2. cd into repo.

```bash
cd dnote-tui
```

3. Run tui

```sh
cargo run
```

### 🐳 Try with Docker

Get started with dnote-tui quickly using Docker! This setup comes with pre-loaded test data so you can dive right into exploring the TUI features.

1. Clone the repository and navigate to the project directory

```bash
git clone https://github.com/deepanchal/dnote-tui.git
cd dnote-tui
```

2. To build the Docker image and run the dnote-tui container using Docker Compose, execute the following command:

```bash
docker compose run --rm dnote-tui
```

Or if you prefer to use Docker directly without Docker Compose, you can follow these steps:

```bash
# Build the docker image
docker build --build-arg RUST_VERSION=1.76.0 --build-arg DNOTE_VERSION=0.15.1 -t dnote-tui .
# Run the docker container
docker run --rm -it dnote-tui
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- USAGE EXAMPLES -->

## 📝 Usage

### 🏃 Run

```sh
cargo run
```

### 🏗️ Build

```sh
cargo build
```

### 🧪 Run Tests

```sh
cargo test
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- ROADMAP -->

## 🎯 Roadmap

- [x] Show basic TUI with three sections (Books, Pages, & Content)
- [x] Create a dnote client
  - [x] Parse output from dnote cli to structs used by dnote client
  - [x] Add methods to perform write operations with dnote cli in dnote client
- [ ] Features
  - [x] Read Operations
    - [x] Read books from dnote cli into tui
    - [x] Read pages from dnote cli into tui
    - [x] Read pages content from dnote cli into tui
    - [x] Show truncated pages content beside page id in pages section
  - [ ] Write Operations
    - [ ] Create a new book with page [#19](https://github.com/deepanchal/dnote-tui/issues/19)
    - [x] Edit a page's content [#37](https://github.com/deepanchal/dnote-tui/issues/37)
    - [x] Add new page to a book [#49](https://github.com/deepanchal/dnote-tui/issues/49)
    - [ ] Move page to another book [#60](https://github.com/deepanchal/dnote-tui/issues/60)
    - [x] Delete a book's page [#51](https://github.com/deepanchal/dnote-tui/issues/51)
    - [x] Delete a book and all its pages [#52](https://github.com/deepanchal/dnote-tui/issues/52)
    - [ ] Rename book with popup input field [#59](https://github.com/deepanchal/dnote-tui/issues/59)
  - [ ] Search and Navigation
    - [ ] Find note within a book [#61](https://github.com/deepanchal/dnote-tui/issues/61)
    - [ ] Global note search [#47](https://github.com/deepanchal/dnote-tui/issues/47)
    - [ ] Make page content scrollable [#54](https://github.com/deepanchal/dnote-tui/issues/54)
  - [ ] Misc (Nice to have)
    - [ ] Allow note syncing with dnote server [#50](https://github.com/deepanchal/dnote-tui/issues/50)
    - [x] Parse keybindings for active mode from current config file and show on footer [#64](https://github.com/deepanchal/dnote-tui/issues/64)
    - [ ] Add ASCII art in header pane [#57](https://github.com/deepanchal/dnote-tui/issues/57)
    - [ ] Show command log pane to show user what commands are being run just like lazygit [#63](https://github.com/deepanchal/dnote-tui/issues/63)
    - [ ] Render note's markdown content with glow and make it toggleable [#53](https://github.com/deepanchal/dnote-tui/issues/53)

See the [open issues](https://github.com/deepanchal/dnote-tui/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTRIBUTING -->

## 🤝 Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feat/amazing-feature`)
3. Commit your Changes (`git commit -m 'feat: add some amazing-feature'`)
4. Push to the Branch (`git push origin feat/amazing-feature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- LICENSE -->

## 📄 License

Distributed under the MIT License. See [`LICENSE`](LICENSE) for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

[contributors-shield]: https://img.shields.io/github/contributors/deepanchal/dnote-tui.svg?style=for-the-badge
[contributors-url]: https://github.com/deepanchal/dnote-tui/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/deepanchal/dnote-tui.svg?style=for-the-badge
[forks-url]: https://github.com/deepanchal/dnote-tui/network/members
[stars-shield]: https://img.shields.io/github/stars/deepanchal/dnote-tui.svg?style=for-the-badge
[stars-url]: https://github.com/deepanchal/dnote-tui/stargazers
[issues-shield]: https://img.shields.io/github/issues/deepanchal/dnote-tui.svg?style=for-the-badge
[issues-url]: https://github.com/deepanchal/dnote-tui/issues
[license-shield]: https://img.shields.io/github/license/deepanchal/dnote-tui.svg?style=for-the-badge
[license-url]: https://github.com/deepanchal/dnote-tui/blob/main/LICENSE
