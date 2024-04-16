mod Mock {
    pub enum DeviceStatus {
        On,
        Off,
        Error
    }
    pub struct Device {
        name: String,
        id: u32,
        status: DeviceStatus,
        status_message: String
    }
    trait _Device {
        
    }
    pub trait Loader {
        fn load(&self) -> Device;
        fn unload(&self) -> Device;
    }
}

fn main(){
    //Create the Gpu struct
    struct Gpu {
        name: String,
        id: u32,
        status: Mock::DeviceStatus,
        status_message: String
    }


    //attach the loader trait to the Gpu struct
    impl Mock::Loader for Gpu {
        fn load(&self,name:String,id:i32) -> Gpu {
            Gpu {
                name: name,
                id: id,
                status: Mock::DeviceStatus::On,
                status_message: "Device is on".to_string()
            }
        }
        fn unload(&self) -> Mock::Device {
            Mock::new_device(self.name.clone(), self.id, Mock::DeviceStatus::Off, "Device is off".to_string())
        }
        
    }
    //Mock Device loader 




}