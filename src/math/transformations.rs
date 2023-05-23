use super::matrix4::Matrix4;
use super::tuple::Tuple;

pub fn view_transform(from: Tuple, to: Tuple, up: Tuple) -> Matrix4 {
    let forward = (to - from).normalize();
    let left = forward.cross(up.normalize());
    let true_up = left.cross(forward);

    let orientation = Matrix4::from_rows([
        [left.x, left.y, left.z, 0.],
        [true_up.x, true_up.y, true_up.z, 0.],
        [-forward.x, -forward.y, -forward.z, 0.],
        [0., 0., 0., 1.],
    ]);

    orientation * Matrix4::translation(-from.x, -from.y, -from.z)
}
