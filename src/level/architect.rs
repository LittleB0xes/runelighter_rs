  pub fn grid_generation() -> [i32; 350] {
    let width = 25;
    let height = 14;
    let mut grid: [i32; 350] = [0; 350];

    // First, place the wall
    for i in 0..width {
      for j in 0..height {
        if i == 0 || j == 0 || i == width - 1 || j == height - 1 {
          grid[i + j * width] = 1;
        }
      }
    }

    //for j in 0..height {
    //  for i in 0..width {
    //    print!("{}", grid[i + j * width]);
    //  }
    //  println!();
    //}
    grid
  }