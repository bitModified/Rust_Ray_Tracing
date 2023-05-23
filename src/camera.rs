use std::io::Write;

use crate::{canvas::Canvas, math::matrix4::Matrix4, math::tuple::Tuple, ray::Ray, world::World};

#[derive(Clone, Copy)]
pub struct Camera {
    pub hsize: i32,
    pub vsize: i32,
    pub field_of_view: f64,
    pub transform: Matrix4,
}

impl Camera {
    pub fn new(hsize: i32, vsize: i32, field_of_view: f64) -> Self {
        Self {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix4::identity(),
        }
    }

    fn half_extents(self) -> (f64, f64) {
        let half_view = (self.field_of_view / 2.).tan();
        let aspect = self.hsize as f64 / self.vsize as f64;

        let (half_width, half_height) = if aspect > 1. {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        (half_width, half_height)
    }

    fn pixel_size(self) -> f64 {
        let (half_width, _) = self.half_extents();

        2. * half_width / self.hsize as f64
    }

    pub fn ray_for_pixel(self, px: i32, py: i32) -> Ray {
        let x_offset = (px as f64 + 0.5) * self.pixel_size();
        let y_offset = (py as f64 + 0.5) * self.pixel_size();

        let (half_width, half_height) = self.half_extents();
        let world_x = half_width - x_offset;
        let world_y = half_height - y_offset;

        let inverse_transform = self.transform.inverse().unwrap();
        let pixel = inverse_transform * Tuple::point(world_x, world_y, -1.);
        let origin = inverse_transform * Tuple::point(0., 0., 0.);

        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn render(self, world: &World) -> Canvas {
        let mut canvas = Canvas::new(self.hsize as usize, self.vsize as usize);
        let total_pixels = self.vsize * self.hsize;

        let mut total_done = 0;
        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(ray);

                canvas.write_pixel(x, y, color);
            }
            total_done += self.hsize;
            print!(
                "Computed: {}/{} ({}%) pixels.\r",
                total_done,
                total_pixels,
                (100. * (total_done as f64 / total_pixels as f64)).round()
            );
            std::io::stdout().flush().unwrap();
        }

        canvas
    }
}