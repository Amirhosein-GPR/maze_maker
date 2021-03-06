pub mod maze_maker {
    use ggez::{graphics, Context, GameResult};
    use ggez::event::EventHandler;
    use rand::Rng;
    use std::fs::File;
    use std::io::{Read};

    #[derive(Clone)]
    struct Vector2D {
        x: f32,
        y: f32
    }

    impl Vector2D {
        pub fn new(x: f32, y: f32) -> Self {
            Self {
                x,
                y
            }
        }
    }

    impl Into<ggez::mint::Point2<f32>> for Vector2D {
        fn into(self) -> ggez::mint::Point2<f32> {
            ggez::mint::Point2::<f32> {
                x: self.x,
                y: self.y
            }
        }
    }

    enum Position {
        Right,
        Down
    }

    struct Edge {
        mesh: graphics::Mesh
    }

    impl Edge {
        fn new(mesh: graphics::Mesh) -> Self {
            Self {
                mesh
            }
        }
    }

    enum ComingFrom {
        Up,
        Right,
        Down,
        Left,
        CanNotMove
    }

    struct HeadNode {
        mesh: graphics::Mesh,
        taken_paths: Vec<[u8; 2]>,
        cell_width: f32,
        cell_height: f32,
        cell_location: [u8; 2],
        table: [u8; 2],
        game_not_finished: bool
    }

    impl HeadNode {
        fn new(mesh: graphics::Mesh, taken_paths: Vec<[u8; 2]>, cell_width: f32, cell_height: f32, cell_location: [u8; 2], table: [u8; 2], game_not_finished: bool) -> Self {
            Self {
                mesh,
                taken_paths,
                cell_width,
                cell_height,
                cell_location,
                table,
                game_not_finished
            }
        }

        fn move_by_one_cell_randomly(&mut self, context: &mut Context, cells: &mut Vec<Vec<Cell>>) -> ComingFrom {
            let mut coming_from: ComingFrom = ComingFrom::CanNotMove;
            let (mut try_up, mut try_right, mut try_down, mut try_left) : (bool, bool, bool, bool) = (true, true, true, true);
            loop {
                let random_number = rand::thread_rng().gen_range(0..4);
                if try_up || try_right || try_down || try_left {
                    match random_number {
                        0 => if self.cell_location[0] != 0 && cells.get(self.cell_location[1] as usize).unwrap().get((self.cell_location[0] - 1) as usize).unwrap().not_visited == true {
                            self.cell_location[0] -= 1;
                            self.mesh = self.recreate_circle_mesh(context, [0.5, 0.0, 0.0, 1.0]);
                            coming_from = ComingFrom::Down;
                            println!("PUSHING : {:?}", self.cell_location);
                            self.taken_paths.push([self.cell_location[0], self.cell_location[1]]);
                            cells.get_mut(self.cell_location[1] as usize).unwrap().get_mut((self.cell_location[0]) as usize).unwrap().not_visited = false;
                            break;
                        } else {
                            try_up = false;
                        },
                        1 => if self.cell_location[1] != self.table[1] - 1 && cells.get((self.cell_location[1] + 1) as usize).unwrap().get(self.cell_location[0] as usize).unwrap().not_visited == true {
                            self.cell_location[1] += 1;
                            self.mesh = self.recreate_circle_mesh(context, [0.5, 0.0, 0.0, 1.0]);
                            coming_from = ComingFrom::Left;
                            println!("PUSHING : {:?}", self.cell_location);
                            self.taken_paths.push([self.cell_location[0], self.cell_location[1]]);
                            cells.get_mut(self.cell_location[1] as usize).unwrap().get_mut((self.cell_location[0]) as usize).unwrap().not_visited = false;
                            break;
                        } else {
                            try_right = false;
                        },
                        2 => if self.cell_location[0] != self.table[0] - 1 && cells.get(self.cell_location[1] as usize).unwrap().get((self.cell_location[0] + 1) as usize).unwrap().not_visited == true {
                            self.cell_location[0] += 1;
                            self.mesh = self.recreate_circle_mesh(context, [0.5, 0.0, 0.0, 1.0]);
                            coming_from = ComingFrom::Up;
                            println!("PUSHING : {:?}", self.cell_location);
                            self.taken_paths.push([self.cell_location[0], self.cell_location[1]]);
                            cells.get_mut(self.cell_location[1] as usize).unwrap().get_mut((self.cell_location[0]) as usize).unwrap().not_visited = false;
                            break;
                        } else {
                            try_down = false;
                        },
                        3 => if self.cell_location[1] != 0 && cells.get((self.cell_location[1] - 1) as usize).unwrap().get(self.cell_location[0] as usize).unwrap().not_visited == true {
                            self.cell_location[1] -= 1;
                            self.mesh = self.recreate_circle_mesh(context, [0.5, 0.0, 0.0, 1.0]);
                            coming_from = ComingFrom::Right;
                            println!("PUSHING : {:?}", self.cell_location);
                            self.taken_paths.push([self.cell_location[0], self.cell_location[1]]);
                            cells.get_mut(self.cell_location[1] as usize).unwrap().get_mut((self.cell_location[0]) as usize).unwrap().not_visited = false;
                            break;
                        } else {
                            try_left = false;
                        },
                        _ => ()
                    }
                } else {
                    break;
                }
            }
            coming_from
        }

        fn move_back_by_one_cell(&mut self, context: &mut Context) {
            if self.taken_paths.len() == 0 {
                println!("FINISHED :D");
                self.game_not_finished = false;
                return;
            }
            let cell_location = self.taken_paths.pop().unwrap();
            println!("POPPING : {:?}", self.cell_location);

            self.cell_location[0] = cell_location[0];
            self.cell_location[1] = cell_location[1];
            self.mesh = self.recreate_circle_mesh(context, [0.5, 0.0, 0.0, 1.0]);
        }

        fn recreate_circle_mesh(&self, context: &mut Context, color: [f32; 4]) -> ggez::graphics::Mesh {
            graphics::Mesh::new_circle(
                context,
                graphics::DrawMode::stroke(2.0),
                Vector2D::new(self.cell_location[1] as f32 * self.cell_width + self.cell_width / 2.0, self.cell_location[0] as f32 * self.cell_height + self.cell_height / 2.0),
                6.0,
                0.5,
                graphics::Color::new(color[0], color[1], color[2], color[3])
            ).unwrap()
        }
    }

    struct Cell {
        mesh: graphics::Mesh,
        not_visited: bool
    }
    
    impl Cell {
        fn new(mesh: graphics::Mesh, not_visited: bool) -> Self {
            Self {
                mesh,
                not_visited
            }
        }
    }

    pub struct Game {
        cells: Vec<Vec<Cell>>,
        edges: Vec<Vec<Edge>>,
        head_node: HeadNode,
        current_milisec: u64,
        next_milisec: u64,
        foreground_color: [f32; 4],
        refresh_rate_in_miliseconds: u64
    }

    impl Game {
        pub fn new(context: &mut Context) -> Self {
            let mut settings = String::new();
            File::open("./settings.conf").expect("Error opening the settings.conf file").read_to_string(&mut settings).expect("Error reading to string");

            let settings_rows = settings.split('\n').collect::<Vec<&str>>();
            
            let table_size = settings_rows.get(0)
                .unwrap()
                .split(':').collect::<Vec<&str>>().get(1).unwrap().split('x').collect::<Vec<&str>>();
            
            let mut table: [u8; 2] = [0; 2];
            table[0] = table_size.get(0).unwrap().parse::<u8>().unwrap();
            table[1] = table_size.get(1).unwrap().parse::<u8>().unwrap();

            let background_color = settings_rows.get(1)
                .unwrap()
                .split(':')
                .collect::<Vec<&str>>()
                .get(1).unwrap().split(',').collect::<Vec<&str>>();
            let background_color: [f32; 4] = [background_color.get(0).unwrap().parse::<f32>().unwrap(), background_color.get(1).unwrap().parse::<f32>().unwrap(), background_color.get(2).unwrap().parse::<f32>().unwrap(), background_color.get(3).unwrap().parse::<f32>().unwrap()];

            let foreground_color = settings_rows.get(2)
                .unwrap()
                .split(':')
                .collect::<Vec<&str>>()
                .get(1).unwrap().split(',').collect::<Vec<&str>>();
            let foreground_color: [f32; 4] = [foreground_color.get(0).unwrap().parse::<f32>().unwrap(), foreground_color.get(1).unwrap().parse::<f32>().unwrap(), foreground_color.get(2).unwrap().parse::<f32>().unwrap(), foreground_color.get(3).unwrap().parse::<f32>().unwrap()];

            let refresh_rate_in_miliseconds = settings_rows
                .get(3)
                .unwrap()
                .split(':')
                .collect::<Vec<&str>>();
            let refresh_rate_in_miliseconds = refresh_rate_in_miliseconds.get(1).unwrap();
            let refresh_rate_in_miliseconds: u64 = refresh_rate_in_miliseconds.parse::<u64>().unwrap();

            let wait_then_solve_in_miliseconds = settings_rows
                .get(4)
                .unwrap()
                .split(':')
                .collect::<Vec<&str>>();
            let wait_then_solve_in_miliseconds = wait_then_solve_in_miliseconds.get(1).unwrap();
            let wait_then_solve_in_miliseconds: u64 = wait_then_solve_in_miliseconds.parse::<u64>().unwrap();

            let (mut cell_width, mut cell_height) = graphics::drawable_size(context);
            cell_width /= table[1] as f32;
            cell_height /= table[0] as f32;

            let taken_paths: Vec<[u8; 2]> = Vec::new();
            let mut cells: Vec<Vec<Cell>> = Vec::new();
            let mut edges: Vec<Vec<Edge>> = Vec::new();

            let mut cell_rect = graphics::Rect::new(0.0, 0.0, cell_width, cell_height);

            for i in 0..table[1] as usize {
                cells.push(Vec::new());
                edges.push(Vec::new());
                for j in 0..table[0] as usize {
                    let (x, y) = (i as f32 * cell_width, j as f32 * cell_height);
                    cell_rect.x = x;
                    cell_rect.y = y;
                    cells.get_mut(i).unwrap().push(Cell::new(
                        graphics::Mesh::new_rectangle(
                            context,
                            graphics::DrawMode::fill(),
                            cell_rect,
                            graphics::Color::new(background_color[0], background_color[1], background_color[2], background_color[3])
                        ).unwrap(),
                        true
                    ));
                    edges.get_mut(i).unwrap().push(Edge::new(
                        graphics::Mesh::new_line(
                            context,
                            &[Vector2D::new(x + cell_width, y), Vector2D::new(x + cell_width, y + cell_height)],
                            5.0,
                            graphics::Color::BLACK).unwrap()
                    ));
                    edges.get_mut(i).unwrap().push(Edge::new(
                        graphics::Mesh::new_line(
                            context,
                            &[Vector2D::new(x, y + cell_height), Vector2D::new(x + cell_width, y + cell_height)],
                            5.0,
                            graphics::Color::BLACK).unwrap()
                    ));
                }
            }

            let random_cell_index_1 = rand::thread_rng().gen_range(0..table[0]);
            let random_cell_index_2 = rand::thread_rng().gen_range(0..table[1]);

            let head_node = HeadNode::new(
                graphics::Mesh::new_circle(
                    context,
                    graphics::DrawMode::stroke(2.0),
                    Vector2D::new(random_cell_index_2 as f32 * cell_width + cell_width / 2.0, random_cell_index_1 as f32 * cell_height + cell_height / 2.0),
                    6.0,
                    0.5,
                    graphics::Color::new(0.5, 0.0, 0.0, 1.0)).unwrap(),
                taken_paths,
                cell_width,
                cell_height,
                [random_cell_index_1, random_cell_index_2],
                table,
                true
            );

            cell_rect.x = random_cell_index_2 as f32 * cell_width;
            cell_rect.y = random_cell_index_1 as f32 * cell_height;
            cells.get_mut(random_cell_index_2 as usize).unwrap().get_mut(random_cell_index_1 as usize).unwrap().mesh = graphics::Mesh::new_rectangle(
                context,
                graphics::DrawMode::fill(),
                cell_rect,
                graphics::Color::new(foreground_color[0], foreground_color[1], foreground_color[2], foreground_color[3])
            ).unwrap();
            cells.get_mut(random_cell_index_2 as usize).unwrap().get_mut(random_cell_index_1 as usize).unwrap().not_visited = false;

            Self {
                cells,
                edges,
                head_node,
                current_milisec: wait_then_solve_in_miliseconds,
                next_milisec: 0,
                foreground_color,
                refresh_rate_in_miliseconds
            }
        }

        fn draw_objects(&mut self, context: &mut Context) {
            for pointer_to_cell in self.cells.iter() {
                for cell in pointer_to_cell.iter() {
                    graphics::draw(context, &cell.mesh, graphics::DrawParam::default()).expect("Error in drawing meshes for cells")
                }
            }
            for pointer_to_edge in self.edges.iter() {
                for edge in pointer_to_edge.iter() {
                    graphics::draw(context, &edge.mesh, graphics::DrawParam::default()).expect("Error in drawing meshes for edges");
                }
            }
            graphics::draw(context, &self.head_node.mesh, graphics::DrawParam::default()).expect("Error in drawing meshe for head node");
        }

        fn recreate_rectangle_mesh(&self, context: &mut Context, color: [f32; 4], cell_location: [u8; 2]) -> ggez::graphics::Mesh {
            graphics::Mesh::new_rectangle(
                context,
                graphics::DrawMode::fill(),
                graphics::Rect::new(cell_location[1] as f32 * self.head_node.cell_width, cell_location[0] as f32 * self.head_node.cell_height, self.head_node.cell_width, self.head_node.cell_height),
                graphics::Color::new(color[0], color[1], color[2], color[3])
            ).unwrap()
        }

        fn recreate_line_mesh(&self, context: &mut Context, cell_location: [u8; 2], position: Position) -> ggez::graphics::Mesh {
            match position {
                Position::Right => {
                    graphics::Mesh::new_line(
                        context,
                        &[Vector2D::new(
                            cell_location[0] as f32 * self.head_node.cell_width + self.head_node.cell_width,
                            cell_location[1] as f32 * self.head_node.cell_height
                        ),
                        Vector2D::new(
                            cell_location[0] as f32 * self.head_node.cell_width + self.head_node.cell_width,
                            cell_location[1] as f32 * self.head_node.cell_height + self.head_node.cell_height
                        )],
                        5.0,
                        graphics::Color::new(self.foreground_color[0], self.foreground_color[1], self.foreground_color[2], self.foreground_color[3])
                    ).unwrap()
                },
                Position::Down => {
                    graphics::Mesh::new_line(
                        context,
                        &[Vector2D::new(
                            cell_location[0] as f32 * self.head_node.cell_width,
                            cell_location[1] as f32 * self.head_node.cell_height + self.head_node.cell_height
                        ),
                        Vector2D::new(
                            cell_location[0] as f32 * self.head_node.cell_width + self.head_node.cell_width,
                            cell_location[1] as f32 * self.head_node.cell_height + self.head_node.cell_height
                        )],
                        5.0,
                        graphics::Color::new(self.foreground_color[0], self.foreground_color[1], self.foreground_color[2], self.foreground_color[3])
                    ).unwrap()
                }
            }
        }

        fn remove_edge_and_light_up_cell(&mut self, context : &mut Context, coming_from: ComingFrom) {
            let mut previous_cell_location: [u8; 2] = [0; 2];
            match coming_from {
                ComingFrom::Up => {
                    previous_cell_location[0] = self.head_node.cell_location[0] - 1;
                    previous_cell_location[1] = self.head_node.cell_location[1];
                    let equvalent_cell_x = previous_cell_location[1];
                    let equvalent_cell_y = previous_cell_location[0] * 2 + 1;
                    self.edges.get_mut(equvalent_cell_x as usize)
                        .unwrap()
                        .get_mut(equvalent_cell_y as usize)
                        .unwrap()
                        .mesh = self.recreate_line_mesh(context, [equvalent_cell_x, equvalent_cell_y], Position::Down);
                    self.cells.get_mut(previous_cell_location[1] as usize).unwrap().get_mut(previous_cell_location[0] as usize).unwrap().not_visited = false;
                },
                ComingFrom::Right => {
                    previous_cell_location[0] = self.head_node.cell_location[0];
                    previous_cell_location[1] = self.head_node.cell_location[1];
                    let equvalent_cell_x = previous_cell_location[1];
                    let equvalent_cell_y = previous_cell_location[0] * 2;
                    self.edges.get_mut(equvalent_cell_x as usize)
                        .unwrap()
                        .get_mut(equvalent_cell_y as usize)
                        .unwrap()
                        .mesh = self.recreate_line_mesh(context, [equvalent_cell_x, equvalent_cell_y], Position::Right);
                    self.cells.get_mut(previous_cell_location[1] as usize).unwrap().get_mut(previous_cell_location[0] as usize).unwrap().not_visited = false;
                },
                ComingFrom::Down => {
                    previous_cell_location[0] = self.head_node.cell_location[0];
                    previous_cell_location[1] = self.head_node.cell_location[1];
                    let equvalent_cell_x = previous_cell_location[1];
                    let equvalent_cell_y = previous_cell_location[0] * 2 + 1;
                    self.edges.get_mut(equvalent_cell_x as usize)
                        .unwrap()
                        .get_mut(equvalent_cell_y as usize)
                        .unwrap()
                        .mesh = self.recreate_line_mesh(context, [equvalent_cell_x, equvalent_cell_y], Position::Down);
                    self.cells.get_mut(previous_cell_location[1] as usize).unwrap().get_mut(previous_cell_location[0] as usize).unwrap().not_visited = false;
                },
                ComingFrom::Left => {
                    previous_cell_location[0] = self.head_node.cell_location[0];
                    previous_cell_location[1] = self.head_node.cell_location[1] - 1;
                    let equvalent_cell_x = previous_cell_location[1];
                    let equvalent_cell_y = previous_cell_location[0] * 2;
                    self.edges.get_mut(equvalent_cell_x as usize)
                        .unwrap()
                        .get_mut(equvalent_cell_y as usize)
                        .unwrap()
                        .mesh = self.recreate_line_mesh(context, [equvalent_cell_x, equvalent_cell_y], Position::Right);
                    self.cells.get_mut(previous_cell_location[1] as usize).unwrap().get_mut(previous_cell_location[0] as usize).unwrap().not_visited = false;
                },
                ComingFrom::CanNotMove => {
                    self.head_node.move_back_by_one_cell(context);
                }
            }
            self.cells.get_mut(self.head_node.cell_location[1] as usize)
                .unwrap()
                .get_mut(self.head_node.cell_location[0] as usize)
                .unwrap()
                .mesh = self.recreate_rectangle_mesh(context, [self.foreground_color[0], self.foreground_color[1], self.foreground_color[2], self.foreground_color[3]], self.head_node.cell_location);
        }

        fn update_objects(&mut self, context: &mut Context) {
            if self.head_node.game_not_finished {
                self.next_milisec = ggez::timer::time_since_start(context).as_millis() as u64;
                if self.current_milisec < self.next_milisec {
                    self.current_milisec = self.next_milisec + self.refresh_rate_in_miliseconds;
                    let coming_from = self.head_node.move_by_one_cell_randomly(context, &mut self.cells);
                    self.remove_edge_and_light_up_cell(
                        context,
                        coming_from
                    );
                }
            }
        }
    }

    impl EventHandler for Game {
        fn update(&mut self, context: &mut Context) -> GameResult<()> {
            self.update_objects(context);
            Ok(())
        }

        fn draw(&mut self, context: &mut Context) -> GameResult<()> {
            graphics::clear(context, graphics::Color::WHITE);

            self.draw_objects(context);

            graphics::present(context)
        }
    }
}