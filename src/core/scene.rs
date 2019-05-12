use crate::lights;

pub struct Scene {
    lights : Vec<Box<lights::Light>>
}