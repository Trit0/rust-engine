use crate::core::component::Component;
use crate::math::vector_3f::Vector3F;

pub struct Transform {
    position: Vector3F,
    rotation: Vector3F,
    scale: Vector3F,
}

impl Transform {
    pub fn empty() -> Transform {
        return Transform {
            position: Vector3F::zero(),
            rotation: Vector3F::zero(),
            scale: Vector3F::zero()
        };
    }
}

impl Component for Transform {}