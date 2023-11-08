# Many Times Pad Attack

[![Build Status](https://github.com/emilk/eframe_template/workflows/CI/badge.svg)](https://github.com/SamPanDonte/many_time_pad/actions?workflow=CI)

This attack assumes that the message was encoded using the same key multiple times. Given the ciphertext and key length,
it will try to find the key and decode the message. After that, user can correct errors and as a result gain the
original message.

## Getting started

Start by clicking "Use this template" at https://github.com/emilk/eframe_template/ or
follow [these instructions](https://docs.github.com/en/free-pro-team@latest/github/creating-cloning-and-archiving-repositories/creating-a-repository-from-a-template).

Change the name of the crate: Chose a good name for your project, and change the name to it in:

* `main.rs`
    * Change `eframe_template::TemplateApp` to `your_crate::TemplateApp`

## Running native locally

Make sure you are using the latest version of stable rust by running `rustup update`.

`cargo run --release`

On Linux you need to first run:

`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`

On Fedora Rawhide you need to run:

`dnf install clang clang-devel clang-tools-extra libxkbcommon-devel pkg-config openssl-devel libxcb-devel gtk3-devel atk fontconfig-devel`

## Running web locally

You can compile your app to [WASM](https://en.wikipedia.org/wiki/WebAssembly) and publish it as a web page.

We use [Trunk](https://trunkrs.dev/) to build for web target.

1. Install the required target with `rustup target add wasm32-unknown-unknown`.
2. Install Trunk with `cargo install --locked trunk`.
3. Run `trunk serve` to build and serve on `http://127.0.0.1:8080`. Trunk will rebuild automatically if you edit the
   project.
4. Open `http://127.0.0.1:8080/index.html#dev` in a browser. See the warning below.

## Web Deploy

1. Just run `trunk build --release`.
2. It will generate a `dist` directory as a "static html" website
