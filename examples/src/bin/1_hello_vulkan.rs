extern crate ash;

use ash::version::{EntryV1_0, InstanceV1_0};

fn main() {
    unsafe {
        let entry = ash::Entry::new().unwrap();

        let instance_create_info = ash::vk::InstanceCreateInfo::builder();

        let instance = entry
            .create_instance(&instance_create_info, None)
            .expect("Could not create Vulkan Instance");

        instance.destroy_instance(None);
        print!("Vulkan Instance just created and destroyed succesfully");
    }
}
