# Installing

These are the installation instructions to install Rust & run the project on a Windows computer using the VS build tools.

## Download the Rust Programming language

1. Download `rustup-ini.exe` from [the official Rust download page](https://www.rust-lang.org/tools/install).
2. Run it and type '1' &  hit enter to proceed with the installation.

## Download the Visual Studio 2019 build tools

1. Download the Build Tools for Visual Studio 2019 from [the Visual Studio website](https://visualstudio.microsoft.com/downloads/#vstool-2019-family).
2. Select the C++ build tools and install

## Set rustup toolchain to stable-x86_64-pc-windows-msvc

1. Open a command promt window (Win + R, type cmd & hit enter).
2. Type `rustup default stable-x86_64-pc-windows-msvc` & hit enter to set `stable-x86_64-pc-windows-msvc` as the default toolchain.
3. Type `rustup show` & hit enter to verify that `stable-x86_64-pc-windows-msvc` is set as the default toolchain.

## Download SDL2 libs

1. Download SDL2-devel-2.0.x-mingw.tar.gz from [the SDL website](http://www.libsdl.org/download-2.0.php).
2. Unpack to a folder of your choosing (You can delete it afterwards).
3. Copy all .lib files from:
`SDL2-devel-2.0.x-mingw\SDL2-2.0.x\x86_64-w64-mingw32\lib`
to
`C:\Users\{Your Username}\.rustup\toolchains\{current toolchain}\lib\rustlib\{current toolchain}\lib`.

## Run the project

1. Open a command promt window (Win + R, type cmd & hit enter).
2. Type `cargo run` & hit enter.
3. If everything is installed correctly the project will run without any errors.
