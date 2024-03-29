#[cfg(target_arch="wasm32")]
use wasm::bindgen::prelude::*;

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }
    env_logger::init();
    let event_loop: winit::event_loop::EventLoop<()> = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new().build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")] {
        use winit::dpi::PhysicalSize;
        window.set_inner_size(PhysicalSize::new(450, 400));

        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("body")?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Failed to append canvas to document body.");
    }

    event_loop.run(move |event: winit::event::Event<()>, _, control_flow: &mut winit::event_loop::ControlFlow| match event {
        winit::event::Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            winit::event::WindowEvent::CloseRequested | winit::event::WindowEvent::KeyboardInput {
                input: winit::event::KeyboardInput {
                    state: winit::event::ElementState::Pressed,
                    virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
                    ..
                },
                ..
            } => *control_flow = winit::event_loop::ControlFlow::Exit,
            _ => {}
        },
        _ => {}
    });
}