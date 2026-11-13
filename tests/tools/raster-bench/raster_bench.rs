//! Host measurement of the CPU renderer on CADCraft's own frames: the app runs without a window,
//! each frame is tessellated and painted by egui-aros's renderer as on AROS, and the frame is
//! compared with the previous one by 64x64 tiles (what a renderer that redraws only changed
//! tiles would still have to draw).
//!
//! cargo run --release -p cadcraft-aros --example raster_bench -- [WxH] [frames per scenario]

use std::time::{Duration, Instant};

use cadcraft_ui_egui::{CadApp, Services};
use egui::{Event, Key, Modifiers, MouseWheelUnit, PointerButton, Pos2, RawInput, Rect, Vec2};
use egui_aros::raster::Renderer;

const TILE: usize = 64;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (w, h) = args
        .get(1)
        .and_then(|s| s.split_once('x'))
        .and_then(|(a, b)| Some((a.parse::<usize>().ok()?, b.parse::<usize>().ok()?)))
        .unwrap_or((1000, 700));
    let per = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(30usize);

    let ctx = egui::Context::default();
    let mut app = CadApp::new(cadcraft_engine::Session::empty(), Services::default());
    let _ = app.run("ui.sample", serde_json::json!({}));
    let mut renderer = Renderer::new(w, h);
    renderer.measure = true;
    let mut tiled = Renderer::new(w, h);
    let mut prev: Option<Vec<u8>> = None;
    let start = Instant::now();
    let mut frame_no = 0u64;

    let screen = Rect::from_min_size(Pos2::ZERO, Vec2::new(w as f32, h as f32));
    let centre = Pos2::new(w as f32 * 0.5, h as f32 * 0.45);
    let scenarios: [(&str, Box<dyn Fn(usize) -> Vec<Event>>); 5] = [
        ("settle", Box::new(|_| Vec::new())),
        ("idle", Box::new(|_| Vec::new())),
        ("hover", Box::new(move |i| vec![Event::PointerMoved(centre + Vec2::new(i as f32 * 3.0, i as f32))])),
        (
            "type",
            Box::new(|i| {
                let word = "circle ";
                let c = word.chars().nth(i % word.len()).unwrap_or(' ');
                if i % word.len() == word.len() - 1 {
                    vec![Event::Key { key: Key::Escape, physical_key: None, pressed: true, repeat: false, modifiers: Modifiers::NONE }]
                } else {
                    vec![Event::Text(c.to_string())]
                }
            }),
        ),
        (
            "zoom",
            Box::new(move |i| {
                vec![
                    Event::PointerMoved(centre),
                    Event::MouseWheel {
                        unit: MouseWheelUnit::Line,
                        delta: Vec2::new(0.0, if (i / 5) % 2 == 0 { 1.0 } else { -1.0 }),
                        modifiers: Modifiers::NONE,
                        phase: egui::TouchPhase::Move,
                        source: egui::MouseWheelSource::Unknown,
                    },
                ]
            }),
        ),
    ];
    let _ = PointerButton::Primary;

    println!("window {w}x{h}, {per} frames per scenario; times on this host, not AROS");
    println!("{:<7} {:>9} {:>9} {:>9} {:>8} {:>7} {:>8} {:>8} {:>8} {:>8}", "", "tess ms", "raster", "of which", "tiles", "pixels", "solid", "atlas", "page", "blended");
    println!("{:<7} {:>9} {:>9} {:>9} {:>8} {:>7} {:>8} {:>8} {:>8} {:>8}", "", "", "ms", "clear ms", "changed", "changed", "Mpx", "Mpx", "Mpx", "Mpx");
    for (name, events) in &scenarios {
        let mut tess = Vec::new();
        let mut tiled_ms = Vec::new();
        let mut tiled_tiles = Vec::new();
        let mut worst_diff = 0u8;
        let mut raster = Vec::new();
        let mut clear = Vec::new();
        let mut tiles = Vec::new();
        let mut pixels = Vec::new();
        let mut kinds = [0u64; 3];
        let mut blended = 0u64;
        for i in 0..per {
            frame_no += 1;
            let raw = RawInput {
                screen_rect: Some(screen),
                time: Some(start.elapsed().as_secs_f64()),
                events: events(i),
                ..Default::default()
            };
            let mut raw = raw;
            app.raw_input_hook(&ctx, &mut raw);
            let out = ctx.run_ui(raw, |ui| {
                app.guarded_logic(ui.ctx());
                app.guarded_ui(ui);
            });
            let t0 = Instant::now();
            let prims = ctx.tessellate(out.shapes, out.pixels_per_point);
            let t1 = Instant::now();
            if i == per - 1 {
                shape_report(name, &prims, out.pixels_per_point);
            }
            let mut delta = out.textures_delta;
            renderer.update_textures(&delta);
            tiled.update_textures(&delta);
            let t2 = Instant::now();
            renderer.clear([0x21, 0x28, 0x30]);
            renderer.paint(&prims, out.pixels_per_point);
            let t3 = Instant::now();
            let t4 = Instant::now();
            let changed = tiled.paint_changed([0x21, 0x28, 0x30], &prims, out.pixels_per_point);
            tiled_ms.push(t4.elapsed());
            tiled_tiles.push(changed.repainted as f64 * 100.0 / changed.tiles.max(1) as f64);
            let worst = tiled.frame.iter().zip(&renderer.frame).map(|(a, b)| a.abs_diff(*b)).max().unwrap_or(0);
            worst_diff = worst_diff.max(worst);
            tiled.free_textures(&delta);
            renderer.free_textures(&delta);
            delta.clear();
            tess.push(t1 - t0);
            raster.push(t3 - t2);
            if i == per - 1 {
                let st = &renderer.stats;
                let names = ["page", "solid", "atlas"];
                for (k, s) in st.kinds.iter().enumerate() {
                    println!("  [{name}] {}: {} triangles, {} us, tested {} covered {} blended {} opaque {}", names[k], s.triangles, s.us, s.visited, s.covered, s.blended, s.opaque);
                }
                println!("  [{name}] rectangle fill {} px, runs {}", st.rect_pixels, st.runs);
            }
            clear.push(Duration::from_micros(renderer.stats.clear_us));
            for (k, s) in renderer.stats.kinds.iter().enumerate() {
                kinds[k] += s.covered;
                if k < 3 {
                    blended += s.blended;
                }
            }
            let (t, p) = diff(prev.as_deref(), &renderer.frame, w, h);
            tiles.push(t);
            pixels.push(p);
            prev = Some(renderer.frame.clone());
        }
        if let Ok(dir) = std::env::var("BENCH_DUMP") {
            let _ = std::fs::write(format!("{dir}/{name}-{w}x{h}.rgba"), &renderer.frame);
        }
        let ms = |v: &[Duration]| median(v.iter().map(|d| d.as_secs_f64() * 1000.0).collect());
        let n = per as f64 * 1e6;
        let total_tiles = w.div_ceil(TILE) * h.div_ceil(TILE);
        println!("  [{name}] changed tiles only: raster {:.2} ms (median), tiles repainted {:.1}% (median), largest difference from the full picture {worst_diff} level(s)", ms(&tiled_ms), median(tiled_tiles.clone()));
        println!(
            "{:<7} {:>9.2} {:>9.2} {:>9.2} {:>7.1}% {:>6.1}% {:>8.3} {:>8.3} {:>8.3} {:>8.3}",
            name,
            ms(&tess),
            ms(&raster),
            ms(&clear),
            median(tiles.iter().map(|&t| t as f64).collect()) * 100.0 / total_tiles as f64,
            median(pixels.iter().map(|&p| p as f64).collect()) * 100.0 / (w * h) as f64,
            kinds[1] as f64 / n,
            kinds[2] as f64 / n,
            kinds[0] as f64 / n,
            blended as f64 / n,
        );
    }
    let _ = frame_no;
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v.get(v.len() / 2).copied().unwrap_or(0.0)
}

