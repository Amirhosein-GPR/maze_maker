pub mod maze_maker {
    use ggez::{graphics, Context, GameResult};
    use ggez::event::EventHandler;

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
        mesh: graphics::Mesh,
        location: Position
    }

    impl Edge {
        fn new(mesh: graphics::Mesh, location: Position) -> Self {
            Self {
                mesh,
                location
            }
        }
    }

    struct Node {
        mesh: graphics::Mesh,
        location: Vector2D
    }

    impl Node {
        fn new(mesh: graphics::Mesh, location: Vector2D) -> Self {
            Self {
                mesh,
                location
            }
        }
    }

    pub struct Game {
        edges: Vec<Vec<Edge>>,
        nodes: Vec<Vec<Node>>
    }

    impl Game {
        pub fn new(context: &mut Context) -> Self {
            let (mut cell_width, mut cell_height) = graphics::drawable_size(context);
            cell_width /= 20.0;
            cell_height /= 20.0;

            let mut edges: Vec<Vec<Edge>> = Vec::new();
            let mut nodes: Vec<Vec<Node>> = Vec::new();

            for i in 0..20 {
                edges.push(Vec::new());
                nodes.push(Vec::new());
                for j in 0..20 {
                    let (x, y) = ((i * cell_width as usize) as f32, (j * cell_height as usize) as f32);
                    edges.get_mut(i).unwrap().push(Edge::new(
                        graphics::Mesh::new_line(
                            context,
                            &[Vector2D::new(x + cell_width, y), Vector2D::new(x + cell_width, y + cell_height)],
                            5.0,
                            graphics::Color::BLACK).unwrap(),
                        Position::Right
                    ));
                    edges.get_mut(i).unwrap().push(Edge::new(
                        graphics::Mesh::new_line(
                            context,
                            &[Vector2D::new(x, y + cell_height), Vector2D::new(x + cell_width, y + cell_height)],
                            5.0,
                            graphics::Color::BLACK).unwrap(),
                        Position::Down
                    ));
                    nodes.get_mut(i).unwrap().push(Node::new(
                        graphics::Mesh::new_circle(
                            context,
                            graphics::DrawMode::stroke(2.0),
                            Vector2D::new(x as f32 + cell_width / 2.0, y as f32 + cell_height / 2.0),
                            6.0,
                            0.5,
                            graphics::Color::new(0xf8 as f32, 0x00 as f32, 0x00 as f32, 0xff as f32)).unwrap(),
                        Vector2D::new(x as f32 + cell_width / 2.0, y as f32 + cell_height / 2.0)
                    ));
                }
            }
            Self {
                edges,
                nodes
            }
        }

        fn draw_objects(&mut self, context: &mut Context) {
            for i in 0..20 {
                for j in 0..20 {
                    graphics::draw(context, &self.edges.get(i).unwrap().get(j).unwrap().mesh, graphics::DrawParam::default()).expect("Error in drawing meshes");
                    graphics::draw(context, &self.edges.get(i).unwrap().get(j + 20).unwrap().mesh, graphics::DrawParam::default()).expect("Error in drawing meshes");
                    graphics::draw(context, &self.nodes.get(i).unwrap().get(j).unwrap().mesh, graphics::DrawParam::default()).expect("Error in drawing meshes");
                }
            }
        }

        fn update_edges(&self) {

        }
    }

    impl EventHandler for Game {
        fn update(&mut self, context: &mut Context) -> GameResult<()> {
            Ok(())
        }

        fn draw(&mut self, context: &mut Context) -> GameResult<()> {
            graphics::clear(context, graphics::Color::WHITE);

            self.draw_objects(context);

            graphics::present(context)
        }
    }
}