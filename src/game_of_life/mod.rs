use std::collections::HashMap;
use std::collections::HashSet;
use rand::Rng;

pub struct Game {
    size: u32,
    pub living_cells: HashSet<(u32, u32)>,
    state: HashMap<(u32, u32), u8>,
    cell_size: f32,
}

impl Game {
    pub fn init(size: u32, initial_values: Vec<(u32, u32)>) -> Game {
        let mut initial_cells = HashSet::new();

        let mut rng = rand::thread_rng();
        
        for v in initial_values {
            initial_cells.insert((v.0, v.1));
        }

        for i in 0..size {
            for j in 0..size {

                if rng.gen_range(0..10) < 6 {
                    initial_cells.insert((i, j));
                }
            }
        }

        initial_cells.insert((4, 4));
        initial_cells.insert((4, 5));
        initial_cells.insert((4, 6));
        initial_cells.insert((3, 6));
        initial_cells.insert((2, 5));

        let t = size as f32;
        let cs = 2.0 / t;

        Game {
            size: size,
            living_cells: initial_cells,
            state: HashMap::new(),
            cell_size: cs,
        }
    }

    pub fn update(&mut self) {
        self.state.clear();

        for &value in self.living_cells.iter() {
            let left = self.sub_one(value.0);
            let center = value.0;
            let right = self.add_one(value.0);
            let top = self.sub_one(value.1);
            let middle = value.1;
            let bottom = self.add_one(value.1);

            let lt = (left, top);
            let ct = (center, top);
            let rt = (right, top);

            let lm = (left, middle);
            let rm = (right, middle);

            let lb = (left, bottom);
            let cb = (center, bottom);
            let rb = (right, bottom);

            *self.state.entry(lt).or_insert(0) += 1;
            *self.state.entry(ct).or_insert(0) += 1;
            *self.state.entry(rt).or_insert(0) += 1;
            *self.state.entry(lm).or_insert(0) += 1;
            *self.state.entry(rm).or_insert(0) += 1;
            *self.state.entry(lb).or_insert(0) += 1;
            *self.state.entry(cb).or_insert(0) += 1;
            *self.state.entry(rb).or_insert(0) += 1;
        }

        for (key, value) in self.state.iter() {
            if self.living_cells.contains(key) {
                if value < &2 || value > &3 {
                    self.living_cells.remove(key);
                }
            } else {
                if value == &3 {
                    self.living_cells.insert(*key);
                }
            }
        }
    }

    pub fn render(&self, vertices: &mut Vec<f32>) {
        vertices.clear();

        for cell in self.living_cells.iter() {
            let x1 = (cell.0 as f32 * self.cell_size) - 1.0;
            let x2 = x1 + self.cell_size;
            let y1 = (cell.1 as f32 * self.cell_size) - 1.0;
            let y2 = y1 + self.cell_size;

            vertices.push(x1);
            vertices.push(y1);
            vertices.push(0.0);

            vertices.push(x2);
            vertices.push(y2);
            vertices.push(0.0);
            vertices.push(x2 as f32);
            vertices.push(y1 as f32);
            vertices.push(0.0);
            vertices.push(x1 as f32);
            vertices.push(y1 as f32);
            vertices.push(0.0);

            vertices.push(x2);
            vertices.push(y2);
            vertices.push(0.0);
            vertices.push(x1);
            vertices.push(y2);
            vertices.push(0.0);
        }
    }

    fn sub_one(&self, value: u32) -> u32 {
        let result;
        if value == 0 {
            result = self.size - 1;
        } else {
            result = value - 1;
        }

        return result;
    }

    fn add_one(&self, value: u32) -> u32 {
        let result;
        if value == self.size - 1 {
            result = 0;
        } else {
            result = value + 1;
        }

        return result;
    }
}
