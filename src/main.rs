use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Alive,
    Dead,
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    // Create new random grid
    fn new(size: usize) -> Grid {
        let mut rng = rand::thread_rng();
        let cells: Vec<Vec<Cell>> = (0..size)
            .map(|_| {
                (0..size)
                    .map(|_| {
                        if rng.r#gen::<f64>() < 0.5 {
                            Cell::Dead
                        } else {
                            Cell::Alive
                        }
                    })
                    .collect()
            })
            .collect();
        Grid { cells }
    }

    fn update(&mut self) {
        let mut new_cells = self.cells.clone();
        for i in 0..self.cells.len() {
            for j in 0..self.cells.len() {
                let live_neighbors = self.live_neighbors_count(i, j);
                if self.cells[i][j] == Cell::Dead && live_neighbors == 3 {
                    new_cells[i][j] = Cell::Alive;
                } else if self.cells[i][j] == Cell::Alive
                    && (live_neighbors < 2 || live_neighbors > 3)
                {
                    new_cells[i][j] = Cell::Dead;
                }
            }
        }
        self.cells = new_cells;
    }

    fn live_neighbors_count(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                let other_x = (x as i32 + i) as usize;
                let other_y = (y as i32 + j) as usize;
                if other_x < self.cells.len() && other_y < self.cells.len() {
                    if self.cells[other_x][other_y] == Cell::Alive {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn render_console(&self) {
        for i in 0..self.cells.len() {
            for j in 0..self.cells.len() {
                if self.cells[i][j] == Cell::Alive {
                    print!("8")
                } else {
                    print!(" ")
                }
            }
            print!("\n")
        }
        
        // let _ = self.cells.iter().map(|v| {
        //     let _ = v.iter().map(|cell| {
        //         if *cell == Cell::Alive {
        //             print!("8")
        //         } else {
        //             print!(" ")
        //         }
        //     });
        //     print!("Хоба\n")
        // });
        Grid::pause();
    }

    fn pause() {
        loop {
            match event::read().unwrap() {
                Event::Key(KeyEvent {
                    code: KeyCode::Char(' '),
                    modifiers: KeyModifiers::NONE,
                    ..
                }) => break,
                _ => continue,
            }
        }
    }
}

fn main() {
    let size: usize = 20;
    let generations: usize = 20;
    let mut grid = Grid::new(size);
    println!("{:#?}", grid);

    for generation in 0..generations {
        println!("Generation: {}", generation);
        grid.update();
        grid.render_console();
    }
}
