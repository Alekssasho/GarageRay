use crate::core::Film;

pub trait Camera {
    fn film(&self) -> Film;
}