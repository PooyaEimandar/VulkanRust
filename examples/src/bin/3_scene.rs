use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use VulkanRust::*;

pub struct Scene {}

// implement Node's interface
impl Node for Scene {
    fn load(&mut self) -> Result<(), ash::vk::Result> {
        println!("Scene is loading");
        Ok(())
    }

    fn unload(&mut self) -> Result<(), ash::vk::Result> {
        println!("Scene is unloading");
        Ok(())
    }

    fn update(&mut self) -> Result<(), ash::vk::Result> {
        println!("Scene is updating");
        Ok(())
    }

    fn render(&mut self) -> Result<(), ash::vk::Result> {
        println!("Scene is rendering");
        Ok(())
    }
}

// implement the main function
impl Scene {
    fn run(&mut self) -> Result<(), ash::vk::Result> {
        let ret = self.update();
        match ret {
            Ok(_) => self.render(),
            Err(e) => Err(e),
        }
    }
}

fn main() {
    // create event loop
    let event_loop = EventLoop::new();

    // create window
    let window = WindowBuilder::new()
        .with_title("Vulkan Rust: 3_scene")
        .build(&event_loop)
        .unwrap();

    // create Scene
    let mut scene = Scene {};
    let _ret = scene.load();

    // run event loop which handles window's events
    event_loop.run(move |event, _eloop_win_target, control_flow| {
        *control_flow = ControlFlow::Wait;

        let _ = scene.run();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::LoopDestroyed => {
                let ret = scene.unload();
                match ret {
                    Ok(_) => {
                        println!("scene just unloaded successfully");
                    }
                    Err(e) => {
                        println!("scene could not unload because {}", e);
                    }
                }
            }
            _ => (),
        }
    });
}
