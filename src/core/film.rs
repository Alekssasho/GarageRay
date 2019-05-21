use crate::math::*;
use crate::spectrum::Spectrum;

pub struct Film {}

impl Film {
    pub fn get_sample_bounds(&self) -> Bounds2Di {
        Bounds2Di {
            min: Vec2i::new(0, 0),
            max: Vec2i::new(0, 0),
        }
    }

    pub fn get_film_tile(&self, bounds: Bounds2Di) -> FilmTile {
        FilmTile {}
    }

    pub fn merge_film_tile(&self, tile: FilmTile) {}

    pub fn write_image(&self) {}
}

pub struct FilmTile {}

impl FilmTile {
    pub fn add_sample(&self, film: &Film, color: Spectrum, ray_weight: f32) -> () {}

    pub fn iter(&self) -> FilmTileIterator {
        FilmTileIterator
    }
}

pub struct FilmTileIterator;

impl Iterator for FilmTileIterator {
    type Item = Point2i;

    fn next(&mut self) -> Option<Point2i> {
        None
    }
}
