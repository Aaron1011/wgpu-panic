use futures::executor::block_on;

fn main() {
    let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
	let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
		power_preference: Default::default(),
		compatible_surface: None
	})).unwrap();

	block_on(adapter.request_device(
		&wgpu::DeviceDescriptor {
			label: None,
			features: wgpu::Features::empty(),
			limits: wgpu::Limits::default(),
		},
        None
	)).unwrap();

    panic!("Panicking");
}
