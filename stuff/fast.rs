struct Image {
    width: u32,
    height: u32,
    pixels: Vec<u32>,
}

//fn RGBAtoSingleValue(pixel: [u8; 4]) -> u32 
fn RGBAtoSingleValue(pixel: [u8; 4]) -> u32 {
    let mut result: u32 = 0;
    result += (pixel[0] as u32) << 24;
    result += (pixel[1] as u32) << 16;
    result += (pixel[2] as u32) << 8;
    result += pixel[3] as u32;
    return result;
}
fn SingleValue_to_rgba(pixel: u32) -> [u8; 4] {
    let mut result: [u8; 4] = [0, 0, 0, 0];
    result[0] = (pixel >> 24) as u8;
    result[1] = (pixel >> 16) as u8;
    result[2] = (pixel >> 8) as u8;
    result[3] = pixel as u8;
    return result;
}
fn main() {
    let pixel: [u8; 4] = [255, 255, 255, 255];
    let result = RGBAtoSingleValue(pixel);
    println!("{}", result);
    println!("{:?}", SingleValue_to_rgba(result));
}

fn debug_read_registers_at(&self, index: i32) -> f64 {
    if self.debug == true {
        if index < 0 || index > 31 {
            panic!("Invalid register index");
        }
        self.registers[index as usize]        
    } else {    
        0.0
    }
}
fn debug_write_registers_at(&mut self, index: i32, value: f64) {
    if self.debug == true {
        if index < 0 || index > 31 {
            panic!("Invalid register index");
        }
        self.registers[index as usize] = value;
    } 
}