# CADCraft 0.20.1 for AROS, revision 1

The AROS port of CADCraft 0.20.1, a 2D CAD drafting program. It opens and saves DWG and DXF drawings, opens SVG, and exports PDF, SVG and PNG. It is built for three AROS systems.

## Downloads

| File | System |
|---|---|
| `CADCraft-0.20.1-r1-aros-x86_64-abiv11-pc.zip` | AROS x86_64, ABIv11 distributions |
| `CADCraft-0.20.1-r1-aros-x86_64-abiv1-pc.zip` | AROS x86_64, mainline ABIv1 (nightly 2026-11-06 or later) |
| `CADCraft-0.20.1-r1-aros-aarch64-abiv1-raspi.zip` | AROS on Raspberry Pi, AArch64 ABIv1 (mainline nightly 2026-11-06 or later) |
| `CADCraft-0.20.1-r1-source.zip` | Port sources, patches, pinned inputs and build recipe |

Each package runs only on the system it was built for. `SHA256SUMS` lists the checksums. The packages can also be installed with the AROS package manager: `apkg install cadcraft`.

## What is new in revision 1

Revision 1 has the same CADCraft 0.20.1 with changes of the AROS port. `CADCraft --version` prints `0.20.1-r1`.

- The window reacts faster with the CPU renderer. Only the parts of the window that changed are drawn again, so typing, moving the pointer and the blinking cursor no longer redraw everything. In the emulators used for testing, a typical frame of a scripted session took 0.15 to 0.4 s before and takes about 0.02 s now. Changes of the whole window (scrolling, zooming, opening a dialog) are several times faster too, and still follow with a delay on slow machines.
- On mainline x86_64, `AUTO` now keeps to the CPU renderer. A program that had opened OpenGL there, even only to look at it, left its log file in use after it ended, and the Shell it was started from no longer redirected the output of later programs (seen on nightly 2026-11-10). `GPU` still opens OpenGL. This also applied to 0.20.1.
- The mainline x86_64 and Raspberry Pi packages need a nightly build from 2026-11-06 or later. On older ones the programs stop with a software failure at start; update the system first. This also applies to 0.20.1.

With the package manager: `apkg upgrade cadcraft`. Settings in `CADCraftData` are kept.

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

Revision 1 was tested on an ABIv11 distribution (AROS One 1.3), on mainline x86_64 AROS nightly 2026-11-10 and on the Raspberry Pi nightly 2026-11-10 in an emulator, not on a physical Raspberry Pi. On all three: an upgrade from 0.20.1 with the package manager (settings and a file of the user kept), then a scripted drawing session from a Shell with Save As, a second start and removing the package. Not repeated for revision 1: the hand tests of 0.20.1 (menus, the Save As requester, the question about unsaved changes, a start as a Workbench program, `GPU`).

## Known limitations

- Changes of the whole window (zooming, panning, opening a dialog) redraw all of it; on slow machines and in emulators these follow with a delay.
- The drawing area is drawn by the interface renderer, not by CADCraft's own GPU canvas; large drawings pan and zoom slowly.
- All drawings share one window, and floating panels stay inside it.
- The control server (`--control`) and 3D mice are not available. No automatic updates; use the AROS package manager.
- OpenGL: where only Mesa's software renderer is present, on mainline x86_64 and on the Raspberry Pi, `AUTO` uses the CPU. `GPU` works with it on x86_64 and is several times slower. Hardware OpenGL drivers have not been tested.
- Not tested on AROS in this release: DWG files written by other programs, plotting and printing from the window, AutoLISP routines, external references, pictures in drawings, keyboard layouts other than the US one.
- Text in scripts other than Latin may be shown as boxes when no font for it is installed.

Licences and notices for CADCraft and everything built into it are in the `Licenses` drawer of each package.
