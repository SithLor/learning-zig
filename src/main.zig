//y=a(1+r)^n


//pixel has x,y,color: 0,0,0
const rgb = struct {
    u8: r,
    u8: g,
    u8: b
};
const Point = struct {
    x: i64,
    y: i64,
    color_red:u8,
    color_green:u8,
    color_blue:u8
};

pub fn draw_line(p1: Point, p2: Point) -> Vec<Point> {
    let mut points = Vec::new();
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    let m = dy as f64 / dx as f64;
    let b = p1.y - m * p1.x as f64;
    let mut x = p1.x;
    let mut y = p1.y;
    let mut i = 0;
    while i < dx {
        x = p1.x + i;
        y = m * x as f64 + b;
        points.push(Point {
            x: x,
            y: y as i64,
            color_red: p1.color_red,
            color_green: p1.color_green,
            color_blue: p1.color_blue,
        });
        i += 1;
    }
    points
}