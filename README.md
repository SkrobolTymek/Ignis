# Ignis

**Ignis** is a blazing fast, GPU-accelerated terminal emulator written in Rust.  
Designed for raw performance and a clean modern aesthetic.

## ⚡ Features (planned)

- [x] GPU rendering via `wgpu`
- [x] Basic VT parser & terminal emulation
- [ ] ANSI color support
- [ ] Scrollback buffer
- [ ] Copy/paste
- [ ] Configurable themes
- [ ] Unicode support

## 🛠 Building

### Linux

**Dependencies:**

- Rust (stable or nightly)
- CMake, pkg-config
- Libraries: `libxkbcommon`, `libwayland`, `libudev`, `mesa`, `x11` (depending on backend)

```bash

git clone https://github.com/FerrixDev/Ignis.git
cd Ignis
cargo build --release
./target/release/ignis
```

### Windows
**Dependencies:**

- Rust (MSVC toolchain)
- Visual Studio Build Tools (for CMake if needed)

```powershell
git clone https://github.com/FerrixDev/Ignis.git
cd Ignis
cargo build --release
.\target\release\ignis.exe
```

### Binaries will be available under the Releases tab for Windows users.

## 📦 Distribution Plans
 - AUR package (Arch Linux)

 - .deb package (Debian/Ubuntu)

 - Windows MSI/EXE installer

 - Flatpak/Snap

### ⚠ Status
Alpha stage — extremely early development
Expect bugs, missing features, and frequent changes.

### ✨ License
#### MIT

### Author
#### Ferrix - A low level dev who loves programming