/// Changed 64x64 tiles and changed pixels against the previous frame (all of them for the first).
fn diff(prev: Option<&[u8]>, cur: &[u8], w: usize, h: usize) -> (usize, usize) {
    let Some(prev) = prev else { return (w.div_ceil(TILE) * h.div_ceil(TILE), w * h) };
    let mut tiles = 0;
    let mut pixels = 0;
    for ty in 0..h.div_ceil(TILE) {
        for tx in 0..w.div_ceil(TILE) {
            let mut changed = 0;
            for y in ty * TILE..((ty + 1) * TILE).min(h) {
                let a = (y * w + tx * TILE) * 4;
                let b = (y * w + ((tx + 1) * TILE).min(w)) * 4;
                if prev[a..b] != cur[a..b] {
                    changed += prev[a..b].chunks_exact(4).zip(cur[a..b].chunks_exact(4)).filter(|(p, c)| p != c).count();
                }
            }
            if changed > 0 {
                tiles += 1;
                pixels += changed;
            }
        }
    }
    (tiles, pixels)
}

/// Area (pixels, before clipping) of solid-colour triangles, and the part of it that is opaque
/// axis-aligned rectangles (two triangles of one quad), and the largest triangles.
fn shape_report(name: &str, prims: &[egui::epaint::ClippedPrimitive], ppp: f32) {
    let (mut solid, mut rect, mut tris, mut big) = (0f64, 0f64, 0usize, 0usize);
    for p in prims {
        let egui::epaint::Primitive::Mesh(m) = &p.primitive else { continue };
        for t in m.indices.chunks_exact(3) {
            let v: Vec<&egui::epaint::Vertex> = t.iter().map(|&i| &m.vertices[i as usize]).collect();
            tris += 1;
            if !(v[0].uv == v[1].uv && v[1].uv == v[2].uv) {
                continue;
            }
            let area = ((v[1].pos.x - v[0].pos.x) * (v[2].pos.y - v[0].pos.y) - (v[1].pos.y - v[0].pos.y) * (v[2].pos.x - v[0].pos.x)).abs() as f64 * 0.5 * (ppp * ppp) as f64;
            solid += area;
            if area > 10_000.0 {
                big += 1;
            }
            let xs: std::collections::BTreeSet<i64> = v.iter().map(|q| (q.pos.x * 64.0) as i64).collect();
            let ys: std::collections::BTreeSet<i64> = v.iter().map(|q| (q.pos.y * 64.0) as i64).collect();
            let opaque = v.iter().all(|q| q.color.a() == 255) && v[0].color == v[1].color && v[1].color == v[2].color;
            if xs.len() == 2 && ys.len() == 2 && opaque {
                rect += area;
            }
        }
    }
    println!("  [{name}] triangles {tris}, solid area {:.2} Mpx, of it opaque axis-aligned rectangles {:.2} Mpx ({:.0}%), triangles over 10k px: {big}", solid / 1e6, rect / 1e6, rect * 100.0 / solid.max(1.0));
}
