# Pot-App Text Recognize Plugin Template Repository (Take [OCR Space](https://ocr.space/) for example)

### English | [简体中文](./README.md)

### This repository is a template repository. You can directly create a plugin repository from this repository when writing plugins

## Plugin Writing Guide

### 1. Create a plugin repository

- Create a new repository using this repository as a template
- Named `pot-app-recognize-plugin-<name>`，Eg: `pot-app-recognize-plugin-ocrspace`

### 2. Plugin information configuration

Edit the `info.json` file and modify the following fields:

- `id`: The unique ID of the plugin, which must start with `[plugin]`, for example `[plugin].com.pot-app.ocrspace`
- `homepage`: The homepage of the plugin, simply fill in your repository address, for example `https://github.com/pot-app/pot-app-recognize-plugin-template`
- `display`: The display name of the plugin, for example `OCR Space`
- `icon`: The icon of the plugin, for example `icon.png`
- `needs`: Dependencies required by the plugin. It is an array where each dependency is an object that includes the following fields:
  - `key`: The key of the dependency, corresponding to its name in the configuration file. For example,`apikey`.
  - `display`: The display name of the dependency as shown to users. For example,`API Key`.
  - `language`: Mapping between language codes used in Pot and language codes used when sending requests to plugins.

### 3. Plugin writing/compiling

Edit `src/lib.rs` to implement `recognize` function

#### Input parameters

```rust
    base64: &str, // base64 encoded image
    lang: &str, // language code
    needs: HashMap<String, String>, // Additional configuration information required by the plugin, defined by info.json
```

#### Return value

```rust
// Returns a String wrapped in Value
  return Ok(Value::String(result));
```

#### Test/Compile

```bash
cargo test --package plugin --lib -- tests --nocapture # run the test case
cargo build --release # Compile
```

### 4. Package pot Plugin

1. Find the `plugin.dll` (Windows)/`libplugin.dylib` (MacOS)/`libplugin.so` (Linux) file in the `target/release` directory and delete the prefix `lib`.

2. Compress the `plugin.dll`/`libplugin.dylib`/`libplugin.so`, with the `info.json` and icon files, into a zip file.

3. Rename the file as `<plugin id>.potext`, for example `[plugin].com.pot-app.ocrspace.potext`, to obtain the plugin required by pot.

## Automatic Compilation and Package

This repository is configured with Github Actions, which allows for automatic compilation and packaging of plugins after pushing.

Every time the commit is pushed to GitHub, actions will run automatically and upload the packaged plugin to artifacts. The packaged plugin can be downloaded from the actions page.

After each tag submission, actions will also run automatically and upload the packaged plugin to releases. The packaged plugin can be downloaded from the release page.

> Please note that you need to add a secret named `TOKEN` in the repository settings. The value should be a GitHub Token with appropriate permissions, which will be used for uploading releases.
