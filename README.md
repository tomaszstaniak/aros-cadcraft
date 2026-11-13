# CADCraft for AROS

An AROS port of [CADCraft](https://github.com/storytold/cadcraft), a 2D CAD drafting program. It opens and saves DWG and DXF drawings, opens SVG, and exports PDF, SVG and PNG.

## Download and run

[Download 0.20.1](https://github.com/tomaszstaniak/aros-cadcraft/releases/tag/v0.20.1) for one of these systems, or install `cadcraft` with arospkg:

| Package | System |
|---|---|
| `CADCraft-0.20.1-aros-x86_64-abiv11-pc.zip` | AROS x86_64, ABIv11 distributions |
| `CADCraft-0.20.1-aros-x86_64-abiv1-pc.zip` | AROS x86_64, mainline ABIv1 (nightly 2026-11-10) |
| `CADCraft-0.20.1-aros-aarch64-abiv1-raspi.zip` | AROS on Raspberry Pi, AArch64 ABIv1 (mainline nightly 2026-11-10) |

Each package runs only on the system it was built for and includes CADCraft and `cadcraft-cli`. Unpack the archive anywhere writable and double-click the CADCraft icon; from a Shell, run `CADCraft [file]`. The program takes the stack it needs itself. `ReadMe.txt` in the drawer describes installation, the command line and the interface renderer.

## Known limitations

The whole window is redrawn on every change, so the interface follows with a delay on slow machines and in emulators. The drawing area is drawn by the interface renderer (the processor, or OpenGL where a hardware driver is present), not by CADCraft's own GPU canvas. All drawings share one window. Hardware OpenGL was not tested in this release, and the Raspberry Pi build was tested only in an emulator. The [release notes](docs/releases/v0.20.1.md) list the rest.

## Build from source

This repository contains source revisions, AROS patches and build scripts. See [Building](docs/development.md), [Patch workflow](docs/patching.md) and [Source revisions](upstreams.json). The window, clipboard and start-up code use [egui-aros](https://github.com/tomaszstaniak/egui-aros), pinned in `upstreams.json`, which also holds the build tools shared by the Craft ports; the Rust standard library for AROS comes from [rust-aros](https://github.com/tomaszstaniak/rust-aros).

The repository stores build recipes rather than a full upstream source tree. The source archive of each release adds the pinned inputs. Licenses and author notices are included in the source and application archives.
