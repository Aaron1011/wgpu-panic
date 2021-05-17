use futures::executor::block_on;

fn main() {
    let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
	let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
		power_preference: Default::default(),
		compatible_surface: None
	})).unwrap();

    panic!("Deliberate panic");
}
