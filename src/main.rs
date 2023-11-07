use std::f64::consts::PI;
use std::{thread::sleep, time::Duration};
use termion::cursor::{Goto, Hide};
use termion::terminal_size;

fn main() {
    // let initial_vertices: [(f32, f32); 3] = [(0.5, 1.0), (0.0, 0.0), (1.0, 0.0)];
    let initial_vertices: [(f32, f32); 3] = [(0.5, 0.8), (0.2, 0.2), (0.8, 0.2)];
    let mut clipspace_vertices: [(f32, f32); 3] = initial_vertices;

    let (mut x, mut y);
    let mut vertices: [(f32, f32); 3];
    let mut map: Vec<Vec<char>>;

    loop {
        sleep(Duration::from_millis(1));

        (x, y) = get_dimensions();
        clipspace_vertices = transform_coord(clipspace_vertices);
        vertices = convert_to_clipspace(x, y, clipspace_vertices);
        //println!("{:?}", vertices);

        //println!("{:?}", vertices);
        map = create_map(x, y);

        //println!("{:?}", vertices);
        map = rasterise(map, vertices);
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
    let map: Vec<Vec<char>> = vec![vec![' '; x as usize]; y as usize];
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
                map[l][p] = 'X';
            }
        }
    }
    map
}

fn transform_coord(mut vertices: [(f32, f32); 3]) -> [(f32, f32); 3] {
    let rot_point: (f32, f32) = (0.5, 0.5);
    //println!("rot point {:?}", rot_point);
    let mut i = 0;
    for coord in vertices {
        let deg: f32 = 0.000698;
        let initial_coord: (f32, f32) = (coord.0 - rot_point.0, coord.1 - rot_point.1);
        //println!("initial coord: {:?}", initial_coord);
        let mut final_coord: (f32, f32) = (
            (initial_coord.0 * deg.cos() + initial_coord.1 * -deg.sin()),
            (initial_coord.0 * deg.sin() + initial_coord.1 * deg.cos()),
        );
        //println!("final coord 1 {:?}", final_coord);
        final_coord = (final_coord.0 + rot_point.0, final_coord.1 + rot_point.1);

        //println!("final coord 2 {:?}", final_coord);

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
