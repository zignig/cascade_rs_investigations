// ref https://gridfinity.xyz/specification/

use glam::{dvec3, DVec3};
use opencascade::{
    angle::Angle,
    primitives::{Direction, Shape, Solid},
    workplane::Workplane,
};

const SIZE: f64 = 41.5;
const FILLET: f64 = 3.75;
const INNER_FILLET: f64 = 3.20;
const MID_LIFT: f64 = 4.75;
const V_UNIT: f64 = 7.0;
const WALL_THICKNESS: f64 = 2.15;

pub struct Wall {
    x: usize,
    y: usize,
    height: usize,
    filled: bool,
}

impl Wall {
    fn new(x: usize, y: usize, height: usize, filled: bool) -> Self {
        Self {
            x,
            y,
            height,
            filled,
        }
    }

    fn shape(&mut self) -> Shape {
        let width: f64 = SIZE * self.x as f64;
        let depth: f64 = SIZE * self.y as f64;
        let mut wall_outline = Workplane::xy().rect(width, depth);
        wall_outline.fillet(FILLET);
        wall_outline.translate(dvec3(0.0, 0.0, V_UNIT));
        let mut wall = wall_outline
            .to_face()
            .extrude(dvec3(0.0, 0.0, V_UNIT * self.height as f64))
            .to_shape();
        if !self.filled {
            let mut cutout_outline =
                Workplane::xy().rect(width - 2.0 * WALL_THICKNESS, depth - 2.0 * WALL_THICKNESS);
            cutout_outline.fillet(INNER_FILLET);
            cutout_outline.translate(dvec3(0.0, 0.0, V_UNIT));
            let mut cutout = cutout_outline
                .to_face()
                .extrude(dvec3(0.0, 0.0, V_UNIT * self.height as f64))
                .to_shape();

            let bot_edges = cutout.faces().farthest(Direction::NegZ).edges();
            cutout.fillet_edges(0.8, bot_edges);
            (wall, _) = wall.subtract_shape(&cutout);
        }
        wall
    }
}

// unfinished
pub struct Lip {
    x: usize,
    y: usize,
}

impl Lip {
    const HEIGHT: f64 = 4.4;
    const WIDTH: f64 = 2.6;
    const TOP_STEP: f64 = 1.9;
    const VIRT_STEP: f64 = 1.8;

    fn new(x: usize, y: usize) -> Self {
        Self { x: x, y: y }
    }

    fn shape(&mut self) -> Shape {
        let profile = Workplane::xy().sketch();
        let mut f = profile
            .line_to(Lip::WIDTH, 0.0)
            .line_to(Lip::WIDTH, Lip::HEIGHT)
            .line_to(Lip::WIDTH - Lip::TOP_STEP, Lip::HEIGHT - Lip::TOP_STEP)
            .line_to(
                Lip::WIDTH - Lip::TOP_STEP,
                Lip::HEIGHT - Lip::TOP_STEP - Lip::VIRT_STEP,
            )
            .line_to(0.0, 0.0)
            .wire();
        let face = f.to_face();
        let mut section = face
            .extrude(dvec3(0.0, 0.0, SIZE * self.x as f64))
            .to_shape();
        let spin = face
            .revolve(
                dvec3(0.0, 0.0, 0.0),
                dvec3(0.0, 1.0, 0.0),
                Some(Angle::Degrees(90.0)),
            )
            .to_shape();
        (section, _) = section.union_shape(&spin);
        section
    }
}
pub struct Plate {
    x: usize,
    y: usize,
}

impl Plate {
    fn new(x: usize, y: usize) -> Self {
        Self { x: x, y: y }
    }

    fn shape(&mut self) -> Shape {
        let mut plate_outline = Workplane::xy().rect(SIZE * self.x as f64, SIZE * self.y as f64);
        plate_outline.fillet(FILLET);
        plate_outline.translate(dvec3(0.0, 0.0, MID_LIFT));
        let mut plate = plate_outline
            .to_face()
            .extrude(dvec3(0.0, 0.0, V_UNIT - MID_LIFT))
            .to_shape();
        for x in 0..self.x {
            for y in 0..self.y {
                println!("{:?},{:?}", x, y);
                let mut base = Base::connector();
                // origin is the center of the plate
                let x_pos = (SIZE * x as f64) - (SIZE * (self.x - 1) as f64) / 2.0;
                let y_pos = (SIZE * y as f64) - (SIZE * (self.y - 1) as f64) / 2.0;
                base.set_global_translation(dvec3(x_pos, y_pos, 0.0));
                (plate, _) = plate.union_shape(&base);
            }
        }
        plate
    }
}

