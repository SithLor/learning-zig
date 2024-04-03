function generateRGB_to_hex_table() {
    let hex_table = [];
    for (let i = 0; i < 256; i++) {
        for (let j = 0; j < 256; j++) {
            for (let k = 0; k < 256; k++) {
                let hex = [i, j, k].map(value => `0x${value.toString(16).padStart(2, '0')}`).join('');
                hex_table.push(hex);
            }
        }
    }
    return hex_table;
}

let hex_table = generateRGB_to_hex_table();
console.log(`export const RGB = [${JSON.stringify(hex_table)}];`);