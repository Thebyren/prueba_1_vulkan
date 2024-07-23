use anyhow::{Ok, Result};
use vulkanalia::vk::DeviceV1_0;
use winit::{dpi::LogicalSize, event_loop::EventLoop, window::{Window, WindowBuilder}};
use winit::event::{Event, WindowEvent};
use crate::vulkan::app::App;

pub fn init_window(event_loop: &EventLoop<()>)->Result<Window>{
    Ok(
        WindowBuilder::new()
        .with_title("Vulkan Tutorial (Rust)")
        .with_inner_size(LogicalSize::new(500, 400))
        .build(event_loop)?
    )
}

pub fn run_event_loop(event_loop: EventLoop<()>, window:Window, app: &mut App)->Result<()>{ 
    event_loop.run(move |event, elwt| {
            match event {
            // Request a redraw when all events were processed.
            Event::AboutToWait => window.request_redraw(),
            Event::WindowEvent { event, .. } => match event {
                // Render a frame if our Vulkan app is not being destroyed.
                WindowEvent::RedrawRequested if !elwt.exiting() => {
                    unsafe { app.render(&window) }.unwrap()
                }
                // Destroy our Vulkan app.
                WindowEvent::CloseRequested => {
                    elwt.exit();

                    unsafe {
                        app.device.device_wait_idle().unwrap();
                    }
                    unsafe {
                        app.destroy();
                    }
                }
                _ => {}
            },
            _ => {}
        }
    })?;
    Ok(())
}
