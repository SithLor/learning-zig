//this not FIPS compla
#![no_std]

 
mod crc32 {
    const CRC_TABLE: [u32; 256] = [
        0x00000000, 0x77073096, 0xee0e612c, 0x990951ba, 0x076dc419, 0x706af48f, 0xe963a535, 0x9e6495a3,
        // ... rest of the table values ...
    ];

    pub fn calculate_crc(data: &[u8]) -> u32 {
        let mut crc = 0xffffffff;
        for &byte in data {
            let index = ((crc ^ (byte as u32)) & 0xff) as usize;
            crc = (crc >> 8) ^ CRC_TABLE[index];
        }
        crc ^ 0xffffffff
    }

    pub fn check_crc(data: &[u8], expected_crc: u32) -> bool {
        calculate_crc(data) == expected_crc
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
