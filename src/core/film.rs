use crate::math::*;

pub struct Film {}

impl Film {
    pub fn get_sample_bounds(&self) -> Bounds2Di {
        Bounds2Di{ min: Vec2i::new(0, 0), max: Vec2i::new(0, 0) }
    }

    pub fn get_film_tile(&self, bounds: Bounds2Di) -> FilmTile {
        FilmTile{}
    }
}

pub struct FilmTile {}

impl Iterator for FilmTile {
    type Item = Point2i;

    fn next(&mut self) -> Option<Point2i> {
        None
    }
}