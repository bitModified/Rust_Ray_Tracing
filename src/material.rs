use crate::color::Color;
use crate::light::Light;
use crate::math::tuple::Tuple;
use crate::misc::approx_equal;
use crate::pattern::Pattern;
use crate::shape::SimpleObject;

#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub reflective: f64,
    pattern: Option<Pattern>,
    pub transparency: f64,
    pub refractive_index: f64,
    pub casts_shadows: bool,
}

impl Material {
    pub fn new() -> Self {
        Self {
            color: Color::white(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.,
            reflective: 0.,
            pattern: None,
            transparency: 0.,
            refractive_index: 1.,
            casts_shadows: true,
        }
    }

    pub fn with_pattern(pattern: Pattern) -> Self {
        Self {
            pattern: Some(pattern),
            ..Self::new()
        }
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
            && approx_equal(self.ambient, other.ambient)
            && approx_equal(self.diffuse, other.diffuse)
            && approx_equal(self.specular, other.specular)
            && approx_equal(self.shininess, other.shininess)
    }
}

pub fn lighting(
    material: Material,
    object: SimpleObject,
    light: Light,
    point: Tuple,
    eye_vector: Tuple,
    normal_vector: Tuple,
    in_shadow: bool,
) -> Color {
    let color = if let Some(pattern) = material.pattern {
        pattern.pattern_at_object(object, point)
    } else {
        material.color
    };

    // Combine the surface color with light color and intensity
    let effective_color = color * light.intensity;
    // find the direction to the light source
    let light_vector = (light.position - point).normalize();
    // compute ambient contribution
    let ambient = effective_color * material.ambient;

    // light_dot_normal represents the cosine of the angle between the
    // light vector and the normal vector. A negative number means the
    // light is on the other side of the surface.
    let light_dot_normal = light_vector.dot(normal_vector);

    let (diffuse, specular) = if light_dot_normal < 0. {
        (Color::black(), Color::black())
    } else {
        // compute diffuse contribution
        let diffuse = effective_color * material.diffuse * light_dot_normal;

        let reflect_vector = (-light_vector).reflect(normal_vector);
        let reflect_dot_eye = reflect_vector.dot(eye_vector);
        if reflect_dot_eye <= 0. {
            let specular = Color::black();
            (diffuse, specular)
        } else {
            // compute specular contribution
            let factor = reflect_dot_eye.powf(material.shininess);
            let specular = light.intensity * material.specular * factor;
            (diffuse, specular)
        }
    };

    if in_shadow {
        ambient
    } else {
        ambient + diffuse + specular
    }
}