use std::collections::VecDeque;
use wgpu;
use winit::{
    event::{ElementState, Event, KeyboardInput, ModifiersState, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

// 🧱 Basic Architecture
pub struct TerminalApp {
    pub state: render::RenderState,
    pub terminal: terminal::Terminal,
    pub event_loop: EventLoop<()>,
}

impl TerminalApp {
    pub async fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("Ignis Terminal")
            .build(&event_loop)
            .unwrap();

        let state = render::RenderState::new(window).await;
        let terminal = terminal::Terminal::new(80, 24);

        Self {
            state,
            terminal,
            event_loop,
        }
    }

    pub fn run(self) {
        let mut terminal = self.terminal;
        let mut state = self.state;

        self.event_loop.run(move |event, _, control_flow| {
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
                } => {
                    terminal.handle_input(input);
                    state.request_redraw();
                }

                Event::WindowEvent {
                    event: WindowEvent::ModifiersChanged(modifiers),
                    ..
                } => terminal.modifiers = modifiers,

                Event::RedrawRequested(_) => {
                    if let Err(e) = state.render(&terminal) {
                        eprintln!("Render error: {:?}", e);
                        *control_flow = ControlFlow::Exit;
                    }
                }

                Event::MainEventsCleared => {
                    state.request_redraw();
                }

                _ => {}
            }
        });
    }
}

// 🧮 Terminal Emulation (VT)
mod terminal {
    use super::*;

    #[derive(Debug, Clone, Copy)]
    pub struct Cell {
        pub char: char,
        pub fg_color: [f32; 3],
        pub bg_color: [f32; 3],
    }

    pub struct Terminal {
        pub width: usize,
        pub height: usize,
        pub cursor_x: usize,
        pub cursor_y: usize,
        pub cells: Vec<Vec<Cell>>,
        pub scrollback: VecDeque<Vec<Cell>>,
        pub modifiers: ModifiersState,
    }

    impl Terminal {
        pub fn new(width: usize, height: usize) -> Self {
            let default_cell = Cell {
                char: ' ',
                fg_color: [1.0, 1.0, 1.0],
                bg_color: [0.0, 0.0, 0.0],
            };

            let cells = vec![vec![default_cell; width]; height];
            let scrollback = VecDeque::with_capacity(1000);

            Self {
                width,
                height,
                cursor_x: 0,
                cursor_y: 0,
                cells,
                scrollback,
                modifiers: ModifiersState::empty(),
            }
        }

        pub fn handle_input(&mut self, input: KeyboardInput) {
            if let Some(keycode) = input.virtual_keycode {
                match input.state {
                    ElementState::Pressed => match keycode {
                        VirtualKeyCode::Back => self.handle_backspace(),
                        VirtualKeyCode::Delete => self.handle_delete(),
                        VirtualKeyCode::Left => self.handle_move_left(),
                        VirtualKeyCode::Right => self.handle_move_right(),
                        VirtualKeyCode::Up => self.handle_move_up(),
                        VirtualKeyCode::Down => self.handle_move_down(),
                        VirtualKeyCode::Return => self.handle_newline(),
                        VirtualKeyCode::Home => self.handle_home(),
                        VirtualKeyCode::End => self.handle_end(),
                        _ => self.handle_insert_char(keycode),
                    },
                    _ => {}
                }
            }
        }

        fn handle_insert_char(&mut self, keycode: VirtualKeyCode) {
            if let Some(c) = self.keycode_to_char(keycode) {
                if self.cursor_x >= self.width {
                    self.handle_newline();
                }

                self.cells[self.cursor_y][self.cursor_x] = Cell {
                    char: c,
                    fg_color: [1.0, 1.0, 1.0],
                    bg_color: [0.0, 0.0, 0.0],
                };
                self.cursor_x += 1;
            }
        }

        fn keycode_to_char(&self, keycode: VirtualKeyCode) -> Option<char> {
            let shift = self.modifiers.shift();
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

        // ... (keep all other terminal methods the same)
    }
}

// 🎨 Rendering
mod render {
    use super::*;
    use wgpu::util::DeviceExt;
    use wgpu_glyph::{GlyphBrush, GlyphBrushBuilder, Section, Text};

    pub struct RenderState {
        surface: wgpu::Surface,
        device: wgpu::Device,
        queue: wgpu::Queue,
        config: wgpu::SurfaceConfiguration,
        glyph_brush: GlyphBrush<()>,
        window: Window,
    }

    impl RenderState {
        pub async fn new(window: Window) -> Self {
            let size = window.inner_size();

            let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
                backends: wgpu::Backends::all(),
                ..Default::default()
            });
            let surface = unsafe { instance.create_surface(&window) }.unwrap();

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

            let config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: surface.get_capabilities(&adapter).formats[0],
                width: size.width,
                height: size.height,
                present_mode: wgpu::PresentMode::Fifo,
                alpha_mode: wgpu::CompositeAlphaMode::Auto,
                view_formats: vec![],
            };
            surface.configure(&device, &config);

            // Create glyph brush for text rendering
            let glyph_brush =
                GlyphBrushBuilder::using_font_bytes(include_bytes!("FiraMono-Regular.ttf"))
                    .build(&device, config.format);

            Self {
                surface,
                device,
                queue,
                config,
                glyph_brush,
                window,
            }
        }

        pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
            if new_size.width > 0 && new_size.height > 0 {
                self.config.width = new_size.width;
                self.config.height = new_size.height;
                self.surface.configure(&self.device, &self.config);
                self.glyph_brush
                    .resize_view(new_size.width, new_size.height);
            }
        }

        pub fn request_redraw(&self) {
            self.window.request_redraw();
        }

        pub fn render(&mut self, terminal: &terminal::Terminal) -> Result<(), wgpu::SurfaceError> {
            let output = self.surface.get_current_texture()?;
            let view = output
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            // Clear screen
            {
                let mut encoder =
                    self.device
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

                self.queue.submit(std::iter::once(encoder.finish()));
            }

            // Render text
            self.glyph_brush.queue(Section {
                screen_position: (30.0, 30.0),
                bounds: (f32::INFINITY, f32::INFINITY),
                text: vec![Text::new("Hello Terminal!")
                    .with_color([1.0, 1.0, 1.0, 1.0])
                    .with_scale(24.0)],
                ..Section::default()
            });

            self.glyph_brush
                .draw_queued(
                    &self.device,
                    &mut self.queue,
                    &view,
                    self.config.width,
                    self.config.height,
                )
                .expect("Draw queued");

            output.present();

            Ok(())
        }
    }
}

fn main() {
    pollster::block_on(async {
        let app = TerminalApp::new().await;
        app.run();
    });
}
