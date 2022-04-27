

pub fn main() {

  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();

  let window = video_subsystem.window("Tiny Raycaster", 1024, 512)
    .position_centered()
    .vulkan()
    .build()
    .unwrap();
}
