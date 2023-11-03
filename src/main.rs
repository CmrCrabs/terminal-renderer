use termion::clear;
use termion::terminal_size;

fn main() {
    // get triangle coords
    let initial_vertices: [(f32, f32); 3] = [(0.5, 1.0), (0.0, 0.0), (1.0, 0.0)];

    //  x y aixis flipped

    let (mut x, mut y) = get_dimensions();

    let vertices: [(f32, f32); 3] = convert_to_clipspace(x, y, initial_vertices);
    println!("{:?}", vertices);
    // create a grid for the screensize to be outputted
    let mut map: Vec<Vec<char>> = create_map(x, y);
    map = calc_vert(vertices, x, y, map);

    render(&map);

    //    loop {
    //        (x, y) = get_dimensions();
    //        map = create_map(x, y);
    //        map = calc_vert(vertices, map);
    //        render(&map);
    //       println!("{}", clear::All);
    // }

    //render(map);

    // calculate vertex position
    // calculate the lines position
    // calculte fragment
    // combine into 1
    // output triangle to terminal
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
    println!("x{:?} y{:?}", x, y);
    let map: Vec<Vec<char>> = vec![vec![' '; x as usize + 1]; y as usize + 1];
    map
}

fn calc_vert(vertices: [(f32, f32); 3], x: f32, y: f32, mut map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for coord in vertices {
        map[y as usize - coord.1 as usize][coord.0 as usize] = 'X';
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
