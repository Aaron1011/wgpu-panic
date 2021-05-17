use futures::executor::block_on;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Transforms {
    world_matrix: [[f32; 4]; 4],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct TextureTransforms {
    u_matrix: [[f32; 4]; 4],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct ColorAdjustments {
    mult_color: [f32; 4],
    add_color: [f32; 4],
}

fn main() {
    let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
	let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
		power_preference: Default::default(),
		compatible_surface: None
	})).unwrap();

	let (device, queue) = block_on(adapter.request_device(
		&wgpu::DeviceDescriptor {
			label: None,
			features: wgpu::Features::PUSH_CONSTANTS,
			limits: wgpu::Limits {
				max_push_constant_size: (std::mem::size_of::<Transforms>()
					+ std::mem::size_of::<ColorAdjustments>())
					as u32,
				..Default::default()
			},
		},
        None
	)).unwrap();

    println!("Hello, world!");
    panic!("Panicking");
}
