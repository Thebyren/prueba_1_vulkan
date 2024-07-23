#![allow(
    dead_code,
    unused_variables,
    clippy::manual_slice_size_calculation,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

mod vulkan;
//use std::ops::ControlFlow;
use vulkan::window::init_window;
use crate::vulkan::prelude::*;

/// Whether the validation layers should be enabled.
const VALIDATION_ENABLED: bool = cfg!(debug_assertions);
/// The name of the validation layers.
const VALIDATION_LAYER: vk::ExtensionName =
    vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");

/// The required device extensions.
const DEVICE_EXTENSIONS: &[vk::ExtensionName] = &[vk::KHR_SWAPCHAIN_EXTENSION.name];
/// The Vulkan SDK version that started requiring the portability subset extension for macOS.
const PORTABILITY_MACOS_VERSION: Version = Version::new(1, 3, 216);

/// The maximum number of frames that can be processed concurrently.
const MAX_FRAMES_IN_FLIGHT: usize = 2;

#[rustfmt::skip]
fn main() -> Result<()> {
    pretty_env_logger::init();

    // Window
    print!("inicia el programa");

    let event_loop = EventLoop::new()?;
    let window = init_window(&event_loop)?;
    
    // App
    let mut app = unsafe { App::create(&window)? };
    let mut minimized = false;
    event_loop.run(
        move |event,
         elwt| {
        match event {
            // Request a redraw when all events were processed.
            Event::AboutToWait => window.request_redraw(),
            Event::WindowEvent { event, .. } => 
            match event {
                // Render a frame if our Vulkan app is not being destroyed.
                WindowEvent::RedrawRequested if !elwt.exiting() && !minimized => 
                    unsafe { app.render(&window) }.unwrap(),
                // Destroy our Vulkan app.
                WindowEvent::Resized(size)=>{
                    if size.width == 0 || size.height == 0{
                        minimized = true;
                    }else{
                        minimized = false;
                        app.resized = true;
                    }
                }
                WindowEvent::CloseRequested => {                    
                    elwt.exit();
                    unsafe { app.destroy(); }
                }
                _ => {}
            }
            _ => {}
        }
    })?;
    trace!("finaliza el programa");
    Ok(())
}

extern "system" fn debug_callback(
    severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    type_: vk::DebugUtilsMessageTypeFlagsEXT,
    data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _: *mut c_void,
) -> vk::Bool32 {
    let data = unsafe { *data };
    let message = unsafe { CStr::from_ptr(data.message) }.to_string_lossy();
    let message_id_name = unsafe { CStr::from_ptr(data.message_id_name).to_string_lossy()};
    let message_id_number = data.message_id_number;
    if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::ERROR {
        error!(
            "Validation Layer: [{}] ({}): {:?} - {}",
            message_id_name, message_id_number, type_, message
    );
    } else if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::WARNING {
        warn!("Validation Layer: [{}] ({}): {:?} - {}",
            message_id_name, message_id_number, type_, message);
    } else if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::INFO {
        debug!("Validation Layer: [{}] ({}): {:?} - {}",
            message_id_name, message_id_number, type_, message);
    } else {
        trace!("Validation Layer: [{}] ({}): {:?} - {}",
            message_id_name, message_id_number, type_, message);
    }
    

    vk::FALSE
}
