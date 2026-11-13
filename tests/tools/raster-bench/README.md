# raster-bench

Host measurement of egui-aros's CPU renderer on CADCraft's own frames: the app runs without a
window, each frame is tessellated and painted as on AROS, once in full and once with only the
changed tiles, and the two pictures are compared.

Copy `raster_bench.rs` to `work/cadcraft/apps/cadcraft-aros/examples/`, then:

    python3 scripts/craft.py host build --release -p cadcraft-aros --example raster_bench
    build/<target>/host-target/release/examples/raster_bench 780x540 30

Remove the copy afterwards: the work copy holds only the patch series. `BENCH_DUMP=<dir>` writes
each scenario's last frame as raw RGBA. Times are the host's, not AROS's; the guest measurement
is `tests/guest/run-perf`.
