CADCraft @VERSION@ for AROS
===========================

CADCraft is a 2D CAD drafting program. It opens and saves DWG and DXF
drawings, opens SVG, and exports PDF, SVG and PNG. Commands are typed on a
command line or chosen from menus and tool panels; AutoLISP routines can be
loaded. This is its AROS port, based on CADCraft @UPSTREAM_VERSION@.

This package is for: @SYSTEM@.
It does not run on other AROS systems; use the package made for yours.

The drawer holds two programs:

  CADCraft       the drafting program
  cadcraft-cli   the command-line program: shows a drawing's contents,
                 converts between formats, runs command scripts, writes
                 the sample drawings and serves an MCP automation
                 interface over standard input and output


Installation
------------

Unpack the archive anywhere writable, for example to Work:. It creates the
drawer CADCraft and its icon. Both programs use only libraries that come
with AROS; nothing in the system is replaced or added.

To upgrade, unpack the new version to a new place (not over the old drawer),
then move the drawer CADCraftData from the old CADCraft drawer into the
new one to keep your settings.

Starting
--------

Workbench: double-click CADCraft. The program takes the stack it needs
itself; it also works from a Shell with the default stack.

Shell:

  CADCraft [--gui-renderer CPU|AUTO|GPU] [--window WxH] [file ...]
  CADCraft --sample

Files named on the command line are opened at start; --sample opens a
sample drawing. All drawings share one window, each on its own tab. Open
and Save As use the standard file requester. To close CADCraft, use the
window's close gadget; with unsaved changes it asks whether to save them.

  cadcraft-cli info plan.dxf
  cadcraft-cli convert plan.dwg plan.pdf
  cadcraft-cli run plan.dxf --script "circle 10,10 5" --save plan2.dxf
  cadcraft-cli sample bracket bracket.dxf
  cadcraft-cli --help

cadcraft-cli mcp runs an MCP server on standard input and output for a
client that starts it as a child process. It works on its own drawing, not
on the one open in the CADCraft window.

Settings and log
----------------

Settings, the panel layout, autosave files, the AutoLISP support folder and
the log file CADCraft.log are kept in the drawer CADCraftData inside the
CADCraft drawer; it is created at the first start. The variable
CADCRAFT_CONFIG_DIR names another drawer instead. CADCRAFT_LOG names
another log file.

Interface renderer
------------------

The ToolType GUI_RENDERER in the program's icon, or the Shell argument
--gui-renderer, chooses how the window is drawn. A Shell argument takes
priority; the icon is not read when the program is started from the Shell.
Values ignore letter case.

  AUTO  OpenGL if it opens and is not Mesa's software renderer, otherwise
        CPU; the reason is written to CADCraft.log (default)
  CPU   the processor draws the window; OpenGL is not opened
  GPU   OpenGL (the software renderer too), or the program stops with an
        error

Known limitations
-----------------

@LIMITATIONS@

Licences
--------

CADCraft is available under the MIT license or the Apache License 2.0, at
your option. DWG files are read and written with the acadrust library
(Mozilla Public License 2.0, unmodified). The texts, the notices for the
bundled material, and the licences of everything linked into the programs
are in the Licenses drawer. BUILD.txt identifies this build.

CADCraft:  https://github.com/storytold/cadcraft
AROS port: https://github.com/tomaszstaniak/aros-cadcraft
