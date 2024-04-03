//fn to tran [red:10, green:10, blue:23] -> [color_index[10], color_index[10], color_index[23]]
fn pack(pixels: Vec<[u8; 3]>) -> Vec<u8> {
    let mut packed: Vec<u8> = Vec::new();
    for pixel in pixels {
        let color_index = (pixel[0] / 8) << 5 | (pixel[1] / 8) << 2 | (pixel[2] / 8);
        packed.push(color_index);
    }
    packed
}
pub fn main() {
    let pixels = vec![[10, 10, 23], [10, 10, 23], [10, 10, 23]];
    let packed = pack(pixels);
    println!("{:?}", packed);
}