// as there are three versions of the bottom of the gridfinity system
// base plate , block bottom and top lip a config should be used to seperate them
pub struct BaseConfig {
    lower_size: f64,
    lower_fillet: f64,
    mid_fillet: f64,
    magnets: bool,
}

pub struct Base {
    x: usize,
    y: usize,
    config: BaseConfig,
}

impl Base {
    const UNDER: BaseConfig = BaseConfig {
        lower_size: 37.2,
        lower_fillet: 1.6,
        mid_fillet: 2.6,
        magnets: true,
    };

    const LIP: BaseConfig = BaseConfig {
        lower_size: 37.2,
        lower_fillet: 1.6,
        mid_fillet: 2.6,
        magnets: false,
    };

    const LOWER_SIZE: f64 = 37.2;
    const LOWER_FILLET: f64 = 1.6;
    const LOWER_HEIGHT: f64 = 2.6;

    const MID_FILLET: f64 = 3.2;

    const HEIGHT: f64 = 7.0;
    const MAG_INSET: f64 = 5.6;

    fn new(x: usize, y: usize, config: BaseConfig) -> Self {
        Self { x, y, config }
    }

    fn shape(&mut self) -> Shape {
        // lower section
        let mut outline = Workplane::xy().rect(
            self.config.lower_size * self.x as f64,
            self.config.lower_size * self.y as f64,
        );
        outline.fillet(Base::MID_FILLET);
        let mut lower = outline
            .to_face()
            .extrude(dvec3(0.0, 0.0, Base::LOWER_HEIGHT))
            .to_shape();
        // chamfer

        let bot_edges = lower.faces().farthest(Direction::NegZ).edges();
        lower.chamfer_edges(0.8, bot_edges);
        // cut the magnets out
        if self.config.magnets {
            let mag_pos = Base::LOWER_SIZE / 2.0 - Base::MAG_INSET;
            let mags = [
                Magnet::new(dvec3(mag_pos, mag_pos, 0.0)),
                Magnet::new(dvec3(-mag_pos, mag_pos, 0.0)),
                Magnet::new(dvec3(mag_pos, -mag_pos, 0.0)),
                Magnet::new(dvec3(-mag_pos, -mag_pos, 0.0)),
            ];
            for mut m in mags {
                // TODO will change to new bool
                (lower, _) = lower.subtract_shape(&m.shape());
            }
        }
        // middle
        let mut mid_lower = Workplane::xy().rect(
            self.config.lower_size * self.x as f64,
            self.config.lower_size * self.y as f64,
        );
        mid_lower.fillet(Base::MID_FILLET);
        mid_lower.translate(dvec3(0.0, 0.0, Base::LOWER_HEIGHT));
        let mut mid_upper = Workplane::xy().rect(SIZE * self.x as f64, SIZE * self.y as f64);
        mid_upper.fillet(FILLET);
        mid_upper.translate(dvec3(0.0, 0.0, MID_LIFT));
        let mut mid = Solid::loft([&mid_lower, &mid_upper]).to_shape();

        (lower, _) = lower.union_shape(&mid);
        lower
    }

    pub fn connector() -> Shape {
        // just git back the under plate
        let mut s = Base::new(1, 1, Self::UNDER);
        s.shape()
    }

    pub fn lip(x: usize, y: usize, height: usize) -> Shape {
        let mut s = Base::new(x, y, Self::LIP).shape();
        let mut plate_outline = Workplane::xy().rect(SIZE * x as f64, SIZE * y as f64);
        plate_outline.fillet(FILLET);
        let mut plate = plate_outline
            .to_face()
            .extrude(dvec3(0.0, 0.0, MID_LIFT))
            .to_shape();
        (s, _) = plate.subtract_shape(&s);
        s.set_global_translation(dvec3(0.0, 0.0, V_UNIT * (height + 1) as f64));
        s
    }
}

#[derive(Debug)]
pub struct Magnet {
    diameter: f64,
    thickness: f64,
    pos: DVec3,
}

impl Magnet {
    fn new(pos: DVec3) -> Self {
        Self {
            diameter: 6.5,
            thickness: 2.0,
            pos: pos,
        }
    }
    fn shape(&mut self) -> Shape {
        let mut rim = Workplane::xy().circle(0.0, 0.0, self.diameter / 2.0);
        rim.translate(self.pos);
        let mut mag = rim.to_face().extrude(dvec3(0.0, 0.0, self.thickness));
        mag.to_shape()
    }
}

pub fn full(x: usize, y: usize, height: usize) -> Shape {
    let mut pl = Plate::new(x, y).shape();
    if height > 0 {
        let mut wall = Wall::new(x, y, height, false);
        (pl, _) = pl.union_shape(&wall.shape());
    }
    //let lip = Base::lip(x,y,height);
    //( pl , _ ) = pl.union_shape(&lip);
    pl
}
