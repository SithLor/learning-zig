use std::time::Duration;
use std::thread::sleep;
use rand::Rng;
use image::{ImageBuffer, Rgb};

const WIDTH: usize = 20;
const HEIGHT: usize = 20;
const ALIVE: [u8; 3] = [0, 0, 0]; // Black for alive cells
const DEAD: [u8; 3] = [255, 255, 255]; // White for dead cells
const REFRESH_RATE: u64 = 500;
const OUTPUT_DIR: &str = "output/";
const MAX_GENERATIONS: usize = 10;

fn main() {
    let mut grid = [[false; WIDTH]; HEIGHT];
    let mut rng = rand::thread_rng();

    // Initialize grid with random values
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            grid[i][j] = rng.gen();
        }
    }

    let mut generation = 0;
    while generation < MAX_GENERATIONS  {
        // Create a new image for each generation
        let mut img = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            *pixel = Rgb(if grid[y as usize][x as usize] { ALIVE } else { DEAD });
        }
        img.save(format!("{}generation{}.png", OUTPUT_DIR, generation)).unwrap();

        grid = next_generation(&grid);
        sleep(Duration::from_millis(REFRESH_RATE));
        generation += 1;
    }
}

fn next_generation(grid: &[[bool; WIDTH]; HEIGHT]) -> [[bool; WIDTH]; HEIGHT] {
    let mut new_grid = [[false; WIDTH]; HEIGHT];

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let neighbors = [
                grid[(i+HEIGHT-1)%HEIGHT][(j+WIDTH-1)%WIDTH],
                grid[(i+HEIGHT-1)%HEIGHT][j],
                grid[(i+HEIGHT-1)%HEIGHT][(j+1)%WIDTH],
                grid[i][(j+WIDTH-1)%WIDTH],
                grid[i][(j+1)%WIDTH],
                grid[(i+1)%HEIGHT][(j+WIDTH-1)%WIDTH],
                grid[(i+1)%HEIGHT][j],
                grid[(i+1)%HEIGHT][(j+1)%WIDTH],
            ].iter().filter(|&&x| x).count();

            new_grid[i][j] = match (grid[i][j], neighbors) {
                (true, x) if x < 2 || x > 3 => false,
                (false, 3) => true,
                (otherwise, _) => otherwise
            };
        }
    }

    new_grid
}