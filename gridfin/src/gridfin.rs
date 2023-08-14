// ref https://gridfinity.xyz/specification/

use glam::{dvec3, DVec3};
use opencascade::{
    primitives::{Direction, Shape, Solid},
    workplane::Workplane,
};

const SIZE: f64 = 41.5;
const FILLET: f64 = 3.75;
const INNER_FILLET: f64 = 3.20;
const MID_LIFT: f64 = 4.75;
const V_UNIT: f64 = 7.0;
const WALL_THICKNESS: f64 = 2.15;

// this is the wall construction
// currently it just a filleted empty subtraction
// it should parse a shape in to subtract out so
// a series of different interiors can be buit.
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
        let mut wall: Shape = wall_outline
            .to_face()
            .extrude(dvec3(0.0, 0.0, V_UNIT * self.height as f64))
            .into();
        if !self.filled {
            let mut cutout_outline =
                Workplane::xy().rect(width - 2.0 * WALL_THICKNESS, depth - 2.0 * WALL_THICKNESS);
            cutout_outline.fillet(INNER_FILLET);
            cutout_outline.translate(dvec3(0.0, 0.0, V_UNIT));
            let mut cutout: Shape = cutout_outline
                .to_face()
                .extrude(dvec3(0.0, 0.0, V_UNIT * self.height as f64))
                .into();

            let bot_edges = cutout.faces().farthest(Direction::NegZ).edges();
            cutout.fillet_edges(0.8, bot_edges);
            wall = wall.subtract(&cutout).into();
        }
        wall
    }
}

#[derive(Debug)]
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
        let mut plate: Shape = plate_outline
            .to_face()
            .extrude(dvec3(0.0, 0.0, V_UNIT - MID_LIFT))
            .into();
        for x in 0..self.x {
            for y in 0..self.y {
                println!("{:?},{:?}", x, y);
                let mut base = Connector::connector();
                // origin is the center of the plate
                let x_pos = (SIZE * x as f64) - (SIZE * (self.x - 1) as f64) / 2.0;
                let y_pos = (SIZE * y as f64) - (SIZE * (self.y - 1) as f64) / 2.0;
                base.set_global_translation(dvec3(x_pos, y_pos, 0.0));
                plate = plate.union(&base).into();
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
    chamfer: bool,
}

pub struct Connector {
    x: usize,
    y: usize,
    config: BaseConfig,
}

impl Connector {
    const UNDER: BaseConfig = BaseConfig {
        lower_size: 37.2,
        lower_fillet: 1.6,
        mid_fillet: 2.6,
        magnets: true,
        chamfer: true,
    };

    const LIP: BaseConfig = BaseConfig {
        lower_size: 37.2,
        lower_fillet: 1.6,
        mid_fillet: 2.6,
        magnets: false,
        chamfer: false,
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
        let inset: f64 = SIZE - self.config.lower_size;
        let mut outline = Workplane::xy().rect(
            (SIZE * self.x as f64) - inset,
            (SIZE * self.y as f64) - inset,
        );
        outline.fillet(Connector::MID_FILLET);
        let mut lower: Shape = outline
            .to_face()
            .extrude(dvec3(0.0, 0.0, Connector::LOWER_HEIGHT))
            .into();
        // chamfer
        if self.config.chamfer {
            let bot_edges = lower.faces().farthest(Direction::NegZ).edges();
            lower.chamfer_edges(0.8, bot_edges);
        }
        // cut the magnets out
        if self.config.magnets {
            let mag_pos = Connector::LOWER_SIZE / 2.0 - Connector::MAG_INSET;
            let mags = [
                Magnet::new(dvec3(mag_pos, mag_pos, 0.0)),
                Magnet::new(dvec3(-mag_pos, mag_pos, 0.0)),
                Magnet::new(dvec3(mag_pos, -mag_pos, 0.0)),
                Magnet::new(dvec3(-mag_pos, -mag_pos, 0.0)),
            ];
            for mut m in mags {
                // TODO will change to new bool
                lower = lower.subtract(&m.shape()).into();
            }
        }
        // middle
        let mut mid_lower =
            Workplane::xy().rect(SIZE * self.x as f64 - inset, SIZE * self.y as f64 - inset);
        mid_lower.fillet(Connector::MID_FILLET);
        mid_lower.translate(dvec3(0.0, 0.0, Connector::LOWER_HEIGHT));
        let mut mid_upper = Workplane::xy().rect(SIZE * self.x as f64, SIZE * self.y as f64);
        mid_upper.fillet(FILLET);
        mid_upper.translate(dvec3(0.0, 0.0, MID_LIFT));
        let mid = Solid::loft([&mid_lower, &mid_upper]).into();

        lower = lower.union(&mid).into();
        lower
    }

    pub fn connector() -> Shape {
        // just git back the under plate
        let mut s = Connector::new(1, 1, Self::UNDER);
        s.shape()
    }

    pub fn lip(x: usize, y: usize, height: usize) -> Shape {
        let mut s = Connector::new(x, y, Self::LIP).shape();
        let mut plate_outline = Workplane::xy().rect(SIZE * x as f64, SIZE * y as f64);
        plate_outline.fillet(FILLET);
        let mut plate: Shape = plate_outline
            .to_face()
            .extrude(dvec3(0.0, 0.0, MID_LIFT))
            .into();
        s = plate.subtract(&s).into();
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
        let mag = rim.to_face().extrude(dvec3(0.0, 0.0, self.thickness));
        mag.into()
    }
}

pub fn full(x: usize, y: usize, height: usize) -> Shape {
    let mut pl = Plate::new(x, y).shape();
    if height > 0 {
        let mut wall = Wall::new(x, y, height, false);
        pl = pl.union(&wall.shape()).into();
        let lip = Connector::lip(x, y, height);
        pl = pl.union(&lip).into();
    }
    pl
}

// The base plate for the bottom to mount the gf modules in
#[derive(Debug)]
pub struct BasePlate {
    x: usize,
    y: usize,
}

impl BasePlate {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x: x, y: y }
    }

    pub fn shape(&mut self) -> Shape {
        let mut plate_outline = Workplane::xy().rect(SIZE * self.x as f64, SIZE * self.y as f64);
        plate_outline.fillet(FILLET);
        let mut plate: Shape = plate_outline
            .to_face()
            .extrude(dvec3(0.0, 0.0, MID_LIFT))
            .into();
        for x in 0..self.x {
            for y in 0..self.y {
                println!("{:?},{:?}", x, y);
                let mut base = Connector::new(1, 1, Connector::LIP).shape();
                // origin is the center of the plate
                let x_pos = (SIZE * x as f64) - (SIZE * (self.x - 1) as f64) / 2.0;
                let y_pos = (SIZE * y as f64) - (SIZE * (self.y - 1) as f64) / 2.0;
                base.set_global_translation(dvec3(x_pos, y_pos, 0.0));
                plate = plate.subtract(&base).into();
            }
        }
        plate
    }
}
