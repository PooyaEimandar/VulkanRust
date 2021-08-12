extern crate ash;

fn main() {
    unsafe {
        //create ash entry
        let entry_res = ash::Entry::new();
        match entry_res {
            Ok(entry) => {
                //create instance create info
                let instance_create_info = ash::vk::InstanceCreateInfo::builder();
                //create instance
                let instance_res = entry.create_instance(&instance_create_info, None);
                match instance_res {
                    Ok(ins) => {
                        println!("Vulkan Instance just created and then destroyed succesfully");
                        ins.destroy_instance(None);
                    }
                    Err(e) => {
                        println!("could not create Vulkan Instance because {}", e);
                    }
                }
            }
            Err(e) => {
                println!("could not load Vulkan entry because {}", e);
            }
        };
    }
}
