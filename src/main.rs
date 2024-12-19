#![no_std]
#![no_main]

pub mod eadk;
pub mod graphics;
pub mod math;
pub mod scene;

use eadk::Color;
use graphics::Buffer;
use math::Vec3;
use scene::{Camera, Scene, Sphere};

#[used]
#[link_section = ".rodata.eadk_app_name"]
pub static EADK_APP_NAME: [u8; 9] = *b"Raymarch\0";

#[used]
#[link_section = ".rodata.eadk_api_level"]
pub static EADK_APP_API_LEVEL: u32 = 0;

#[used]
#[link_section = ".rodata.eadk_app_icon"]
pub static EADK_APP_ICON: [u8; 4250] = *include_bytes!("../target/icon.nwi");

#[no_mangle]
#[link_section = ".rodata._eadk_main"]
pub fn main() {
    let mut buff = Buffer::new();
    buff.clear(Color::from_rgb(255, 255, 255));
    buff.render();

    let camera = Camera {
        position: Vec3::new(0., 0., 0.),
        focal_length: 1.,
    };
    let sphere = Sphere {
        color: Color::from_rgb(255, 0, 0),
        position: Vec3::new(0., 0., 5.),
        radius: 2.,
    };
    let scene = Scene {
        camera,
        background_color: Color::from_rgb(0, 50, 0),
        sphere,
    };
    scene.render(50, &mut buff);
    buff.render();

    // hang when finished
    loop {}
}
