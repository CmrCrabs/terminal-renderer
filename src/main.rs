use std::{thread::sleep, time::Duration};
use termion::terminal_size;
use termion::{
    clear,
    cursor::{Goto, Hide},
};

fn main() {
    let initial_vertices: [(f32, f32); 3] = [(0.5, 1.0), (0.0, 0.0), (1.0, 0.0)];

    let (mut x, mut y);
    let mut vertices: [(f32, f32); 3];
    let mut map: Vec<Vec<char>>;

    loop {
        sleep(Duration::from_millis(17));
        (x, y) = get_dimensions();
        vertices = convert_to_clipspace(x, y, initial_vertices);
        map = create_map(x, y);
        map = calc_vert(vertices, x, y, map);

        print!("{}{}", Hide, Goto(1, 1));
        render(&map);
    }
}

// CURRENT PROBLEMS
// final branch of match staement does not work

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

fn calc_vert(vertices: [(f32, f32); 3], x: f32, y: f32, mut map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for coord in vertices {
        let coord_real: (u32, u32) = (coord.0 as u32, coord.1 as u32);

        match coord_real {
            (a, b) if a == x as u32 && b == y as u32 => {
                map[y as usize - (coord.1 as usize + 1)][coord.0 as usize + 1] = 'X';
            }
            (0, 0) => {
                map[(y as usize - 1) - coord.1 as usize][coord.0 as usize] = 'X';
            }
            (0, _) => {
                map[(y as usize - 1) - coord.1 as usize][coord.0 as usize] = 'X';
            }
            (_, 0) => {
                map[y as usize - coord.1 as usize - 1][coord.0 as usize - 1] = 'X';
            }
            (_, _) => {
                map[(y as usize) - coord.1 as usize][coord.0 as usize] = 'X';
            }
        }
    }

    map
}

fn calc_frag() {}

fn create_tri() {}

fn render(map: &Vec<Vec<char>>) {
    for row in map {
        for pixel in row {
            print!("{}", pixel);
        }
    }
}
