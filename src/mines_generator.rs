use rand::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum BlockState {
    Safe(usize),
    Unsafe,
}

pub struct MinesManager {
    pub lines: usize,
    pub columns: usize,
    pub mines_count: usize,
    pub blocks_map: Vec<Vec<BlockState>>
}

impl MinesManager {
    pub fn new(lines: usize, columns: usize, mines_count: usize) -> Self {
        let blocks_count = lines * columns;
        if blocks_count <= mines_count {
            panic!("blocks_count <= mines_count");
        }

        let mut blocks_map: Vec<Vec<BlockState>> = vec![vec![BlockState::Safe(0); columns]; lines];
        Self::fill_unsafe_fields(lines, columns, mines_count, &mut blocks_map);
        Self::fill_safe_fields(&mut blocks_map, lines as i32, columns as i32);

        MinesManager {
            lines,
            columns,
            mines_count,
            blocks_map
        }
    }

    fn fill_unsafe_fields(lines: usize, columns: usize, mines_count: usize, blocks_map: &mut Vec<Vec<BlockState>>) {
        let mut mines: usize = mines_count;
        let mut rng = thread_rng();
        while mines > 0 {
            let random_line = rng.gen_range(0..lines);
            let random_column = rng.gen_range(0..columns);

            match blocks_map[random_line][random_column] {
                BlockState::Safe(_) => { blocks_map[random_line][random_column] = BlockState::Unsafe },
                BlockState::Unsafe => { continue }
            }
            mines -= 1;
        }
    }

    fn fill_safe_fields(blocks_map: &mut Vec<Vec<BlockState>>, lines: i32, columns: i32) {
        for line in 0..lines {
            for column in 0..columns {
                 if blocks_map[line as usize][column as usize] == BlockState::Unsafe {
                     // Preenche os itens da linha anterior
                     if line - 1 >= 0 && column - 1 >= 0 && blocks_map[line as usize - 1][column as usize - 1] != BlockState::Unsafe {
                         Self::increment(&mut blocks_map[line as usize - 1][column as usize - 1]);
                     }
                     if line - 1 >= 0 && blocks_map[line as usize - 1][column as usize] != BlockState::Unsafe {
                         Self::increment(&mut blocks_map[line as usize - 1][column as usize]);
                     }
                     if line - 1 >= 0 && column + 1 < lines && blocks_map[line as usize - 1][column as usize + 1] != BlockState::Unsafe {
                         Self::increment(&mut blocks_map[line as usize - 1][column as usize + 1]);
                     }

                     // Preenche os itens da linha atual
                     if column - 1 >= 0 && blocks_map[line as usize][column as usize - 1] != BlockState::Unsafe {
                         Self::increment(&mut blocks_map[line as usize][column as usize - 1]);
                     }
                     if column + 1 < lines && blocks_map[line as usize][column as usize + 1] != BlockState::Unsafe {
                         Self::increment(&mut blocks_map[line as usize][column as usize + 1]);
                     }

                     // Preenche os itens da próxima linha
                     if line + 1 < lines && column - 1 >= 0 && blocks_map[line as usize + 1][column as usize - 1] != BlockState::Unsafe {
                         Self::increment(&mut blocks_map[line as usize + 1][column as usize - 1]);
                     }
                     if line + 1 < lines && blocks_map[line as usize + 1][column as usize] != BlockState::Unsafe {
                         Self::increment(&mut blocks_map[line as usize + 1][column as usize]);
                     }
                     if line + 1 < lines && column + 1 < columns && blocks_map[line as usize + 1][column as usize + 1] != BlockState::Unsafe {
                         Self::increment(&mut blocks_map[line as usize + 1][column as usize + 1]);
                     }
                 }
            }
        }
    }

    fn increment(block: &mut BlockState) {
        if let BlockState::Safe(ref mut value) = block {
            *value += 1;
        }
    }

    pub fn print_map(&self) {
        for line in 0..self.lines {
            for column in 0..self.columns {
                match self.blocks_map[line as usize][column as usize].clone() {
                    BlockState::Safe(count) => {
                        print!("{} ", count);
                    }
                    BlockState::Unsafe => {
                        print!("X ");
                    }
                }
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mines_generator::*;

    #[test]
    fn test_mines_map() {
        let lines = 10;
        let columns = 10;
        let mines_quantity = 10;
        let map = MinesManager::new(lines, columns, mines_quantity);
        assert_eq!(map.blocks_map.len(), lines);
        assert_eq!(map.blocks_map[0].len(), columns);
        assert_eq!(map.lines, lines);
        assert_eq!(map.columns, columns);
        assert_eq!(map.mines_count, mines_quantity);
    }

    #[test]
    #[should_panic]
    fn test_safe_mines_map() {
        std::panic::set_hook(Box::new(|_| {}));

        let lines = 5;
        let columns = 3;
        let mines_quantity = lines * columns;
        MinesManager::new(lines, columns, mines_quantity);
    }
}