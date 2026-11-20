# Guest scripts

An AmigaDOS script for one run on an AROS guest. Put the two programs from a build output,
`run-smoke`, `steps.json` and `mcp-in.txt` into one directory, unpack it to `RAM:cc` in the
guest and run `Execute RAM:cc/run-smoke <results directory>`. The results directory must
exist; the `.rc` files hold each command's return code.

- CLI: version, the built-in sample written as DXF, information, conversions to DWG, PNG, PDF
  and SVG, a scripted edit saved twice to the same file, and the MCP handshake over stdio.
- Window: a start with a DXF file and one with the sample, each ending with `--screenshot`;
  then `--test-steps steps.json` on a DWG file: a line and a circle, Save As, undo, redo, Save,
  two screenshots, quit. The saved drawing is read back with `cadcraft-cli info`.

In the AROS Shell an asterisk inside quotes is an escape character: a `*` in a command's text
is written `**`.

Other scripts, run the same way after `run-smoke` or on their own payload:

- `run-gl`: one start with `--gui-renderer GPU` and a screenshot.
- `run-pkg <results> <machine>`, then `run-pkg-remove`: the release archive installed, verified,
  used and removed with apkg from a local index. Payload in `RAM:ctest`: `apkg`, `cc.zip` (the
  archive), `idx.json` (a test index naming it), `steps-pkg.json`, `steps-pkg2.json`.
- `run-public <results> <machine>`: the same from the public catalogue; needs the network.
- `run-probe`: `tests/tools/open-probe` (built with `craft.py build --tool open-probe`), which
  prints the error a missing file gives when it is opened.
