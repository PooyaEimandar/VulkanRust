extern crate ash;
extern crate winit;

use ash::*;
use mimalloc::MiMalloc;

//using global allocator
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub fn error_string(p_error_code: vk::Result) -> String {
    match p_error_code {
        vk::Result::SUCCESS => "SUCCESS".to_string(),
        vk::Result::NOT_READY => "NOT_READY".to_string(),
        vk::Result::TIMEOUT => "TIMEOUT".to_string(),
        vk::Result::EVENT_SET => "EVENT_SET".to_string(),
        vk::Result::EVENT_RESET => "EVENT_RESET".to_string(),
        vk::Result::INCOMPLETE => "INCOMPLETE".to_string(),
        vk::Result::ERROR_OUT_OF_HOST_MEMORY => "ERROR_OUT_OF_HOST_MEMORY".to_string(),
        vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => "ERROR_OUT_OF_DEVICE_MEMORY".to_string(),
        vk::Result::ERROR_INITIALIZATION_FAILED => "ERROR_INITIALIZATION_FAILED".to_string(),
        vk::Result::ERROR_DEVICE_LOST => "ERROR_DEVICE_LOST".to_string(),
        vk::Result::ERROR_MEMORY_MAP_FAILED => "ERROR_MEMORY_MAP_FAILED".to_string(),
        vk::Result::ERROR_LAYER_NOT_PRESENT => "ERROR_LAYER_NOT_PRESENT".to_string(),
        vk::Result::ERROR_EXTENSION_NOT_PRESENT => "ERROR_EXTENSION_NOT_PRESENT".to_string(),
        vk::Result::ERROR_FEATURE_NOT_PRESENT => "ERROR_FEATURE_NOT_PRESENT".to_string(),
        vk::Result::ERROR_INCOMPATIBLE_DRIVER => "ERROR_INCOMPATIBLE_DRIVER".to_string(),
        vk::Result::ERROR_TOO_MANY_OBJECTS => "ERROR_TOO_MANY_OBJECTS".to_string(),
        vk::Result::ERROR_FORMAT_NOT_SUPPORTED => "ERROR_FORMAT_NOT_SUPPORTED".to_string(),
        vk::Result::ERROR_FRAGMENTED_POOL => "ERROR_FRAGMENTED_POOL".to_string(),
        vk::Result::ERROR_UNKNOWN => "ERROR_UNKNOWN".to_string(),
        _ => "ERROR_NOT_DEFINED".to_string(),
    }
}

pub trait Node {
    fn load(&mut self) -> Result<(), vk::Result>;
    fn unload(&mut self) -> Result<(), vk::Result>;
    fn update(&mut self) -> Result<(), vk::Result>;
    fn render(&mut self) -> Result<(), vk::Result>;
}

struct GraphicsDeviceInfo {
    pub properties: vk::PhysicalDeviceProperties,
    pub features: vk::PhysicalDeviceFeatures,
    pub extensions: Vec<String>,
}

impl GraphicsDeviceInfo {
    pub fn new() -> GraphicsDeviceInfo {
        GraphicsDeviceInfo {
            properties: Default::default(),
            features: Default::default(),
            extensions: vec!["VK_KHR_SWAPCHAIN_EXTENSION_NAME".to_string()],
        }
    }
}
