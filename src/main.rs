use iocraft::prelude::*;

fn main() {
    smol::block_on(element!(Mainview).render_loop()).unwrap();
    //element!(Lilview).render_loop().await.unwrap();
}

#[component]
fn Mainview(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let (width, height) = hooks.use_terminal_size();
    let mut should_exit = hooks.use_state(|| false);
    let mut system = hooks.use_context_mut::<SystemContext>();

    hooks.use_terminal_events({
        move |event| match event {
            TerminalEvent::Key(KeyEvent { code, kind, .. }) if kind != KeyEventKind::Release => {
                match code {
                    KeyCode::Char('q') => should_exit.set(true),
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
            height,
            background_color: Color::DarkGrey,
            border_style: BorderStyle::Single,
            border_color: Color::Grey,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
        ){
            Lilview()
            Text(content: "Press q to quit")
        }
    }
}

#[component]
fn Lilview(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    element! {
        View(
            border_style: BorderStyle::Round,
            border_color: Color::Blue,
        ) {
            Text(content: "Hello, world!")
        }
    }
}
