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
        //Varibles
        //Static Str
        let title_str = String::from("Rusty Circuits");
        let header_buf = 2;
        let file_menu_str = "file";
        let sim_menu_str = "sim";
        let option_menu_str = "options";

        let info_title_str = "Info";
        let mode_cursor_str = "cursor";
        let mode_edit_str = "edit";
        let mode_visual_str = "visual";

        let hotkey_title_str = "hotkeys";
        let hotkey_next_str = "next";
        let hotkey_prev_str = "prev";

        //Values
        let open_file = "test file";
        let view_use_ratio = true;
        let view_ratio = (0.8, 0.7);
        let view_dim = (50, 50);

        let mut info_content = "This is some test text";
        let mut view_current_dim = (0,0);
        let mut view_current_mode = 0;

        let mut cursor_pos = (0,0);
        let mut map_pos = (0,0);
        let mut current_selected_pallet = 0;

        //Hotkeys
        let hotkey_next_key = 'm';
        let hotkey_prev_key = 'n';
        let hotkey_up_key = 'w';
        let hotkey_down_key = 's';
        let hotkey_left_key = 'a';
        let hotkey_right_key = 'd';
        let hotkey_edit_key = 'e';
        let hotkey_visual_key = 'v';
        let hotkey_back_key = 'q';

        //Fonts
        let tile_border_ew = '═';
        let tile_border_ns = '║';
        let tile_border_nw = '╝';
        let tile_border_ne = '╚';
        let tile_border_sw = '╗';
        let tile_border_se = '╔';
        let tile_border_nse = '╠';
        let tile_border_nsw = '╣';
        let tile_border_sew = '╦';
        let tile_border_new = '╩';
        let tile_border_nsew = '╬';


        let termsize = terminal::size();
        let mut dim = (0, 0);

        match termsize {

            Ok((col,row)) =>  dim = (col, row),
            Err(..) => {},
        }

        //println!("{}, {}", dim.0, dim.1);


        execute!(w, terminal::EnterAlternateScreen)?;

        terminal::enable_raw_mode()?;
        

        //Clear term
        queue!(
            w,
            style::SetBackgroundColor(style::Color::DarkGrey),
            style::SetForegroundColor(style::Color::Blue),
            terminal::Clear(terminal::ClearType::All),
            cursor::Show,
            cursor::MoveTo(0, 0)
        )?;

        //Drawborder
        queue!(w, cursor::MoveTo(0, 0) ,style::Print(tile_border_se))?;
        queue!(w, cursor::MoveTo(0, dim.1-1) ,style::Print(tile_border_ne))?;
        for x in 1..dim.0-1 {
            queue!(w, cursor::MoveTo(x,0), style::Print(tile_border_ew))?;
            queue!(w, cursor::MoveTo(x,2), style::Print(tile_border_ew))?;
            queue!(w, cursor::MoveTo(x,dim.1-1), style::Print(tile_border_ew))?;
        }
        queue!(w, cursor::MoveTo(dim.0-1, 0) ,style::Print(tile_border_sw))?;
        queue!(w, cursor::MoveTo(dim.0-1, dim.1-1) ,style::Print(tile_border_nw))?;
        for y in 1..dim.1-1 {
            queue!(w, cursor::MoveTo(0,y), style::Print(tile_border_ns))?;
            queue!(w, cursor::MoveTo(dim.0-1,y), style::Print(tile_border_ns))?;
        }
        queue!(w, cursor::MoveTo(dim.0-1, 2) ,style::Print(tile_border_nsw))?;
        queue!(w, cursor::MoveTo(0, 2) ,style::Print(tile_border_nse))?;
        

        //Draw heading
        queue!(w, cursor::MoveTo(header_buf+1,1), style::Print(title_str.clone()))?;
        queue!(w, cursor::MoveTo(header_buf*4+title_str.len() as u16,1), style::Print(open_file))?;

        queue!(w, cursor::MoveTo((dim.0-1)-header_buf, 1))?;
        queue!(w, cursor::MoveLeft(option_menu_str.len() as u16), style::Print(option_menu_str), cursor::MoveLeft(option_menu_str.len() as u16))?;
        queue!(w, cursor::MoveLeft(sim_menu_str.len() as u16 + header_buf*2), style::Print(sim_menu_str), cursor::MoveLeft(sim_menu_str.len() as u16))?;
        queue!(w, cursor::MoveLeft(file_menu_str.len() as u16 + header_buf*2), style::Print(file_menu_str), cursor::MoveLeft(file_menu_str.len() as u16))?;




        //Draw View Box
        let view_current_dim = ((dim.0 as f32 * view_ratio.0) as u16, (dim.1 as f32 * view_ratio.1) as u16);

        queue!(w, cursor::MoveTo(0,view_current_dim.1+3), style::Print(tile_border_nse))?;
        for _ in 0..view_current_dim.0 {
            queue!(w, style::Print(tile_border_ew))?;
        }
        queue!(w, style::Print(tile_border_new))?;
        for _ in 0..view_current_dim.1 {
            queue!(w, cursor::MoveLeft(1), cursor::MoveUp(1), style::Print(tile_border_ns))?;
        }
        queue!(w, cursor::MoveLeft(1), cursor::MoveUp(1), style::Print(tile_border_sew), cursor::MoveTo(1, 3))?;
        queue!(w, style::SetBackgroundColor(style::Color::Black))?;
        for _ in 0..view_current_dim.1 {
            for _ in 0..view_current_dim.0 {
                queue!(w, style::Print(" "))?;
            }
            queue!(w, cursor::MoveToNextLine(1), cursor::MoveRight(1))?;
        }


        queue!(w, cursor::MoveTo(cursor_pos.0+1, cursor_pos.1+3))?;

        w.flush()?;

        loop {
            

            

            match read_char()? {
                'q' => {
                    execute!(w, cursor::SetCursorStyle::DefaultUserShape).unwrap();
                    break;
                },
                'a' => {if cursor_pos.0 > 0 {cursor_pos.0 -=1 }},
                'd' => {if cursor_pos.0 < view_current_dim.0-1 {cursor_pos.0 +=1 }},
                'w' => {if cursor_pos.1 > 0 {cursor_pos.1 -=1 }},
                's' => {if cursor_pos.1 < view_current_dim.1-1 {cursor_pos.1 +=1 }},
                _ => {}
            }
            

            execute!(
                w,
                cursor::MoveTo(cursor_pos.0+1, cursor_pos.1+3),
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
