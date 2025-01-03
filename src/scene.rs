use core::f32;

use crate::{
    eadk::{
        display::{HALF_RATIO, SCREEN_HEIGHT, SCREEN_WIDTH},
        Color,
    },
    graphics::Buffer,
    math::Vec3,
};

trait SceneObject {
    fn distance_with(self, position: Vec3) -> f32;
    fn get_ray_color(self, ray_distance: f32) -> Color;
}

#[derive(Clone, Copy)]
pub struct Sphere {
    pub position: Vec3,
    pub radius: f32,
    pub color: Color,
}

impl SceneObject for Sphere {
    fn distance_with(self, position: Vec3) -> f32 {
        let delta = (self.position - position).magnitude();
        delta - self.radius
    }

    fn get_ray_color(self, _ray_distance: f32) -> Color {
        self.color
    }
}

pub struct Light {
    pub position: Vec3,
}

#[derive(Clone, Copy)]
pub struct Camera {
    pub position: Vec3,
    pub focal_length: f32,
}

impl Camera {
    fn get_sensor_position(self) -> Vec3 {
        let z = self.position.z - self.focal_length;
        Vec3 { z, ..self.position }
    }
}

pub struct Scene {
    pub spheres: [Sphere; 3],
    pub camera: Camera,
    pub background_color: Color,
    pub light: Light,
}

impl Scene {
    fn march_radius_at(&self, position: Vec3) -> (f32, Sphere) {
        let mut min_distance = f32::MAX;
        let mut closest_sphere = self.spheres[0];
        for sphere in self.spheres {
            let distance = sphere.distance_with(position);
            if distance < min_distance {
                min_distance = distance;
                closest_sphere = sphere;
            }
        }
        return (min_distance, closest_sphere);
    }

    fn march_ray_for(
        &self,
        x: f32,
        y: f32,
        max_iteration: u16,
        camera_sensor_position: Vec3,
    ) -> Color {
        let mut current_position = Vec3::new(x, y, self.camera.position.z);
        let start_position = current_position.clone();
        let mut ray_direction = current_position - camera_sensor_position;
        ray_direction.normalize();
        for _ in 0..max_iteration {
            let (march_radius, closest_sphere) = self.march_radius_at(current_position);
            if march_radius < 0.1 {
                let travel_distance = (current_position - start_position).magnitude();
                return closest_sphere.get_ray_color(travel_distance);
            }
            current_position += ray_direction * march_radius;
        }
        return self.background_color;
    }

    pub fn render(&self, max_iteration: u16, buff: &mut Buffer) {
        let camera_sensor_position = self.camera.get_sensor_position();
        for screen_x in 0..=SCREEN_WIDTH {
            let x = screen_x as f32 / 240. - HALF_RATIO;
            for screen_y in 0..=SCREEN_HEIGHT {
                let y = screen_y as f32 / 240. - 0.5;
                let color = self.march_ray_for(x, y, max_iteration, camera_sensor_position);
                buff.set_at(screen_x, screen_y, color);
            }
            buff.fast_render();
        }
    }
}
