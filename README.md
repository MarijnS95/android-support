# Android support code for Rust written in Java

Demonstrating how to:

1. Compile some Java code against `android.jar`;
2. Create a `.dex` file from the resulting `.class` [^1];
3. Build an APK containing the `.dex` file and some Rust code in a `.so`;
4. Call a static class function in the aforementioned class from Rust code via `jni`.

[^1]: Expecting that this is later extended with user code, `jar` files etc, all these raw inputs should likely be passed to `xbuild` in `manifest.yaml` so that it can call `d8` to combine everything at once.

## Usage

Install `xbuild` from a WIP branch with "dex packaging support":

```sh
cargo install --git https://github.com/rust-mobile/xbuild --branch classes-dex
```

```sh
x devices
x run -p example --device adb:<device identifier>
```
