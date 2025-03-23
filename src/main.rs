use std::io::Cursor;

use iocraft::prelude::*;

/*
* Basically two modes:
*   Edit
*   Explore
* Use Ctrl+E to swap between the list and the editor
* 
* Tree
*   App:
*       NoteEditor:
*           TextInput
*           Controls
*       NoteExplorer:
*           NoteList:
*               Note, Note, etc
*           Controls
*/

fn main() {
    smol::block_on(element!(App).render_loop()).unwrap();
    //element!(Lilview).render_loop().await.unwrap();
}

#[component]
fn App(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let (width, height) = hooks.use_terminal_size();
    let mut should_exit = hooks.use_state(|| false);
    let mut system = hooks.use_context_mut::<SystemContext>();
    //hooks.use_state(initial_value);

    hooks.use_terminal_events({
        move |event| match event {
            TerminalEvent::Key(KeyEvent { modifiers, code, kind, .. }) if kind != KeyEventKind::Release => {
                match (modifiers, code) {
                    (KeyModifiers::CONTROL, KeyCode::Char('q')) => should_exit.set(true),
                    _ => {}
                }
            }
            _ => {}
        }
    });

    if should_exit.get() {
        system.exit();
    }

    element! {
        View(
            width: width - 1,
            height: height - 1,
            //background_color: Color::DarkGrey,
            border_style: BorderStyle::Single,
            border_color: Color::Grey,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
        ){
            NoteEditor()
            Footer()
        }
    }
}

#[component]
fn AppBody(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    element! {
        View(
            flex_grow: 1.0,
            flex_direction: FlexDirection::Row,
        ){
            // shows editor unless ctrl+n opens note list
            NoteExplorer()
            NoteEditor()
        }
    }
}

#[component]
fn NoteEditor(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let mut note_text = hooks.use_state(|| "".to_string());    
    let mut cursor_pos = hooks.use_state(|| 0);

    hooks.use_terminal_events({
        move |event| match event {
            TerminalEvent::Key(KeyEvent { code, kind, .. }) if kind != KeyEventKind::Release => {
                match code {
                    KeyCode::Char(c) => {
                        let mut s = note_text.to_string();
                        s.insert(cursor_pos.get(), c);
                        note_text.set(s);

                        cursor_pos.set(move_cursor_forward(cursor_pos.get(), note_text.to_string().len()))
                    }
                    KeyCode::Enter => {
                        let mut s = note_text.to_string();
                        s.insert(s.len(), '\n');
                        note_text.set(s);

                        cursor_pos.set(move_cursor_forward(cursor_pos.get(), note_text.to_string().len()))
                    }
                    KeyCode::Backspace => {
                        if cursor_pos.get() == 0 {return}

                        let mut s = note_text.to_string();
                        s.remove(cursor_pos.get() - 1);
                        note_text.set(s);

                        cursor_pos.set(move_cursor_back(cursor_pos.get(), note_text.to_string().len()));
                    }
                    KeyCode::Left => {
                        cursor_pos.set(move_cursor_back(cursor_pos.get(), note_text.to_string().len()));
                    }
                    KeyCode::Right => {
                        cursor_pos.set(move_cursor_forward(cursor_pos.get(), note_text.to_string().len()));
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    });

    let char_at_cursor = note_text.to_string().chars().nth(cursor_pos.get());
    let mut cursor_char: char = ' ';
    if char_at_cursor.is_some() {
        cursor_char = char_at_cursor.unwrap()
    }

    element! {
        View(
            width: 100pct,
            height: 100pct,
            flex_wrap: FlexWrap::Wrap,
            flex_direction: FlexDirection::Column,
        ){
            View(
                flex_grow: 1.0
            ) {
                Text(content: note_text.to_string())
            }
            
            //TextInput(
            //    has_focus: true,
            //    value: note_text.to_string(),
            //    on_change: move |new_value| note_text.set(new_value),
            //)
            View(
                height: 1,
            ){
                Text(content: format!("Cursor position: {}", cursor_pos.get()))
            }
            View(
                position: Position::Absolute,
                width: 1,
                height: 1,
                left: Inset::Length(cursor_pos.get().try_into().unwrap()),
                background_color: Color::Grey,
            ){
                Text(
                    content: format!("{}", cursor_char),
                    color: Color::Black,
                )
            }
        }
    }
}

fn move_cursor_back(cursor_pos: usize, text_length: usize) -> usize {
    if cursor_pos == 0 {return 0}
    cursor_pos - 1
}

fn move_cursor_forward(cursor_pos: usize, text_length: usize) -> usize {
    if cursor_pos >= text_length {return text_length}
    cursor_pos + 1
}

fn move_cursor_up(cursor_pos: usize, text: String) -> usize {
    // check for the last newline? this will be shenanigans
    // goto beggining of file if at top nl
    cursor_pos
}

fn move_cursor_down(cursor_pos: usize, text: String) -> usize {
    // check for next newline, else goto eof
    cursor_pos
}


#[component]
fn NoteExplorer(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    element! {
        View(
            flex_grow: 0.3,
        ){

        }
    }
}

#[component]
fn Footer(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    element! {
        View(
            height: 1,
            border_style: BorderStyle::None,
            border_color: Color::White,
            background_color: Color::DarkGrey,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Start,
            gap: 3,
        ) {
            Text(
                content: "Ctrl Q: Quit",
            )
            //Text(content: "Ctrl E: Explore") // or Edit if in explorer
        }
    }
}
