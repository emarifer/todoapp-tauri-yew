## Todoapp Tauri + Yew.

Desktop Task list application developed with Tauri and Yew.

---

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

---

### Screenshot:

<img src="https://user-images.githubusercontent.com/68773736/219088530-b1f7fc2b-17e1-4d36-9169-637b824b4f0f.png" width="75%">

---

#### As prerequisites, in addition to the Rust language and some OS-dependent libraries required for Tauri, you must also install the build target for browser-based WebAssembly called "wasm32-unknown-unknown" and the "Trunk" tool to the deployment and packaging of the Yew frontend:

```
rustup target add wasm32-unknown-unknown && cargo install --locked trunk
```

#### On the other hand, if we want to start from scratch, to create the scaffolding of the Tauri + Yew application it is necessary to install the Tauri app creation tool for the Cargo package manager and the Tauri CLI:

```
cargo install create-tauri-app && cargo install tauri-cli
```

#### Finally, since we use the Tailwind CSS framework, we will have to run in the root of the project:

```
npm i
```

#### With all this accomplished, run the app under development with the command:

```
cargo tauri dev
```

#### or build it with the command:

```
cargo tauri build
```

#### For more information see the documentation of [Tauri](https://tauri.app/) and [Yew](https://yew.rs/).
