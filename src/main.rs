use std::collections::VecDeque;
use wgpu;
use winit::{
    event::{Event, KeyboardInput, VirtualKeyCode, ElementState, ModifiersState, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
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
                    state.window().request_redraw();
                },

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
                    state.window().request_redraw();
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
        pub fg_color: [f32; 4],  // Changed to RGBA
        pub bg_color: [f32; 4],  // Changed to RGBA
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
                fg_color: [1.0, 1.0, 1.0, 1.0],  // Added alpha
                bg_color: [0.0, 0.0, 0.0, 1.0],  // Added alpha
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
                        VirtualKeyCode::Back => self.backspace(),
                        VirtualKeyCode::Delete => self.delete(),
                        VirtualKeyCode::Left => self.move_left(),
                        VirtualKeyCode::Right => self.move_right(),
                        VirtualKeyCode::Up => self.move_up(),
                        VirtualKeyCode::Down => self.move_down(),
                        VirtualKeyCode::Return => self.newline(),
                        VirtualKeyCode::Home => self.home(),
                        VirtualKeyCode::End => self.end(),
                        _ => self.insert_char(keycode),
                    },
                    _ => {}
                }
            }
        }

        fn insert_char(&mut self, keycode: VirtualKeyCode) {
            if let Some(c) = self.keycode_to_char(keycode) {
                if self.cursor_x >= self.width {
                    self.newline();
                }

                self.cells[self.cursor_y][self.cursor_x] = Cell {
                    char: c,
                    fg_color: [1.0, 1.0, 1.0, 1.0],
                    bg_color: [0.0, 0.0, 0.0, 1.0],
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

        fn backspace(&mut self) {
            if self.cursor_x > 0 {
                self.cursor_x -= 1;
                self.cells[self.cursor_y][self.cursor_x] = Cell {
                    char: ' ',
                    fg_color: [1.0, 1.0, 1.0, 1.0],
                    bg_color: [0.0, 0.0, 0.0, 1.0],
                };
            }
        }

        fn delete(&mut self) {
            self.cells[self.cursor_y][self.cursor_x] = Cell {
                char: ' ',
                fg_color: [1.0, 1.0, 1.0, 1.0],
                bg_color: [0.0, 0.0, 0.0, 1.0],
            };
        }

        fn move_left(&mut self) {
            if self.cursor_x > 0 {
                self.cursor_x -= 1;
            }
        }

        fn move_right(&mut self) {
            if self.cursor_x < self.width - 1 {
                self.cursor_x += 1;
            }
        }

        fn move_up(&mut self) {
            if self.cursor_y > 0 {
                self.cursor_y -= 1;
            }
        }

        fn move_down(&mut self) {
            if self.cursor_y < self.height - 1 {
                self.cursor_y += 1;
            }
        }

        fn newline(&mut self) {
            self.cursor_x = 0;
            if self.cursor_y < self.height - 1 {
                self.cursor_y += 1;
            } else {
                self.scroll();
            }
        }

        fn home(&mut self) {
            self.cursor_x = 0;
        }

        fn end(&mut self) {
            self.cursor_x = self.width - 1;
        }

        fn scroll(&mut self) {
            let row = self.cells.remove(0);
            self.scrollback.push_back(row);
            self.cells.push(vec![
                Cell {
                    char: ' ',
                    fg_color: [1.0, 1.0, 1.0, 1.0],
                    bg_color: [0.0, 0.0, 0.0, 1.0],
                };
                self.width
            ]);
        }
    }
}



// 🎨 Rendering
mod render {
    use super::*;
    use wgpu_glyph::{ab_glyph, GlyphBrush, GlyphBrushBuilder, Section, Text};

    
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
                .request_device(&wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                }, None)
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
            let font_data = include_bytes!("SpecialGothic-Regular.ttf");
            let font = ab_glyph::FontArc::try_from_slice(font_data).unwrap();
            let glyph_brush = GlyphBrushBuilder::using_font(font)
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
            }
        }

        pub fn window(&self) -> &Window {
            &self.window
        }

        pub fn render(&mut self, terminal: &terminal::Terminal) -> Result<(), wgpu::SurfaceError> {
            let output = self.surface.get_current_texture()?;
            let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

            let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

            // Clear screen
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

            // Render all terminal cells
            let cell_width = 10.0;
            let cell_height = 20.0;
            let font_size = 16.0;

            for (y, row) in terminal.cells.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    if cell.char != ' ' {
                        self.glyph_brush.queue(Section {
                            screen_position: (
                                x as f32 * cell_width + 10.0,
                                y as f32 * cell_height + 10.0,
                            ),
                            bounds: (f32::INFINITY, f32::INFINITY),
                            text: vec![Text::new(&cell.char.to_string())
                                .with_color(cell.fg_color)
                                .with_scale(font_size)],
                            ..Section::default()
                        });
                    }
                }
            }

            // Draw cursor
            self.glyph_brush.queue(Section {
                screen_position: (
                    terminal.cursor_x as f32 * cell_width + 10.0,
                    terminal.cursor_y as f32 * cell_height + 10.0,
                ),
                bounds: (f32::INFINITY, f32::INFINITY),
                text: vec![Text::new("_")
                    .with_color([1.0, 1.0, 1.0, 1.0])
                    .with_scale(font_size)],
                ..Section::default()
            });



            // Draw queued text
            self.glyph_brush
                .draw_queued(
                    &self.device,
                    &mut self.queue,
                    &view,
                    wgpu_glyph::orthographic_projection(self.config.width, self.config.height),
                    wgpu_glyph::ScreenSize {
                        width: self.config.width as f32,
                        height: self.config.height as f32,
                    },
                    wgpu_glyph::Scale::uniform(font_size),
                )
                .expect("Draw queued");

            self.queue.submit(std::iter::once(encoder.finish()));
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