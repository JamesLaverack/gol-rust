extern crate rand;
use std::io::{self, Write};
use std::collections::HashMap;
use rand::{ThreadRng, Rng};
use std::time::Duration;
use std::env;

struct Grid {
    cells: HashMap<(i32, i32), bool>,
    width: i32,
    height: i32,
    rng: ThreadRng,
    ascii: bool
}

impl Grid {
    pub fn new(width: i32, height: i32, ascii: bool) -> Self {
        let mut grid = Grid {
            cells: HashMap::new(),
            width: width,
            height: height,
            rng: rand::thread_rng(),
            ascii: ascii
        };

        grid.each_position(|grid, coord| {
            grid.cells.insert(coord.clone(), false);
        });

        return grid;
    }

    pub fn step(&mut self) {
        let mut hmc = self.copy_grid();
        self.each_position(|grid, coords| {
            let alive_count = grid.alive_count(coords);
            if alive_count < 2 {
                *hmc.get_mut(coords).unwrap() = false;
            } else if alive_count >= 2 && alive_count <= 3 && grid.cells.get(coords).unwrap().clone() {
                *hmc.get_mut(coords).unwrap() = true;
            } else if alive_count > 3 {
                *hmc.get_mut(coords).unwrap() = false;
            } else if alive_count == 3 && grid.cells.get(coords).unwrap().clone() == false {
                *hmc.get_mut(coords).unwrap() = true;
            }
        });

        self.cells = hmc;
    }

    pub fn randomize(&mut self) {
        self.each_position(|grid, coords| {
            *grid.cells.get_mut(coords).unwrap() = grid.rng.gen();
        });
    }

    fn copy_grid(&mut self) -> HashMap<(i32, i32), bool> {
        let mut hmc = HashMap::new();

        self.each_position(|grid, coords| {
            hmc.insert(coords.clone(), grid.cells.get(coords).unwrap().clone());
        });

        return hmc;
    }

    fn alive_count(&self, coords: &(i32, i32)) -> i32 {
        let mut accum = 0;
        for y_offset in -1..2 {
            for x_offset in -1..2 {
                if x_offset == 0 && y_offset == 0 {
                    continue
                }
                let &(x,y) = coords;
                let new_coords = (x + x_offset, y + y_offset);
                match self.cells.get(&new_coords) {
                    Some(state) => match state {
                        &true => accum += 1,
                        &false => (),
                    },
                    None => (),
                }
            }
        }

        return accum;
    }

    fn each_position<T>(&mut self, mut callback: T) where T: FnMut(&mut Self, &(i32, i32)) {
        for y in 0..self.height {
            for x in 0..self.width {
                callback(self, &(x, y));
            }
        }
    }

    pub fn print_to(&self, output: &mut Write) -> io::Result<()> {
        for y in 0..self.height {
            for x in 0..self.width {
                let state = self.cells.get(&(x, y)).unwrap();
                try!(self.print_cell(state.clone(), output));
            }
            try!(output.write(b"\n"));
        }

        return Ok(());
    }

    fn print_alive_cell(&self, output: &mut Write) -> io::Result<usize> {
        if self.ascii {
            return output.write(b"x");
        } else {
            return output.write("\u{2588}".as_bytes());
        }
    }

    fn print_dead_cell(&self, output: &mut Write) -> io::Result<usize> {
        if self.ascii {
            return output.write(b".");
        } else {
            return output.write("\u{2591}".as_bytes());
        }
    }

    fn print_cell(&self, state: bool, output: &mut Write) -> io::Result<()> {
        let result = match state {
            true => self.print_alive_cell(output),
            false => self.print_dead_cell(output)
        };

        try!(result);

        return Ok(());

    }
}

fn main() {
    let ascii: bool = env::args().find(|s| s == "--ascii") != None;
    let mut g = Grid::new(70, 30, ascii);
    let mut stdout = io::stdout();
    g.randomize();
    let sleep_time = Duration::from_millis(100);
    loop {
        println!("\x1Bc");
        g.print_to(&mut stdout).unwrap();
        std::thread::sleep(sleep_time);
        g.step();
    }
}
