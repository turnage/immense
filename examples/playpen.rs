use hex;
use immense::*;
use itertools::iproduct;
use lazy_static::lazy_static;
use noise::{Fbm, NoiseFn};
use palette::encoding::pixel::Pixel;
use palette::encoding::srgb::Srgb;
use palette::rgb::Rgb;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::fs::File;
use std::io::BufWriter;
use std::rc::Rc;

const SPHERE_RESOLUTION: usize = 0;

lazy_static! {
    static ref PALETTE: [Hsv; 5] = [
        hexcolor("4F4052"),
        hexcolor("6D7577"),
        hexcolor("95A8A9"),
        hexcolor("A8C4BE"),
        hexcolor("AFD8DB"),
    ];
}

fn hexcolor(hex: &str) -> Hsv {
    let bytes: Vec<u8> = hex::decode(hex).expect("raw bytes from hex");
    let fmt = |i| bytes[i] as f32 / 255.0;
    let color: Rgb<Srgb, f32> = *Rgb::from_raw(&[fmt(0), fmt(1), fmt(2)]);
    Hsv::from(color)
}

trait Tilable: Clone + 'static {
    fn to_tile(&self, row: usize, col: usize) -> Rule;
}

struct GridTile<T> {
    row: usize,
    col: usize,
    tilable: T,
}

impl<T: Tilable> ToRule for GridTile<T> {
    fn to_rule(&self) -> Rule {
        self.tilable.to_tile(self.row, self.col)
    }
}

fn grid(rows: usize, cols: usize, rule: impl Tilable) -> Rule {
    rule![
        tf![Tf::tx((cols - 1) as f32 / -2.0), Tf::tz((rows - 1) as f32 / -2.0)] =>
        iproduct!(0..rows, 0..cols).fold(Rule::new(), |root_rule, (r, c)| {
            root_rule.push(tf![
                Tf::tx(c as f32),
                Tf::tz(r as f32)
            ], GridTile {
                row: r,
                col: c,
                tilable: rule.clone()
            })
        })
    ]
}

#[derive(Clone)]
struct Pyramid {
    levels: usize,
    sphere: Rc<Mesh>,
}

impl ToRule for Pyramid {
    fn to_rule(&self) -> Rule {
        (0..self.levels).fold(Rule::new(), |rule, i| {
            let target_downscale = 1.0 - ((i + 1) as f32 / self.levels as f32);
            rule.push(
                tf![
                    Tf::sby(
                        target_downscale,
                        ((1.0 / self.levels as f32) * 1.6).powi(2),
                        target_downscale
                    ),
                    Tf::ty(i as f32),
                    Tf::color(*PALETTE.choose(&mut thread_rng()).unwrap())
                ],
                (*&[cube(), self.sphere.to_rule()]
                    .choose(&mut thread_rng())
                    .unwrap())
                    .clone(),
            )
        })
    }
}

#[derive(Clone)]
struct Tower;

impl ToRule for Tower {
    fn to_rule(&self) -> Rule {
        let thin = 0.002;
        let bars = thread_rng().gen_range(4, 20);
        let height = 0.03 * (1.0 / bars as f32);
        let bar = rule![
            tf![Tf::color(*PALETTE.choose(&mut thread_rng()).unwrap()), Tf::tx(-0.5), Tf::sby(thin, height, 1.0)] => cube(),
            tf![Tf::color(*PALETTE.choose(&mut thread_rng()).unwrap()), Tf::tx(0.5), Tf::sby(thin, height, 1.0)] => cube(),
            tf![Tf::color(*PALETTE.choose(&mut thread_rng()).unwrap()), Tf::tz(-0.5), Tf::sby(1.0, height, thin)] => cube(),
            tf![Tf::color(*PALETTE.choose(&mut thread_rng()).unwrap()), Tf::tz(0.5), Tf::sby(1.0, height, thin)] => cube(),
            tf![Tf::color(*PALETTE.choose(&mut thread_rng()).unwrap()), Tf::ty(0.0), Tf::s(0.2)] => icosphere(),
        ];
        rule![
            Replicate::n(bars, Tf::ty(height * 13.0)) => bar,
        ]
    }
}

#[derive(Clone)]
struct CityBlock {
    sphere: Rc<Mesh>,
    noise: Fbm,
    depth: usize,
}

impl Tilable for CityBlock {
    fn to_tile(&self, row: usize, col: usize) -> Rule {
        let division = (self.noise.get([row as f64, col as f64]).abs() * 10.0) as usize + 4;
        let mut candidates = vec![
            rule![None => Pyramid {
                levels: division,
                sphere: self.sphere.clone(),
            }],
            rule![None => Tower {}],
        ];
        if self.depth < 3 {
            candidates.push(rule![tf![Tf::sby(0.5, 0.5, 0.5)] => grid(
                2,
                2,
                CityBlock {
                    sphere: self.sphere.clone(),
                    noise: self.noise.clone(),
                    depth: self.depth + 1,
                },
            )]);
        }
        let mut rng = thread_rng();
        (&candidates).choose(&mut rng).unwrap().clone()
    }
}

#[derive(Copy, Clone)]
struct Wire;

impl Tilable for Wire {
    fn to_tile(&self, _: usize, _: usize) -> Rule {
        let height = 0.03;
        let thin = 0.05;
        rule![Tf::color(*PALETTE.choose(&mut thread_rng()).unwrap()) => rule![
            tf![Tf::tx(-0.5), Tf::sby(thin, height, 1.0)] => cube(),
            tf![Tf::tx(0.5), Tf::sby(thin, height, 1.0)] => cube(),
            tf![Tf::tz(-0.5), Tf::sby(1.0, height, thin)] => cube(),
            tf![Tf::tz(0.5), Tf::sby(1.0, height, thin)] => cube()
        ]]
    }
}

fn main() {
    let grid_size = 10;
    let meshes = rule![
        tf![
            Tf::sby(grid_size as f32, 1.0, grid_size as f32),
            Tf::ty(-0.5),
            Tf::color(Hsv::new(0.0, 0.0, 1.0))
        ] => cube(),
        None => grid(grid_size, grid_size, CityBlock { sphere: sphere(SPHERE_RESOLUTION), noise: Fbm::new(), depth: 0 }),
        None => grid(grid_size, grid_size, Wire {})
    ]
    .generate();
    let output = File::create("pyramid.obj").expect("obj file");
    write_meshes(
        ExportConfig {
            grouping: MeshGrouping::ByColor,
            export_colors: Some(String::from("pyramid_colors.mtl")),
        },
        meshes,
        &mut BufWriter::new(output),
    )
    .expect("rendered scene");
}
