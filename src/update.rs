// update

use crate::{msg::Message, model::Model};

const LEFT:  char = 'e';
const DOWN:  char = 'i';
const UP:    char = 'o';
const RIGHT: char = 'n';
const QUIT:  char = 'q';

// return new model based on old model and message
pub fn update<'a>(model: Model<'a>, msg: Message) -> Model<'a>
{
    let mut m = model.clone();

    match msg 
    {
        Message::Resize(x, y) => m.resize((x, y)),

        Message::Code(c) => {
            match c {
                UP   => m.move_cursor_up(),
                DOWN => m.move_cursor_down(),
                QUIT => m.quit = true,
                _ => {}
            }
        }

        _ => {}
    }
    m
}
