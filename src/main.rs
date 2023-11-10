use termion::cursor::{Goto, Hide};
use termion::terminal_size;

const THING: char = ' ';
const OTHER_THING: char = '󰝤';

fn main() {
    let mut _initial_vertices: [(f32, f32); 3] = [
        (0.2 * 0.8, 0.2),
        (0.8 * 0.8, 0.2),
        (0.5 * 0.8, 0.2 + (0.27 as f32).sqrt()),
    ];

    let mut triangle1_vertices: [(f32, f32); 3] =
        [(0.25 * 0.8, 0.5), (0.1 * 0.8, 0.2), (0.4 * 0.8, 0.2)];

    let mut triangle2_vertices: [(f32, f32); 3] =
        [(0.75 * 0.8, 0.5), (0.6 * 0.8, 0.2), (0.9 * 0.8, 0.2)];

    let (mut x, mut y);
    let mut vertices1: [(f32, f32); 3];
    let mut vertices2: [(f32, f32); 3];
    let mut map: Vec<Vec<char>>;

    loop {
        (x, y) = get_dimensions();
        triangle1_vertices = transform_coord(triangle1_vertices);
        vertices1 = convert_to_clipspace(x, y, triangle1_vertices);

        triangle2_vertices = transform_coord(triangle2_vertices);
        vertices2 = convert_to_clipspace(x, y, triangle2_vertices);

        map = create_map(x, y);

        map = rasterise(map, vertices1);
        map = rasterise(map, vertices2);
        print!("{}{}", Hide, Goto(1, 1));
        render(&map);
    }
}

fn get_dimensions() -> (f32, f32) {
    let (f, r) = terminal_size().unwrap();
    let (x, y) = (f as f32, r as f32);

    (x, y)
}

fn convert_to_clipspace(x: f32, y: f32, vertices: [(f32, f32); 3]) -> [(f32, f32); 3] {
    let mut new_vertices: [(f32, f32); 3] = vertices;
    for i in 0..vertices.len() {
        let mut coord = vertices[i];
        coord.0 = x * &coord.0;
        coord.1 = y * &coord.1;
        new_vertices[i] = coord;
    }
    new_vertices
}

fn create_map(x: f32, y: f32) -> Vec<Vec<char>> {
    let map: Vec<Vec<char>> = vec![vec![THING; x as usize]; y as usize];
    map
}

fn rasterise(mut map: Vec<Vec<char>>, vertices: [(f32, f32); 3]) -> Vec<Vec<char>> {
    for l in 0..map.len() - 1 {
        for p in 0..map[l].len() - 1 {
            let mut count = 0;

            for i in 0..vertices.len() {
                let v_0: (f32, f32, f32) = (vertices[i].0, vertices[i].1, 0.0);

                let v_1: (f32, f32, f32);
                if vertices[i] == vertices[vertices.len() - 1] {
                    v_1 = (vertices[0].0, vertices[0].1, 0.0);
                } else {
                    v_1 = (vertices[i + 1].0, vertices[i + 1].1, 0.0);
                }

                let p: (f32, f32, f32) = (p as f32, l as f32, 0.0);

                match ((v_1.0 - v_0.0) * (p.1 - v_0.1)) - ((v_1.1 - v_0.1) * (p.0 - v_0.0)) {
                    real if real > 0.0 => {
                        count = count + 1;
                    }
                    _ => {}
                }
            }
            if count == 3 {
                map[l][p] = OTHER_THING;
            }
        }
    }
    map
}

fn transform_coord(mut vertices: [(f32, f32); 3]) -> [(f32, f32); 3] {
    let rot_point: (f32, f32) = (
        ((vertices[0].0 + vertices[1].0 + vertices[2].0) / 3.0),
        ((vertices[0].1 + vertices[1].1 + vertices[2].1) / 3.0),
    );
    let mut i = 0;
    for coord in vertices {
        let deg: f32 = 0.00698;
        let initial_coord: (f32, f32) = (coord.0 - rot_point.0, coord.1 - rot_point.1);
        let mut final_coord: (f32, f32) = (
            (initial_coord.0 * deg.cos() + initial_coord.1 * -deg.sin()),
            (initial_coord.0 * deg.sin() + initial_coord.1 * deg.cos()),
        );
        final_coord = (final_coord.0 + rot_point.0, final_coord.1 + rot_point.1);

        vertices[i] = final_coord;
        i = i + 1;
    }

    vertices
}

fn render(map: &Vec<Vec<char>>) {
    for row in map {
        for pixel in row {
            print!("{}", pixel);
        }
    }
}
