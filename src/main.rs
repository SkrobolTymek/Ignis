use wgpu;
use winit::{
    event::{ElementState, Event, KeyboardInput, ModifiersState, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    input_text: String,
    cursor_position: usize,
    modifiers: ModifiersState,
}

impl State {
    async fn new(window: &Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let surface = unsafe { instance.create_surface(window) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to find an appropriate adapter");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let size = window.inner_size();
        let config = Self::create_surface_config(&adapter, &surface, size);
        surface.configure(&device, &config);

        Self {
            surface,
            device,
            queue,
            config,
            input_text: String::new(),
            cursor_position: 0,
            modifiers: ModifiersState::empty(),
        }
    }

    fn create_surface_config(
        adapter: &wgpu::Adapter,
        surface: &wgpu::Surface,
        size: winit::dpi::PhysicalSize<u32>,
    ) -> wgpu::SurfaceConfiguration {
        let surface_caps = surface.get_capabilities(adapter);
        wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_caps.formats[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn render(&self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let _rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }

        self.queue.submit(Some(encoder.finish()));
        output.present();

        Ok(())
    }

    fn handle_input(&mut self, input: KeyboardInput) {
        if let Some(keycode) = input.virtual_keycode {
            match input.state {
                ElementState::Pressed => match keycode {
                    VirtualKeyCode::Back => {
                        if self.cursor_position > 0 {
                            self.input_text.remove(self.cursor_position - 1);
                            self.cursor_position -= 1;
                        }
                    }
                    VirtualKeyCode::Delete => {
                        if self.cursor_position < self.input_text.len() {
                            self.input_text.remove(self.cursor_position);
                        }
                    }
                    VirtualKeyCode::Left => {
                        if self.cursor_position > 0 {
                            self.cursor_position -= 1;
                        }
                    }
                    VirtualKeyCode::Right => {
                        if self.cursor_position < self.input_text.len() {
                            self.cursor_position += 1;
                        }
                    }
                    VirtualKeyCode::Return => {
                        self.input_text.insert(self.cursor_position, '\n');
                        self.cursor_position += 1;
                    }
                    VirtualKeyCode::Home => {
                        self.cursor_position = 0;
                    }
                    VirtualKeyCode::End => {
                        self.cursor_position = self.input_text.len();
                    }
                    _ => {
                        if let Some(c) = Self::keycode_to_char(keycode, self.modifiers.shift()) {
                            self.input_text.insert(self.cursor_position, c);
                            self.cursor_position += 1;
                        }
                    }
                },
                _ => {}
            }
        }
    }

    fn keycode_to_char(keycode: VirtualKeyCode, shift: bool) -> Option<char> {
        match keycode {
            VirtualKeyCode::Key1 => Some(if shift { '!' } else { '1' }),
            VirtualKeyCode::Key2 => Some(if shift { '@' } else { '2' }),
            VirtualKeyCode::Key3 => Some(if shift { '#' } else { '3' }),
            VirtualKeyCode::Key4 => Some(if shift { '$' } else { '4' }),
            VirtualKeyCode::Key5 => Some(if shift { '%' } else { '5' }),
            VirtualKeyCode::Key6 => Some(if shift { '^' } else { '6' }),
            VirtualKeyCode::Key7 => Some(if shift { '&' } else { '7' }),
            VirtualKeyCode::Key8 => Some(if shift { '*' } else { '8' }),
            VirtualKeyCode::Key9 => Some(if shift { '(' } else { '9' }),
            VirtualKeyCode::Key0 => Some(if shift { ')' } else { '0' }),
            VirtualKeyCode::A => Some(if shift { 'A' } else { 'a' }),
            VirtualKeyCode::B => Some(if shift { 'B' } else { 'b' }),
            VirtualKeyCode::C => Some(if shift { 'C' } else { 'c' }),
            VirtualKeyCode::D => Some(if shift { 'D' } else { 'd' }),
            VirtualKeyCode::E => Some(if shift { 'E' } else { 'e' }),
            VirtualKeyCode::F => Some(if shift { 'F' } else { 'f' }),
            VirtualKeyCode::G => Some(if shift { 'G' } else { 'g' }),
            VirtualKeyCode::H => Some(if shift { 'H' } else { 'h' }),
            VirtualKeyCode::I => Some(if shift { 'I' } else { 'i' }),
            VirtualKeyCode::J => Some(if shift { 'J' } else { 'j' }),
            VirtualKeyCode::K => Some(if shift { 'K' } else { 'k' }),
            VirtualKeyCode::L => Some(if shift { 'L' } else { 'l' }),
            VirtualKeyCode::M => Some(if shift { 'M' } else { 'm' }),
            VirtualKeyCode::N => Some(if shift { 'N' } else { 'n' }),
            VirtualKeyCode::O => Some(if shift { 'O' } else { 'o' }),
            VirtualKeyCode::P => Some(if shift { 'P' } else { 'p' }),
            VirtualKeyCode::Q => Some(if shift { 'Q' } else { 'q' }),
            VirtualKeyCode::R => Some(if shift { 'R' } else { 'r' }),
            VirtualKeyCode::S => Some(if shift { 'S' } else { 's' }),
            VirtualKeyCode::T => Some(if shift { 'T' } else { 't' }),
            VirtualKeyCode::U => Some(if shift { 'U' } else { 'u' }),
            VirtualKeyCode::V => Some(if shift { 'V' } else { 'v' }),
            VirtualKeyCode::W => Some(if shift { 'W' } else { 'w' }),
            VirtualKeyCode::X => Some(if shift { 'X' } else { 'x' }),
            VirtualKeyCode::Y => Some(if shift { 'Y' } else { 'y' }),
            VirtualKeyCode::Z => Some(if shift { 'Z' } else { 'z' }),
            VirtualKeyCode::Space => Some(' '),
            VirtualKeyCode::Apostrophe => Some(if shift { '\"' } else { '\'' }),
            VirtualKeyCode::Comma => Some(if shift { '<' } else { ',' }),
            VirtualKeyCode::Period => Some(if shift { '>' } else { '.' }),
            VirtualKeyCode::Slash => Some(if shift { '?' } else { '/' }),
            VirtualKeyCode::Backslash => Some(if shift { '|' } else { '\\' }),
            VirtualKeyCode::Semicolon => Some(if shift { ':' } else { ';' }),
            VirtualKeyCode::LBracket => Some(if shift { '{' } else { '[' }),
            VirtualKeyCode::RBracket => Some(if shift { '}' } else { ']' }),
            VirtualKeyCode::Grave => Some(if shift { '~' } else { '`' }),
            VirtualKeyCode::Minus => Some(if shift { '_' } else { '-' }),
            VirtualKeyCode::Equals => Some(if shift { '+' } else { '=' }),
            _ => None,
        }
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("Ignis Terminal")
        .build(&event_loop)
        .unwrap();

    let mut state = pollster::block_on(State::new(&window));

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,

            Event::WindowEvent {
                event: WindowEvent::Resized(new_size),
                ..
            } => state.resize(new_size),

            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => state.handle_input(input),

            Event::WindowEvent {
                event: WindowEvent::ModifiersChanged(modifiers),
                ..
            } => {
                state.modifiers = modifiers;
            }

            Event::RedrawRequested(_) => {
                if let Err(e) = state.render() {
                    eprintln!("Render error: {:?}", e);
                    *control_flow = ControlFlow::Exit;
                }
            }

            Event::MainEventsCleared => window.request_redraw(),

            _ => {}
        }
    });
}
