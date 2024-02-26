# Flutter Rust Bridge - Test cpal microphone on ios

[Flutter Rust Bridge](https://github.com/fzyzcjy/flutter_rust_bridge) project to investigate an [issue](https://github.com/RustAudio/cpal/issues/842) with [cpal](https://crates.io/crates/cpal).

**When the project runs, the number of in and output devices is logged to console.
On iOS the built in microhpone should always be accessible, but it cannot be found by cpal.**

### Versions:

`rustc --version` 
```
rustc 1.76.0 (07dca489a 2024-02-04)**
```

`flutter --version`
```
Flutter 3.19.0 • channel stable • https://github.com/flutter/flutter.git
Framework • revision bae5e49bc2 (13 days ago) • 2024-02-13 17:46:18 -0800
Engine • revision 04817c99c9
Tools • Dart 3.3.0 • DevTools 2.31.1
```

`cpal`
```
cpal = "0.15.2"
```

`flutter_rust_bridge_codegen --version`
```
flutter_rust_bridge_codegen 2.0.0-dev.24
```


# Requirements

* [install rust](https://www.rust-lang.org/tools/install)
  * add necessary targets: 
    * for android (if necessary): ```rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android```
  * install packages
    * to generate rust bindings (if necessary): ```cargo install flutter_rust_bridge_codegen```
* [install flutter](https://docs.flutter.dev/get-started/install/macos)

* test rust build
  * `cd rust`
  * `cargo build`
* test flutter build
  * `flutter build ios`


# Run

* run with `flutter run`
* or just open `ios/Runner.xcworkspace` and run on simulator
