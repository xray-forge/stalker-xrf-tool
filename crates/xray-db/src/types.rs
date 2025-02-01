use crate::data::generic::vector_3d::Vector3d;

pub type Sphere3d<T = f32> = (Vector3d<T>, T);

pub type Matrix3d<T = f32> = (Vector3d<T>, Vector3d<T>, Vector3d<T>, Vector3d<T>);
