// msg

use crossterm::event::{Event, KeyEvent, KeyEventKind, KeyCode};

#[derive(Clone, PartialEq, Debug)]
pub enum Message 
{
    Code(char),
    Resize(u16, u16),
    Enter,
    Escape,
    Stop,
}

impl Message 
{
    // return a Message for a relevant event
    pub fn from_event(event: Event) -> Option<Message> 
    {
        match event 
        {
            Event::Key(keyevent) => Self::from_key_event(keyevent),

            Event::Resize(y, x)  => Some(Message::Resize(y, x)),

            _                    => None
        }
    }

    fn from_key_event(keyevent: KeyEvent) -> Option<Message> 
    {
        match keyevent {
            KeyEvent {
                code: KeyCode::Char(c),
                kind: KeyEventKind::Press,
                ..
            } => {
                Some(Message::Code(c))
            }
            _ => 
                None
        }
    }
}
