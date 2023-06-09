use std::{fs::File, io::Write};

use ray_tracer::{camera::Camera, world::World};

pub fn output_file_path(example_name: &str) -> String {
    format!("./output/{}.ppm", example_name)
}

pub fn run_and_save_scene(example_name: &str, camera: Camera, world: World) {
    let file_name = output_file_path(example_name);
    println!("Writing to: {}", file_name);

    let ppm = camera.render(&world).to_ppm();

    let mut f = File::create(&file_name).expect("Error creating file");
    f.write_all(ppm.as_bytes()).expect("Error writing data");
}
