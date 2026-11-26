//! Helvilette as geometry, converted to colored glyphs (ADR-0003).
//!
//! All coordinates are pixels of the original 2752x1536 artwork. The artwork is
//! the model for the trace; it is not part of the program. The outline, eyes and
//! face come from naughtian-website's measurements; the tail, legs, nose, rings
//! and glitch bars were traced from the artwork on a 50 px grid.
//!
//! Per frame: each character cell takes 2x2 samples of the scene. Neon lines
//! fade with distance, so they stay visible at any scale. A cell whose samples
//! show an edge gets a line glyph, other cells get a glyph from a density ramp.

pub type P = (f32, f32);
type Rgb = [f32; 3];

pub const FACE: P = (1386.0, 720.0);
pub const EYES: [P; 2] = [(1317.0, 705.0), (1455.0, 707.0)];
const EYE_R: P = (35.0, 31.0);
/// Left, top, right, bottom of the scene.
const VIEW: [f32; 4] = [1030.0, 380.0, 1790.0, 1450.0];

const CYAN: Rgb = [0.37, 0.95, 1.0];
const VIOLET: Rgb = [0.70, 0.45, 1.0];
const PINK: Rgb = [1.0, 0.40, 0.85];
const EYE: Rgb = [0.62, 0.90, 1.0];

/// Silhouette of the sitting cat. Index 0 and 3 are the ear tips.
const OUTLINE: [P; 25] = [
    (1265.0, 542.0),
    (1340.0, 612.0),
    (1445.0, 608.0),
    (1518.0, 548.0),
    (1540.0, 650.0),
    (1548.0, 730.0),
    (1530.0, 790.0),
    (1520.0, 815.0),
    (1540.0, 860.0),
    (1580.0, 940.0),
    (1620.0, 1040.0),
    (1645.0, 1160.0),
    (1645.0, 1270.0),
    (1625.0, 1350.0),
    (1580.0, 1375.0),
    (1270.0, 1375.0),
    (1240.0, 1340.0),
    (1230.0, 1240.0),
    (1240.0, 1120.0),
    (1260.0, 1000.0),
    (1270.0, 900.0),
    (1260.0, 820.0),
    (1245.0, 780.0),
    (1238.0, 710.0),
    (1245.0, 630.0),
];
pub const EAR_TIPS: [P; 2] = [OUTLINE[0], OUTLINE[3]];
const TAIL: [P; 8] = [
    (1240.0, 1180.0),
    (1200.0, 1185.0),
    (1165.0, 1210.0),
    (1150.0, 1250.0),
    (1150.0, 1300.0),
    (1165.0, 1340.0),
    (1200.0, 1362.0),
    (1250.0, 1378.0),
];
const NOSE: [[P; 3]; 2] = [
    [(1370.0, 735.0), (1408.0, 735.0), (1389.0, 758.0)],
    [(1389.0, 758.0), (1369.0, 786.0), (1409.0, 786.0)],
];
pub const MOUTH: P = (1389.0, 806.0);
/// Glitch bars of the artwork: left, top, right, bottom.
const BARS: [[f32; 4]; 7] = [
    [1205.0, 925.0, 1370.0, 955.0],
    [1445.0, 1010.0, 1632.0, 1040.0],
    [1455.0, 1120.0, 1665.0, 1135.0],
    [1200.0, 1035.0, 1305.0, 1045.0],
    [1265.0, 815.0, 1310.0, 822.0],
    [1430.0, 832.0, 1555.0, 842.0],
    [1550.0, 1280.0, 1650.0, 1290.0],
];
const RING_C: P = (1388.0, 714.0);
const RINGS: [f32; 2] = [232.0, 300.0];
/// Front legs, from the leg lines of the artwork: the centre of each leg goes
/// from the shoulder past the bend to the wrist.
const SHOULDERS: [P; 2] = [(1318.0, 1105.0), (1515.0, 1065.0)];
const LEG_BENDS: [P; 2] = [(1312.0, 1205.0), (1482.0, 1192.0)];
/// Wrists at rest. The round paw below each wrist sits on the keyboard.
pub const PAW_REST: [P; 2] = [(1318.0, 1270.0), (1450.0, 1270.0)];
/// Half of the leg width at the shoulder and at the wrist.
const LEG_HW: P = (40.0, 26.0);
/// Height of the paw below the wrist.
const PAW_H: f32 = 24.0;
/// Width of a leg line.
const LEG_W: f32 = 7.0;
pub const MOUSE_REST: P = (1700.0, 1358.0);
const MOUSE_R: P = (30.0, 42.0);
pub const KEYBOARD: [f32; 4] = [1190.0, 1312.0, 1590.0, 1404.0];

