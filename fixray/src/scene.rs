use core::ops::Add;
use core::cmp::min;

pub use fp::FP;
pub use v::V3;
use {Color, Material};

/// Signed distance field object.
pub trait Body {
    /// Signed distance function for the surface of the body.
    fn distance(&self, pos: &V3) -> FP;

    /// Material of the body at position.
    fn material(&self, pos: &V3) -> Material {
        let _ = pos;
        Material::Surface(Color::Yellow, Color::Green, Color::Blue)
    }

    /// The normal is computed as gradient of the distance field.
    fn normal(&self, pos: &V3) -> V3 {
        let _ = pos;
        pos.grad(|p| self.distance(&p))
    }
}

/// A combination of several bodies.
///
/// A `Union` can be built by starting from a `Scene` and adding `Body` values to it.
pub struct Union<T, U> {
    first: T,
    second: U,
}

impl<T: Body, U: Body> Body for Union<T, U> {
    fn distance(&self, pos: &V3) -> FP {
        min(self.first.distance(pos), self.second.distance(pos))
    }

    fn material(&self, pos: &V3) -> Material {
        if self.first.distance(pos) < self.second.distance(pos) {
            self.first.material(pos)
        } else {
            self.second.material(pos)
        }
    }

    fn normal(&self, pos: &V3) -> V3 {
        // Overriding this so that it'll fall into the cheaper component method before starting the
        // expensive gradient operation.
        if self.first.distance(pos) < self.second.distance(pos) {
            self.first.normal(pos)
        } else {
            self.second.normal(pos)
        }
    }
}

impl<T: Body, U: Body, V: Body> Add<V> for Union<T, U> {
    type Output = Union<Union<T, U>, V>;

    fn add(self, other: V) -> Self::Output {
        Union {
            first: self,
            second: other,
        }
    }
}

/// Starter object for a scene.
pub struct Scene;

impl Body for Scene {
    fn distance(&self, pos: &V3) -> FP {
        let _ = pos;
        FP(i32::max_value())
    }

    fn material(&self, pos: &V3) -> Material {
        let _ = pos;
        Material::Surface(Color::Cyan, Color::Cyan, Color::Cyan)
    }
}

impl<T: Body> Add<T> for Scene {
    type Output = Union<Scene, T>;

    fn add(self, other: T) -> Self::Output {
        Union {
            first: self,
            second: other,
        }
    }
}

pub struct Object<F, G> {
    distance_f: F,
    material_f: G,
}

impl<F, G> Object<F, G>
    where F: Fn(&V3) -> FP,
          G: Fn(&V3) -> Material
{
    pub fn new(distance_f: F, material_f: G) -> Object<F, G> {
        Object {
            distance_f: distance_f,
            material_f: material_f,
        }
    }
}

impl<F, G> Body for Object<F, G>
    where F: Fn(&V3) -> FP,
          G: Fn(&V3) -> Material
{
    fn distance(&self, pos: &V3) -> FP {
        (self.distance_f)(pos)
    }

    fn material(&self, pos: &V3) -> Material {
        (self.material_f)(pos)
    }
}

pub fn sphere_fn(center: V3, radius: FP) -> impl Fn(&V3) -> FP {
    move |&p| (p - center).abs() - radius
}

pub fn plane_fn(normal: V3, offset: FP) -> impl Fn(&V3) -> FP {
    move |&p| p.dot(&normal) - offset
}
