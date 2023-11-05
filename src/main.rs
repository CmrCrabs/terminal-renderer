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
        sleep(Duration::from_millis(1));
        (x, y) = get_dimensions();
        vertices = convert_to_clipspace(x, y, initial_vertices);
        map = create_map(x, y);
        map = calc_vert(vertices, x, y, map);
        map = calc_line(map, vertices);

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

fn calc_line(mut map: Vec<Vec<char>>, vertices: [(f32, f32); 3]) -> Vec<Vec<char>> {
    // TODO: refactor this garbage eventually
    // TODO: entiere thing mirrored in x axis why idk??
    for coord in 0..vertices.len() {
        let initial_coord = vertices[coord];
        let next_coord: (f32, f32);
        match coord {
            coord if coord == vertices.len() - 1 => {
                next_coord = vertices[0];
            }
            _ => {
                next_coord = vertices[coord + 1];
            }
        }
        let mut x_jump: u32 = 0;
        match x_jump {
            _a if initial_coord.0 < next_coord.0 => {
                x_jump = next_coord.0 as u32 - initial_coord.0 as u32;
            }
            _ => {
                x_jump = initial_coord.0 as u32 - next_coord.0 as u32;
            }
        }
        let mut y_jump: u32 = 0;

        match y_jump {
            _a if initial_coord.1 < next_coord.1 => {
                y_jump = next_coord.1 as u32 - initial_coord.1 as u32;
            }
            _ => {
                y_jump = initial_coord.1 as u32 - next_coord.1 as u32;
            }
        }
        let gradient: f32 = y_jump as f32 / x_jump as f32;
        let mut pos_coord: (f32, f32) = initial_coord;

        // will refactor later into less disgusting i swear
        if initial_coord.1 < next_coord.1 && initial_coord.0 > next_coord.0 {
            for _i in 0..(x_jump - 2) as u32 {
                pos_coord = (pos_coord.0 - 1.0, pos_coord.1 + gradient as f32);
                map[pos_coord.1 as usize][pos_coord.0 as usize] = 'X';
            }
        } else if initial_coord.0 < next_coord.0 && initial_coord.1 > next_coord.1 {
            for _i in 0..(x_jump - 2) as u32 {
                pos_coord = (pos_coord.0 + 1.0, pos_coord.1 - gradient);
                map[pos_coord.1 as usize][pos_coord.0 as usize] = 'X';
            }
        } else if initial_coord.1 > next_coord.1 && initial_coord.0 > next_coord.0 {
            for _i in 0..(x_jump - 2) as u32 {
                pos_coord = (pos_coord.0 - 1.0, pos_coord.1 - gradient as f32);
                map[pos_coord.1 as usize][pos_coord.0 as usize] = 'X';
            }
        } else if initial_coord.1 < next_coord.1 && initial_coord.0 > initial_coord.0 {
            for _i in 0..(x_jump - 2) as u32 {
                pos_coord = (pos_coord.0 + gradient, pos_coord.1 + 1.0);
                map[pos_coord.1 as usize][pos_coord.0 as usize] = 'X';
            }
        } else if initial_coord.1 == next_coord.1 {
            for _i in 0..(x_jump - 2) as u32 {
                pos_coord = (pos_coord.0 + 1.0, pos_coord.1);
                map[pos_coord.1 as usize][pos_coord.0 as usize] = 'X';
            }
        } else if initial_coord.0 == next_coord.0 {
            for _i in 0..(y_jump - 2) as u32 {
                pos_coord = (pos_coord.0, pos_coord.1 - 1.0);
                map[pos_coord.1 as usize][pos_coord.0 as usize] = 'X';
            }
        }
    }

    map
}

fn calc_fill() {}

fn render(map: &Vec<Vec<char>>) {
    for row in map {
        for pixel in row {
            print!("{}", pixel);
        }
    }
}