/// Everything that moves. `Pose::default()` is the cat at rest.
#[derive(Clone, Debug)]
pub struct Pose {
    /// Seconds since start, for the ring shimmer.
    pub t: f32,
    /// Offset of the head.
    pub head: P,
    /// Extra offset of each ear tip.
    pub ears: [P; 2],
    /// 0 is open, 1 is closed.
    pub lid: f32,
    /// Closed eyes curve up (content) instead of down (asleep).
    pub happy: bool,
    /// Pupil half-width: 13 is round, 3.5 is a slit.
    pub pupil: f32,
    /// The point that the eyes look at.
    pub gaze: P,
    /// Wrists of the front legs.
    pub paws: [P; 2],
    pub mouse: P,
    /// 0 is closed, 1 is open.
    pub mouth: f32,
    /// A pressed key, as a fraction of the keyboard width.
    pub key: Option<f32>,
    /// The pointer, which lights the glyphs near it.
    pub lamp: Option<P>,
    /// A torn row: image y and the shift in cells.
    pub glitch: Option<(f32, i32)>,
}

impl Default for Pose {
    fn default() -> Self {
        Pose {
            t: 0.0,
            head: (0.0, 0.0),
            ears: [(0.0, 0.0); 2],
            lid: 0.0,
            happy: false,
            pupil: 13.0,
            gaze: FACE,
            paws: PAW_REST,
            mouse: MOUSE_REST,
            mouth: 0.0,
            key: None,
            lamp: None,
            glitch: None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Cell {
    pub ch: char,
    pub rgb: [u8; 3],
}

pub const BLANK: Cell = Cell {
    ch: ' ',
    rgb: [0, 0, 0],
};

/// Maps character cells to artwork pixels. A cell is two times as tall as it is wide.
#[derive(Clone, Copy, Debug)]
pub struct Layout {
    pub cols: u16,
    pub rows: u16,
    s: f32,
    ox: f32,
    oy: f32,
}

impl Layout {
    pub fn new(cols: u16, rows: u16) -> Self {
        let [x0, y0, x1, y1] = VIEW;
        let s = ((x1 - x0) / cols as f32).max((y1 - y0) / (rows as f32 * 2.0));
        let ox = (x0 + x1) / 2.0 - cols as f32 * s / 2.0;
        let oy = (y0 + y1) / 2.0 - rows as f32 * s;
        Layout {
            cols,
            rows,
            s,
            ox,
            oy,
        }
    }

    /// Centre of a cell, in artwork pixels.
    pub fn to_image(self, col: u16, row: u16) -> P {
        (
            self.ox + (col as f32 + 0.5) * self.s,
            self.oy + (row as f32 + 0.5) * 2.0 * self.s,
        )
    }

    /// The cell that contains an artwork pixel (it can be outside the grid).
    pub fn to_cell(self, p: P) -> (i32, i32) {
        (
            ((p.0 - self.ox) / self.s).floor() as i32,
            ((p.1 - self.oy) / (2.0 * self.s)).floor() as i32,
        )
    }
}

const RAMP: &[u8] = b" .:-=+*#%@";

/// Glyphs for 2x2 sample patterns that read better as a line than as a shade.
/// Bits: 1 top-left, 2 top-right, 4 bottom-left, 8 bottom-right.
fn edge_glyph(bits: usize) -> Option<char> {
    Some(match bits {
        9 => '\\',
        6 => '/',
        5 | 10 => '|',
        3 => '-',
        12 => '_',
        1 => '\'',
        2 => '`',
        4 => ',',
        8 => '.',
        _ => return None,
    })
}

/// Draws the scene for a pose. The result has `cols * rows` cells, row by row.
pub fn render(layout: &Layout, pose: &Pose) -> Vec<Cell> {
    let scene = Scene::new(pose, layout.s);
    let (cols, rows) = (layout.cols as usize, layout.rows as usize);
    let mut cells = vec![BLANK; cols * rows];
    for r in 0..rows {
        for c in 0..cols {
            let mut sub = [0.0f32; 4];
            let mut sum = [0.0f32; 3];
            for (k, l) in sub.iter_mut().enumerate() {
                let x = layout.ox + (c as f32 + 0.25 + 0.5 * (k & 1) as f32) * layout.s;
                let y = layout.oy + (r as f32 + 0.25 + 0.5 * (k >> 1) as f32) * 2.0 * layout.s;
                let mut rgb = scene.sample((x, y));
                if let Some(p) = pose.lamp {
                    let d2 = (x - p.0).powi(2) + (y - p.1).powi(2);
                    let gain = 0.8 + 0.5 * (-d2 / (180.0f32 * 180.0)).exp();
                    rgb.iter_mut().for_each(|v| *v *= gain);
                }
                *l = rgb[0].max(rgb[1]).max(rgb[2]).min(1.0);
                (0..3).for_each(|i| sum[i] += rgb[i]);
            }
            cells[r * cols + c] = glyph(sub, sum);
        }
    }
    if let Some((y, off)) = pose.glitch {
        let (_, row) = layout.to_cell((0.0, y));
        for r in [row, row + 1] {
            if (0..rows as i32).contains(&r) {
                let line = &mut cells[r as usize * cols..(r as usize + 1) * cols];
                let old = line.to_vec();
                for (c, cell) in line.iter_mut().enumerate() {
                    let from = c as i32 - off;
                    *cell = if (0..cols as i32).contains(&from) {
                        old[from as usize]
                    } else {
                        BLANK
                    };
                }
            }
        }
    }
    cells
}

/// Selects a glyph and its color from the 2x2 samples of a cell.
fn glyph(sub: [f32; 4], sum: Rgb) -> Cell {
    let mx = sub.iter().cloned().fold(0.0, f32::max);
    let mn = sub.iter().cloned().fold(9.0, f32::min);
    let b = sub.iter().sum::<f32>() / 4.0;
    let mut ch = None;
    let mut level = b;
    if mx > 0.12 && mx - mn > 0.2 {
        let mid = (mx + mn) / 2.0;
        let bits = (0..4).filter(|&k| sub[k] > mid).map(|k| 1 << k).sum();
        ch = edge_glyph(bits);
        if ch.is_some() {
            level = mx * 0.8;
        }
    }
    if ch.is_none() && b > 0.04 {
        let i = (b.min(1.0).powf(0.8) * (RAMP.len() - 1) as f32)
            .round()
            .max(1.0) as usize;
        ch = Some(RAMP[i] as char);
    }
    let Some(ch) = ch else { return BLANK };
    let m = sum[0].max(sum[1]).max(sum[2]).max(1e-4);
    let a = (0.22 + level * 1.15).min(1.0);
    let rgb = sum.map(|v| (v / m * 255.0 * a) as u8);
    Cell { ch, rgb }
}

/* -- geometry ------------------------------------------------------------- */

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn mix(a: Rgb, b: Rgb, t: f32) -> Rgb {
    [
        lerp(a[0], b[0], t),
        lerp(a[1], b[1], t),
        lerp(a[2], b[2], t),
    ]
}

/// The neon of the artwork: cyan on the left, violet, then pink on the right.
fn neon(x: f32) -> Rgb {
    let t = ((x - 1230.0) / 420.0).clamp(0.0, 1.0);
    if t < 0.6 {
        mix(CYAN, VIOLET, t / 0.6)
    } else {
        mix(VIOLET, PINK, (t - 0.6) / 0.4)
    }
}

fn add(c: &mut Rgb, color: Rgb, k: f32) {
    (0..3).for_each(|i| c[i] += color[i] * k);
}

fn seg_dist(p: P, a: P, b: P) -> f32 {
    seg_dist_t(p, a, b).0
}

/// Distance from `p` to the segment, and where the nearest point is (0 at `a`, 1 at `b`).
fn seg_dist_t(p: P, a: P, b: P) -> (f32, f32) {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let len2 = dx * dx + dy * dy;
    let t = if len2 > 0.0 {
        (((p.0 - a.0) * dx + (p.1 - a.1) * dy) / len2).clamp(0.0, 1.0)
    } else {
        0.0
    };
    (
        ((p.0 - a.0 - t * dx).powi(2) + (p.1 - a.1 - t * dy).powi(2)).sqrt(),
        t,
    )
}

fn inside_poly(p: P, poly: &[P]) -> bool {
    let mut inside = false;
    let mut j = poly.len() - 1;
    for i in 0..poly.len() {
        let (a, b) = (poly[i], poly[j]);
        if (a.1 > p.1) != (b.1 > p.1) && p.0 < (b.0 - a.0) * (p.1 - a.1) / (b.1 - a.1) + a.0 {
            inside = !inside;
        }
        j = i;
    }
    inside
}

/// 0 at the centre of an ellipse, 1 on its edge.
fn ellipse_q(p: P, c: P, r: P) -> f32 {
    (((p.0 - c.0) / r.0).powi(2) + ((p.1 - c.1) / r.1).powi(2)).sqrt()
}

fn ellipse_pts(c: P, r: P, n: usize) -> Vec<P> {
    (0..n)
        .map(|i| {
            let a = i as f32 / n as f32 * std::f32::consts::TAU;
            (c.0 + r.0 * a.cos(), c.1 + r.1 * a.sin())
        })
        .collect()
}

/// Quadratic curve from `a` to `b` with control point `k`.
fn curve(a: P, k: P, b: P) -> Vec<P> {
    (0..=8)
        .map(|i| {
            let t = i as f32 / 8.0;
            let u = 1.0 - t;
            (
                u * u * a.0 + 2.0 * u * t * k.0 + t * t * b.0,
                u * u * a.1 + 2.0 * u * t * k.1 + t * t * b.1,
            )
        })
        .collect()
}

#[derive(Clone, Copy)]
enum Paint {
    Neon,
    Solid(Rgb),
}

struct Stroke {
    pts: Vec<P>,
    closed: bool,
    w: f32,
    /// Width at each point, for a line that is thin at both ends (a brush stroke).
    taper: Option<Vec<f32>>,
    aa: f32,
    paint: Paint,
    bbox: [f32; 4],
}

impl Stroke {
    fn new(pts: Vec<P>, closed: bool, w: f32, paint: Paint, aa: f32) -> Self {
        let m = 3.0 * w.max(aa);
        let mut bbox = [f32::MAX, f32::MAX, f32::MIN, f32::MIN];
        for p in &pts {
            bbox = [
                bbox[0].min(p.0 - m),
                bbox[1].min(p.1 - m),
                bbox[2].max(p.0 + m),
                bbox[3].max(p.1 + m),
            ];
        }
        Stroke {
            pts,
            closed,
            w: w.max(aa),
            taper: None,
            aa,
            paint,
            bbox,
        }
    }

    /// An open line with a width for each point, like a brush stroke.
    fn brush(pts: Vec<P>, widths: Vec<f32>, aa: f32) -> Self {
        let w = widths.iter().cloned().fold(0.0, f32::max);
        Stroke {
            taper: Some(widths),
            ..Stroke::new(pts, false, w, Paint::Neon, aa)
        }
    }

    fn add_to(&self, p: P, c: &mut Rgb) {
        let [x0, y0, x1, y1] = self.bbox;
        if p.0 < x0 || p.0 > x1 || p.1 < y0 || p.1 > y1 {
            return;
        }
        let n = self.pts.len();
        let segs = if self.closed { n } else { n - 1 };
        let k = (0..segs)
            .map(|i| {
                let (d, t) = seg_dist_t(p, self.pts[i], self.pts[(i + 1) % n]);
                match &self.taper {
                    None => (-(d / self.w).powi(2)).exp(),
                    // Thin parts are fainter, so the line fades at its ends.
                    Some(ws) => {
                        let w = lerp(ws[i], ws[i + 1], t);
                        (-(d / w.max(self.aa)).powi(2)).exp() * (w / self.aa).min(1.0).sqrt()
                    }
                }
            })
            .fold(0.0, f32::max);
        if k > 0.01 {
            add(
                c,
                match self.paint {
                    Paint::Neon => neon(p.0),
                    Paint::Solid(rgb) => rgb,
                },
                k,
            );
        }
    }
}

/// The geometry of one pose, ready to sample.
struct Scene<'a> {
    pose: &'a Pose,
    aa: f32,
    outline: Vec<P>,
    body: Vec<Stroke>,
    eyes: [P; 2],
    keyboard: Vec<Stroke>,
    key: Option<[f32; 4]>,
    mouse: Vec<Stroke>,
    front: Vec<Stroke>,
    legs: Vec<Vec<P>>,
}

impl<'a> Scene<'a> {
    fn new(pose: &'a Pose, s: f32) -> Self {
        let aa = s * 0.35;
        let h = pose.head;
        // Points of the head move with it; the neck blends into the body.
        let head_w = |y: f32| ((850.0 - y) / 60.0).clamp(0.0, 1.0);
        let mv = |p: P| (p.0 + h.0 * head_w(p.1), p.1 + h.1 * head_w(p.1));
        let mut outline: Vec<P> = OUTLINE.iter().map(|&p| mv(p)).collect();
        for (i, tip) in [0, 3].into_iter().enumerate() {
            outline[tip].0 += pose.ears[i].0;
            outline[tip].1 += pose.ears[i].1;
        }
        let hp = |p: P| (p.0 + h.0, p.1 + h.1);

        let mut body = vec![
            Stroke::new(outline.clone(), true, 7.0, Paint::Neon, aa),
            Stroke::new(TAIL.to_vec(), false, 7.0, Paint::Neon, aa),
        ];
        for tri in NOSE {
            body.push(Stroke::new(
                tri.iter().map(|&p| hp(p)).collect(),
                true,
                4.0,
                Paint::Solid(EYE),
                aa,
            ));
        }
        let eyes = EYES.map(hp);
        if pose.lid >= 0.95 {
            for e in eyes {
                let (rx, dy) = (EYE_R.0, if pose.happy { -18.0 } else { 16.0 });
                let base = if pose.happy { 8.0 } else { 0.0 };
                let pts = curve(
                    (e.0 - rx, e.1 + base),
                    (e.0, e.1 + dy),
                    (e.0 + rx, e.1 + base),
                );
                body.push(Stroke::new(pts, false, 7.0, Paint::Solid(CYAN), aa));
            }
        }
        if pose.mouth > 0.05 {
            let r = (13.0, 4.0 + 12.0 * pose.mouth);
            body.push(Stroke::new(
                ellipse_pts(hp(MOUTH), r, 12),
                true,
                5.0,
                Paint::Solid(PINK),
                aa,
            ));
        }

        let [kx0, ky0, kx1, ky1] = KEYBOARD;
        let mut keyboard = vec![Stroke::new(
            vec![(kx0, ky0), (kx1, ky0), (kx1, ky1), (kx0, ky1)],
            true,
            6.0,
            Paint::Neon,
            aa,
        )];
        // Keys must stay at least two cells wide, or the separators merge.
        let n = ((kx1 - kx0) / (3.0 * s).max(34.0)).floor().max(1.0);
        let kw = (kx1 - kx0) / n;
        for i in 1..n as usize {
            let x = kx0 + i as f32 * kw;
            keyboard.push(Stroke::new(
                vec![(x, ky0 + 22.0), (x, ky1 - 12.0)],
                false,
                4.0,
                Paint::Neon,
                aa,
            ));
        }
        let key = pose.key.map(|f| {
            let i = (f.clamp(0.0, 0.999) * n).floor();
            [
                kx0 + i * kw + 4.0,
                ky0 + 18.0,
                kx0 + (i + 1.0) * kw - 4.0,
                ky1 - 10.0,
            ]
        });

        let m = pose.mouse;
        let mouse = vec![
            Stroke::new(
                vec![
                    (m.0, m.1 - MOUSE_R.1),
                    (m.0 - 25.0, m.1 - 80.0),
                    (kx1 + 8.0, 1335.0),
                ],
                false,
                3.0,
                Paint::Solid(mix(VIOLET, [0.0; 3], 0.4)),
                aa,
            ),
            Stroke::new(ellipse_pts(m, MOUSE_R, 20), true, 6.0, Paint::Neon, aa),
            Stroke::new(
                vec![(m.0, m.1 - MOUSE_R.1), (m.0, m.1 - 8.0)],
                false,
                4.0,
                Paint::Neon,
                aa,
            ),
        ];

        // Each front leg has two side lines: wide at the shoulder, narrow at the
        // wrist, and thin at the top, so that the leg grows out of the chest.
        // A round paw joins the two lines. The inside of the leg is dark, so the
        // leg hides the lines behind it. When the wrist moves (a lifted paw, a
        // paw on the mouse), the bend moves half as far, so the leg curves.
        let mut front = Vec::new();
        let mut legs = Vec::new();
        for i in 0..2 {
            let (s, w, rest) = (SHOULDERS[i], pose.paws[i], PAW_REST[i]);
            let b = (
                LEG_BENDS[i].0 + (w.0 - rest.0) * 0.5,
                LEG_BENDS[i].1 + (w.1 - rest.1) * 0.5,
            );
            let mut sides = [Vec::new(), Vec::new()];
            let mut widths = Vec::new();
            for k in 0..=8 {
                let t = k as f32 / 8.0;
                let u = 1.0 - t;
                let c = (
                    u * u * s.0 + 2.0 * u * t * b.0 + t * t * w.0,
                    u * u * s.1 + 2.0 * u * t * b.1 + t * t * w.1,
                );
                let d = (
                    2.0 * u * (b.0 - s.0) + 2.0 * t * (w.0 - b.0),
                    2.0 * u * (b.1 - s.1) + 2.0 * t * (w.1 - b.1),
                );
                let len = d.0.hypot(d.1).max(1e-3);
                let hw = lerp(LEG_HW.0, LEG_HW.1, t);
                let n = (-d.1 / len * hw, d.0 / len * hw);
                sides[0].push((c.0 + n.0, c.1 + n.1));
                sides[1].push((c.0 - n.0, c.1 - n.1));
                widths.push(LEG_W * (t / 0.4).clamp(0.2, 1.0));
            }
            let (a, z) = (sides[0][8], sides[1][8]);
            let toe = (w.0, w.1 + PAW_H);
            let mut paw = curve(a, (a.0 + (a.0 - w.0) * 0.5, toe.1), toe);
            paw.extend(
                curve(toe, (z.0 + (z.0 - w.0) * 0.5, toe.1), z)
                    .into_iter()
                    .skip(1),
            );

            let mut shape = sides[0].clone();
            shape.extend(paw.iter().skip(1));
            shape.extend(sides[1].iter().rev().skip(1));
            legs.push(shape);
            for side in sides {
                front.push(Stroke::brush(side, widths.clone(), aa));
            }
            front.push(Stroke::new(paw, false, LEG_W, Paint::Neon, aa));
        }

        Scene {
            pose,
            aa,
            outline,
            body,
            eyes,
            keyboard,
            key,
            mouse,
            front,
            legs,
        }
    }

    fn sample(&self, p: P) -> Rgb {
        let mut c = [0.0; 3];
        self.ring(p, inside_poly(p, &self.outline), &mut c);
        self.body.iter().for_each(|s| s.add_to(p, &mut c));
        for [x0, y0, x1, y1] in BARS {
            if p.0 >= x0 && p.0 <= x1 && p.1 >= y0 && p.1 <= y1 {
                let t = (p.0 - x0) / (x1 - x0);
                add(
                    &mut c,
                    if t < 0.5 {
                        mix(CYAN, VIOLET, t * 2.0)
                    } else {
                        mix(VIOLET, PINK, t * 2.0 - 1.0)
                    },
                    0.85,
                );
            }
        }
        if self.pose.lid < 0.95 {
            self.eye_fill(p, &mut c);
        }

        // In front of the cat: keyboard, mouse, then the legs.
        let [kx0, ky0, kx1, ky1] = KEYBOARD;
        if p.0 > kx0 && p.0 < kx1 && p.1 > ky0 && p.1 < ky1 {
            c = [0.0; 3];
        }
        self.keyboard.iter().for_each(|s| s.add_to(p, &mut c));
        if let Some([x0, y0, x1, y1]) = self.key
            && p.0 >= x0
            && p.0 <= x1
            && p.1 >= y0
            && p.1 <= y1
        {
            add(&mut c, CYAN, 1.0);
        }
        if ellipse_q(p, self.pose.mouse, MOUSE_R) < 1.0 {
            c = [0.0; 3];
        }
        self.mouse.iter().for_each(|s| s.add_to(p, &mut c));
        if self.legs.iter().any(|leg| inside_poly(p, leg)) {
            c = [0.0; 3];
        }
        self.front.iter().for_each(|s| s.add_to(p, &mut c));
        c
    }

    /// The rune halo: two full circles with a light that runs around them.
    /// The circles pass in front of the body; the rune marks show only outside it.
    fn ring(&self, p: P, on_body: bool, c: &mut Rgb) {
        let (dx, dy) = (p.0 - RING_C.0, p.1 - RING_C.1);
        let d = (dx * dx + dy * dy).sqrt();
        if !(RINGS[0] - 40.0..RINGS[1] + 40.0).contains(&d) {
            return;
        }
        let a = dy.atan2(dx);
        let deg = a.to_degrees();
        let mut da = a - self.pose.t * 0.7;
        da = (da + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU) - std::f32::consts::PI;
        let shimmer = 0.6 + 1.2 * (-(da * da) / 0.12).exp();
        let color = neon(p.0);
        let w = 6.0f32.max(self.aa);
        for r in RINGS {
            add(c, color, (-((d - r) / w).powi(2)).exp() * shimmer);
        }
        // Rune marks: short radial strokes in the band between the rings.
        if !on_body && (d - 266.0).abs() < 20.0 {
            let off = ((deg + 360.0).rem_euclid(9.0) - 4.5).abs().to_radians() * d;
            add(c, color, (-(off / w).powi(2)).exp() * 0.5 * shimmer);
        }
    }

    fn eye_fill(&self, p: P, c: &mut Rgb) {
        let pose = self.pose;
        let (rx, ry) = EYE_R;
        for e in self.eyes {
            let q = ellipse_q(p, e, EYE_R);
            if q > 1.3 {
                continue;
            }
            // The upper lid closes from the top; its edge is a cyan line.
            let yl = e.1 - ry + 2.0 * ry * pose.lid;
            if pose.lid > 0.02 {
                let k = ((yl - e.1) / ry).clamp(-0.98, 0.98);
                let hw = rx * (1.0 - k * k).sqrt();
                let w = 6.0f32.max(self.aa);
                add(
                    c,
                    CYAN,
                    (-(seg_dist(p, (e.0 - hw, yl), (e.0 + hw, yl)) / w).powi(2)).exp(),
                );
            }
            if p.1 < yl {
                continue;
            }
            let fill = ((1.0 - q) * rx.min(ry) / self.aa + 0.5).clamp(0.0, 1.0);
            let pw = pose.pupil;
            let ox = ((pose.gaze.0 - FACE.0) / 260.0).clamp(-1.0, 1.0) * (rx - pw - 4.0);
            let oy = ((pose.gaze.1 - FACE.1) / 260.0).clamp(-1.0, 1.0) * (ry - 12.0);
            if ellipse_q(p, (e.0 + ox, e.1 + oy), (pw, 22.0)) < 1.0 {
                continue;
            }
            add(c, EYE, fill);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn drawn(cells: &[Cell]) -> usize {
        cells.iter().filter(|c| c.ch != ' ').count()
    }

    #[test]
    fn draws_the_cat_at_terminal_sizes() {
        for (cols, rows) in [(40, 22), (60, 38), (120, 60)] {
            let l = Layout::new(cols, rows);
            let cells = render(&l, &Pose::default());
            assert_eq!(cells.len(), cols as usize * rows as usize);
            assert!(
                drawn(&cells) > cells.len() / 10,
                "{cols}x{rows}: {} glyphs",
                drawn(&cells)
            );
        }
    }

    #[test]
    fn eyes_are_blue_and_close() {
        let l = Layout::new(60, 38);
        let (c, r) = l.to_cell((EYES[0].0 - 18.0, EYES[0].1));
        let at = |cells: &[Cell]| cells[r as usize * 60 + c as usize];
        let open = at(&render(&l, &Pose::default()));
        assert!(
            open.ch != ' ' && open.rgb[2] > open.rgb[0],
            "open eye cell: {open:?}"
        );
        let closed = render(
            &l,
            &Pose {
                lid: 1.0,
                ..Pose::default()
            },
        );
        assert!(drawn(&closed) < drawn(&render(&l, &Pose::default())));
    }

    /// Prints the scene, to compare it with the artwork by eye:
    /// `COLS=60 ROWS=36 COLOR=1 cargo test preview -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn preview() {
        let env = |k: &str, d: u16| {
            std::env::var(k)
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(d)
        };
        let (cols, rows) = (env("COLS", 60), env("ROWS", 36));
        let color = std::env::var_os("COLOR").is_some();
        for line in render(&Layout::new(cols, rows), &Pose::default()).chunks(cols as usize) {
            let text: String = line
                .iter()
                .map(|c| {
                    if color {
                        format!("\x1b[38;2;{};{};{}m{}", c.rgb[0], c.rgb[1], c.rgb[2], c.ch)
                    } else {
                        c.ch.to_string()
                    }
                })
                .collect();
            println!("{text}{}", if color { "\x1b[0m" } else { "" });
        }
    }

    #[test]
    fn layout_maps_cells_both_ways() {
        let l = Layout::new(80, 40);
        for (c, r) in [(0, 0), (40, 20), (79, 39)] {
            assert_eq!(l.to_cell(l.to_image(c, r)), (c as i32, r as i32));
        }
    }

    #[test]
    fn outline_is_a_closed_cat() {
        assert!(inside_poly(FACE, &OUTLINE));
        assert!(!inside_poly(
            RING_C,
            &[(0.0, 0.0), (10.0, 0.0), (10.0, 10.0)]
        ));
        assert!(!inside_poly(MOUSE_REST, &OUTLINE));
    }
}
