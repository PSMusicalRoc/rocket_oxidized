mod libraries;
use libraries::logger::log as log;
mod engine;

use std::{
    collections::HashSet,
    ffi::CStr,
    os::raw::c_void
};

use anyhow::{anyhow, Result};

use vulkanalia::{
    loader::{LibloadingLoader, LIBRARY},
    prelude::v1_0::*,
    Version,
    vk::ExtDebugUtilsExtension,
    window as vk_window,
};

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

const PORTABILITY_MACOS_VERSION: Version = Version::new(1, 3, 216);
const VALIDATION_ENABLED: bool = cfg!(debug_assertions);
const VALIDATION_LAYER: vk::ExtensionName = vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");

fn main() {

    log::info("Welcome to Rocket-Oxidized vNULL");


    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = unsafe { App::create(&window).unwrap() };
    let mut destroying = false;

    let _ = event_loop.run(move |event, elwt| {
        match event {

            // On Clicking the "X" button
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                destroying = true;
                elwt.exit();
                unsafe { app.destroy(); }
            },

            // Any wait-time, meaning we should redraw the screen.
            Event::AboutToWait if !destroying => {
                // render code here
                unsafe { app.render(&window) }.unwrap();
            }

            // All other events
            _ => {}
        }
    });


    if engine::status::is_engine_running() { engine::status::engine_stop(); }

    log::info("Rocket-Oxidized is closing!");

}


unsafe fn create_instance(window: &Window, entry: &Entry) -> Result<Instance> {
    let application_info = vk::ApplicationInfo::builder()
        .application_name(b"Vulkan Tutorial\0")
        .application_version(vk::make_version(1, 0, 0))
        .engine_name(b"No Engine\0")
        .engine_version(vk::make_version(1, 0, 0))
        .api_version(vk::make_version(1, 0, 0));

        let mut extensions = vk_window::get_required_instance_extensions(window)
        .iter()
        .map(|e| e.as_ptr())
        .collect::<Vec<_>>();
    
    // Required by Vulkan SDK on macOS since 1.3.216.
    let flags = if 
        cfg!(target_os = "macos") && 
        entry.version()? >= PORTABILITY_MACOS_VERSION
    {
        log::info("Enabling extensions for macOS portability.");
        extensions.push(vk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_EXTENSION.name.as_ptr());
        extensions.push(vk::KHR_PORTABILITY_ENUMERATION_EXTENSION.name.as_ptr());
        vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
    } else {
        vk::InstanceCreateFlags::empty()
    };

    let available_layers = entry
        .enumerate_instance_layer_properties()?
        .iter()
        .map(|l| l.layer_name)
        .collect::<HashSet<_>>();

    if VALIDATION_ENABLED && !available_layers.contains(&VALIDATION_LAYER) {
        return Err(anyhow!("Validation layer requested but not supported."));
    }

    let layers = if VALIDATION_ENABLED {
        vec![VALIDATION_LAYER.as_ptr()]
    } else {
        Vec::new()
    };
    
    let mut info = vk::InstanceCreateInfo::builder()
        .application_info(&application_info)
        .enabled_extension_names(&layers)
        .enabled_extension_names(&extensions)
        .flags(flags);
    
    // let mut debug_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
    //     .message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::all())
    //     .message_type(vk::DebugUtilsMessageTypeFlagsEXT::all())
    //     .user_callback(None);

    // if VALIDATION_ENABLED {
    //     info = info.push_next(&mut debug_info);
    // }
        

    Ok(entry.create_instance(&info, None)?)
    
}





#[derive (Clone, Debug)]
struct App {
    entry: Entry,
    instance: Instance,
}

impl App {
    unsafe fn create(window: &Window) -> Result<Self> {
        let loader = LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
        let instance = create_instance(window, &entry)?;
        Ok(Self { entry, instance })
    }
    
    

    unsafe fn render(&mut self, window: &Window) -> Result<()> {
        Ok(())
    }

    unsafe fn destroy(&mut self) {
        self.instance.destroy_instance(None);
    }
}

#[derive(Clone, Debug, Default)]
struct AppData{}
