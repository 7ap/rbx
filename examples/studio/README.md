# 📖 studio

> ⚠️ YOU MUST BE ABLE TO SOLVE [THIS](https://www.k5learning.com/worksheets/kindergarten-reading-comprehension-matching-sentences-pictures-1.pdf) BEFORE CONTINUING!

This is the example that is used to do "example-driven-development" against [rbx](https://github.com/7ap/rbx). It will constantly be modified and requires a slight amount of setup to get started.

## 🏗️ Setup

1. Install [attach](https://lib.rs/crates/attach) with `cargo install attach`
2. Open [rbx](https://github.com/7ap/rbx) with [Visual Studio Code](https://code.visualstudio.com/), we have an integrated build + launch task for this.

3. Open `rbx/.vscode/tasks.json`, and change `STUDIO_PATH`, `EXAMPLE_LIB`, and `RUST_LOG` respectively.
    - `STUDIO_PATH` should be the set to the absolute path of your Roblox Studio installation (the folder with "version" in it).
    -  `EXAMPLE_LIB` should be the absolute path to `studio.dll`, this is typically `rbx/target/debug/studio.dll`, however you'll need to convert that to an absolute path.
    - `RUST_LOG` is the log level. You won't need to change this - however it's there if you want to change it.
4. Press `F5`.
5. Wait ~10 seconds. `attach` should wait 10 seconds before injecting to account for the time studio takes to start up. You should see a console attach to Roblox Studio when everything has completed.
