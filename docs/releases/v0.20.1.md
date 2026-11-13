# CADCraft 0.20.1 for AROS

The first AROS release of CADCraft, a 2D CAD drafting program. It opens and saves DWG and DXF drawings, opens SVG, and exports PDF, SVG and PNG. It is built from CADCraft 0.20.1 for three AROS systems.

## Downloads

| File | System |
|---|---|
| `CADCraft-0.20.1-aros-x86_64-abiv11-pc.zip` | AROS x86_64, ABIv11 (AROS One 1.3) |
| `CADCraft-0.20.1-aros-x86_64-abiv1-pc.zip` | AROS x86_64, mainline ABIv1 (nightly 2026-11-10) |
| `CADCraft-0.20.1-aros-aarch64-abiv1-raspi.zip` | AROS on Raspberry Pi, AArch64 ABIv1 (mainline nightly 2026-11-10) |
| `CADCraft-0.20.1-source.zip` | Port sources, patches, pinned inputs and build recipe |

Each package runs only on the system it was built for. `SHA256SUMS` lists the checksums. The packages can also be installed with the AROS package manager: `apkg install cadcraft`.

## What is in it

- `CADCraft`, the drafting program, in one window on the Workbench screen: command line, menus, tool panels, layers, properties, layouts.
- `cadcraft-cli`, the command-line program: drawing information, conversion between DWG, DXF, SVG, PDF and PNG, command scripts, sample drawings, and an MCP server over standard input and output.
- The window is drawn by the processor or through OpenGL. The `GUI_RENDERER` ToolType or the `--gui-renderer` Shell argument chooses: `AUTO` (default), `CPU` or `GPU`. `AUTO` uses OpenGL when it is there and is not Mesa's software renderer; the reason for the choice is written to the log.
- Open and Save As use the standard file requester. Closing the window with unsaved changes asks whether to save them.
- Settings, the panel layout, autosave files and `CADCraft.log` are kept in `CADCraftData` inside the program drawer.
- Updates come through the AROS package manager.

## Installation

Unpack the archive for your system anywhere writable and double-click CADCraft. From a Shell, run `CADCraft [--gui-renderer CPU|AUTO|GPU] [file]`. The archive also holds a `.arospkg` drawer with the package manager's description; after unpacking by hand it can be deleted.

`CADCraft --version` and the first line of `CADCraft.log` name the build; please include them when you report a problem.

Tested on AROS One 1.3 (ABIv11), on mainline x86_64 AROS nightly 2026-11-10 and on the Raspberry Pi nightly 2026-11-10 in an emulator, not on a physical Raspberry Pi. On all three: installing, verifying and removing the package with the package manager (settings kept), starting from a Shell, opening DXF and DWG drawings, drawing, undo and redo, Save As and Save over the same file, the panel layout kept between starts, closing, and the command-line program (DXF, DWG, PNG, PDF and SVG output, command scripts, the MCP handshake). On mainline x86_64 and the Raspberry Pi also a start as a Workbench program. On mainline x86_64 also typing commands, the menus, the Save As requester and the question about unsaved changes. On the two x86_64 systems also a start with `GPU` (Mesa's software renderer).

## Known limitations

- Every change redraws the whole window, so on slow machines and in emulators the interface follows with a delay.
- The drawing area is drawn by the interface renderer, not by CADCraft's own GPU canvas; large drawings pan and zoom slowly.
- All drawings share one window, and floating panels stay inside it.
- The control server (`--control`) and 3D mice are not available. No automatic updates; use the AROS package manager.
- OpenGL: where only Mesa's software renderer is present, `AUTO` uses the CPU. `GPU` works with it on x86_64 and is several times slower. Hardware OpenGL drivers have not been tested.
- Not tested on AROS in this release: DWG files written by other programs, plotting and printing from the window, AutoLISP routines, external references, pictures in drawings, keyboard layouts other than the US one.
- Text in scripts other than Latin may be shown as boxes when no font for it is installed.

Licences and notices for CADCraft and everything built into it are in the `Licenses` drawer of each package.
