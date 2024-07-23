use anyhow::{Ok, *};
use vulkanalia::prelude::v1_2::*;

// utilidades del conector con el manejador de ventanas.
use vulkanalia::vk::KhrSurfaceExtension;
use vulkanalia::vk::SurfaceKHR;
use crate::vulkan::logical_device::SuitabilityError;
use crate::vulkan::app::AppData;
pub struct QueueFamilyIndices {
    pub graphics: u32,
    pub present: u32,
}

impl QueueFamilyIndices {
    pub unsafe fn get(
        instance: &Instance,
        data: &AppData,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Self> {
        let mut present = None;
        let properties: Vec<vk::QueueFamilyProperties> =
            instance.get_physical_device_queue_family_properties(physical_device);
        let graphics = properties
            .iter()
            .position(|p| p.queue_flags.contains(vk::QueueFlags::GRAPHICS))
            .map(|i| i as u32);

        if data.surface == SurfaceKHR::null() {
            return Err(anyhow!(SuitabilityError("superficie sin crear o nula")));
        }

        for (index, properties) in properties.iter().enumerate() {
            match instance.get_physical_device_surface_support_khr(
                physical_device,
                index as u32,
                data.surface,
            ) {
                Result::Ok(support) => {
                    if support {
                        present = Some(index as u32);
                        break;
                    }
                }
                Err(e) => {
                    println!("error al comprobar soporte");
                    return Err(anyhow!(SuitabilityError(
                        "error al verificar el soporte a la superficie"
                    )));
                }
            }
        }
        if let (Some(graphics), Some(present)) = (graphics, present) {
            Ok(Self { graphics, present })
        } else {
            Err(anyhow!(SuitabilityError(
                "no se encontraron las familias de cola requeridas"
            )))
        }
    }
}
