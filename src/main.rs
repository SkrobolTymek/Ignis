use winit::{
    event::{self, *},
    event_loop::{self, EventLoopBuilder},
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

#[derive(Debug, Clone, Copy)]
enum CustomEvent {
    Timer,
}

fn main() {
    env_logger::init();

    let event_loop = EventLoopBuilder::<CustomEvent>::with_user_event()
        .build()
        .unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    
    let event_loop_proxy = event_loop.create_proxy();

    std::thread::spawn(move || loop{
        std::thread::sleep(std::time::Duration::from_millis(17));
        event_loop_proxy.send_event(CustomEvent::Timer).ok();
    });

    event_loop.run(move |event, elwt | match event{
        Event::UserEvent(..)=>{
            // println!("new frame")
        },
        Event::WindowEvent{
            window_id,
            ref event
        } if window_id == window.id() => match event {

            WindowEvent::CloseRequested => {
                println!("Closing lol");
                elwt.exit();
            }

            _=>(),
        },
        _=>{

        }
    }).expect("error");
}
