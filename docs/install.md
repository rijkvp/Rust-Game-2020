# Installing

Follow these instuctions to install Rust & SDL2 needed for building.


# Linux

## Download the Rust Programming language

1. Download Rust by using the command from [the official Rust download page](https://www.rust-lang.org/tools/install).
2. Type '1' &  hit enter to proceed with the installation.

## Download SDL2

1. SDL2: `sudo apt-get install libsdl2-dev`

2. SDL2 TTF: `sudo apt-get install libsdl2-ttf-dev`

## Run the project

`cargo run`

# Windows

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

1. Download SDL2-devel-2.0.x-VC.zip from [the SDL website](http://www.libsdl.org/download-2.0.php).
2. Unpack to a folder of your choosing (You can delete it afterwards).
3. Copy all .lib files from:
`SDL2-devel-2.0.x-VC\SDL2-2.0.x\lib\x64\`
to
`C:\Users\{Your Username}\.rustup\toolchains\{current toolchain}\lib\rustlib\{current toolchain}\lib`. The DLL's are already copied to the repo.
4. To use SDL2 TTF you also need to download the SDL2 TFF libs. Download SDL2_ttf-devel-2.0.15-VC form [the SDL TTF website](https://www.libsdl.org/projects/SDL_ttf/).
5. Unpack to a folder of your choosing (You can delete it afterwards).
6. Copy all .lib files from:
`SDL2_ttf-devel-2.0.x-VC\SDL2_ttf-2.0.x\lib\x64`
to
`C:\Users\{Your Username}\.rustup\toolchains\{current toolchain}\lib\rustlib\{current toolchain}\lib`. The DLL's are already copied to the repo.

## Run the project

1. Open a command promt window (Win + R, type cmd & hit enter).
2. Type `cargo run` & hit enter.
3. If everything is installed correctly the project will run without any errors.
