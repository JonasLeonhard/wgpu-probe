use wgpu_probe::run;

fn main() {
    pollster::block_on(run());
}
