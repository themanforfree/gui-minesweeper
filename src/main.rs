use gamie::minesweeper::{Minesweeper, Status};
use rand::rngs::ThreadRng;
use slint::VecModel;
use std::rc::Rc;

slint::include_modules!();

struct Board(Vec<BoxData>);

impl<R> From<Minesweeper<R>> for Board
where
    R: rand::Rng,
{
    fn from(game: Minesweeper<R>) -> Self {
        let mut board = Vec::new();
        for row in 0..game.get_height() {
            for col in 0..game.get_width() {
                let cell = game.get(row, col);
                board.push(BoxData {
                    is_flagged: cell.is_flagged,
                    is_mine: cell.is_mine,
                    is_revealed: cell.is_revealed,
                    mine_adjacent: cell.mine_adjacent as i32,
                });
            }
        }

        Board(board)
    }
}

fn main() {
    let app = AppWindow::new();
    let mut game = Minesweeper::new(8, 8, 9, ThreadRng::default()).unwrap();
    let board = Board::from(game.clone());
    let board_model = Rc::new(VecModel::from(board.0));
    app.set_boxes(board_model.into());

    let app_week = app.as_weak();
    app.on_click(move |r, c| {
        if let Ok(true) = game.click(r as usize, c as usize, true) {
            let board = Board::from(game.clone());
            let board_model = Rc::new(VecModel::from(board.0));
            let app = app_week.unwrap();
            app.set_boxes(board_model.into());
            if game.is_ended() {
                app.set_is_ended(true);
                match game.get_game_status() {
                    Status::Win => println!("You win!"),
                    Status::Exploded(v) => {
                        println!("You exploded at ({}, {})", v[0].0, v[0].1);
                        return true;
                    }
                    _ => unreachable!(),
                }
            }
        }
        false
    });

    app.run();
}
