# Minesweeper - Prover9

This is a simple configurable minesweeper game that you can play.

The objective of this project is to solve the puzzle using **Prover9**.

## Dev

```bash
yarn install
yarn tauri dev
```

If you encounter problems, please visit [Tauri Studio's getting started page](https://tauri.studio/en/docs/getting-started/intro).

## Dependencies

Every dependency required by `Tauri`. For a comprehensive list please see [this page](https://tauri.studio/en/docs/getting-started/intro).

Prover9 and Mace4 available anywhere on the system regardless of the context. Visit [this git repo](https://github.com/ai4reason/Prover9) for more details.

## Notes

Because Prover9 can only be built on linux, this is supposed to function correctly only on a linux environment.

I currently run it inside **WSL2**, on a **Windows11** machine, with *gpu passthrough* and **WslG** support, with a custom distro installed

**Relevant info:**

```text
OS: Manjaro 21.2.0 Qonos(on the Windows Subsystem for Linux)
Kernel: x86_64 Linux 5.10.60.1-microsoft-standard-WSL2
Packages: 935
Resolution: 1920x1080
WM: Weston WM
GTK Theme: Breeze [GTK2/3]
CPU: AMD Ryzen 7 4800H with Radeon Graphics @ 16x 2.894GHz
GPU: NVIDIA GeForce GTX 1650 Ti
```

## Images

![Light mode](/assets/light_mode.png)
![Night mode](/assets/night_mode.png)
![Light error](/assets/light_errors.png)
![Night error](/assets/night_errors.png)
