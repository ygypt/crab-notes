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

    hooks.use_terminal_events({
        move |event| match event {
            TerminalEvent::Key(KeyEvent { code, kind, .. }) if kind != KeyEventKind::Release => {
                if code == KeyCode::Enter {
                    let mut ntstring = note_text.to_string();
                    ntstring.insert(ntstring.len(), '\n');
                    note_text.set(ntstring);
                }
            }
            _ => {}
        }
    });

    element! {
        View(
            width: 100pct,
            height: 100pct,
            flex_wrap: FlexWrap::Wrap
        ){
            TextInput(
                has_focus: true,
                value: note_text.to_string(),
                on_change: move |new_value| note_text.set(new_value),
            )
        }
    }
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
            Text(content: "Ctrl Q: Quit")
            //Text(content: "Ctrl E: Explore") // or Edit if in explorer
        }
    }
}
