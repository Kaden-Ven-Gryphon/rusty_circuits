use std::io;

use crossterm::event::KeyEventKind;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    queue,
    style::{self, style},
    terminal,
    ExecutableCommand
};



fn run<W>(w: &mut W) -> io::Result<()>
    where 
        W: io::Write,
    {
        let termsize = terminal::size();
        let mut dim = (0, 0);

        match termsize {

            Ok((col,row)) =>  dim = (col, row),
            Err(..) => {},
        }

        println!("{}, {}", dim.0, dim.1);


        execute!(w, terminal::EnterAlternateScreen)?;

        terminal::enable_raw_mode()?;
        let mut c = '#';

        queue!(
            w,
            style::ResetColor,
            terminal::Clear(terminal::ClearType::All),
            cursor::Show,
            cursor::MoveTo(1, 1)
        )?;

        let mut cursor_pos = (0,0);

        for x in 1..=dim.0 {
            for y in 1..=dim.1 {
                queue!(w, style::Print(c), cursor::MoveTo(x, y))?;
            }
        }
        w.flush()?;

        loop {
            

            

            match read_char()? {
                'q' => {
                    execute!(w, cursor::SetCursorStyle::DefaultUserShape).unwrap();
                    break;
                },
                'a' => {cursor_pos.0 -=1 },
                'd' => {cursor_pos.0 +=1 },
                'w' => {cursor_pos.1 -=1 },
                's' => {cursor_pos.1 +=1 },
                _ => {}
            }

            execute!(
                w,
                cursor::MoveTo(cursor_pos.0, cursor_pos.1),
            )?;
        }

        execute!(
            w,
            style::ResetColor,
            cursor::Show,
            terminal::LeaveAlternateScreen
        )?;

        terminal::disable_raw_mode()
    }

pub fn read_char() -> std::io::Result<char> {
    loop {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            kind: KeyEventKind::Press,
            modifiers: _,
            state: _,
        })) = event::read()
        {
            return Ok(c);
        }
    }
}


fn main() -> std::io::Result<()>{
    let mut stdout = io::stdout();
    run(&mut stdout)    
}
