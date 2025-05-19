#![deny(clippy::all)]

use once_cell::sync::Lazy;
use parser::parse;
use reedline::{
    ColumnarMenu, DefaultCompleter, DefaultPrompt, DefaultPromptSegment, Emacs, ExampleHighlighter,
    KeyCode, KeyModifiers, MenuBuilder, Reedline, ReedlineEvent, ReedlineMenu, Signal,
    default_emacs_keybindings,
};
use std::io::Result;

static KEYWORDS: Lazy<Vec<String>> = Lazy::new(|| vec!["fn".into(), "let".into()]);

fn create_line_editor() -> Reedline {
    let highlighter = Box::new(ExampleHighlighter::new(KEYWORDS.clone()));
    let completer = Box::new(DefaultCompleter::new_with_wordlen(KEYWORDS.clone(), 2));
    let completion_menu = Box::new(ColumnarMenu::default().with_name("completion_menu"));
    let mut keybindings = default_emacs_keybindings();
    keybindings.add_binding(
        KeyModifiers::NONE,
        KeyCode::Tab,
        ReedlineEvent::UntilFound(vec![
            ReedlineEvent::Menu("completion_menu".to_string()),
            ReedlineEvent::MenuNext,
        ]),
    );

    let edit_mode = Box::new(Emacs::new(keybindings));
    Reedline::create()
        .with_highlighter(highlighter)
        .with_completer(completer)
        .with_menu(ReedlineMenu::EngineCompleter(completion_menu))
        .with_edit_mode(edit_mode)
}

fn handle_input(input: &str) {
    let parse = parse(input);
    println!("{}", parse.debug_tree());

    let syntax = parse.syntax();
    for error in ast::validation::validate(&syntax) {
        println!("{error}");
    }

    let root = ast::Root::cast(syntax).unwrap();
    let var_defs: Vec<_> = root
        .stmts()
        .filter_map(|stmt| match stmt {
            ast::Stmt::VariableDef(def) => Some(def),
            _ => None,
        })
        .collect();

    dbg!(var_defs);
    dbg!(hir::lower(&root));
}

fn repl(mut line_editor: Reedline, prompt: &DefaultPrompt) -> Result<()> {
    loop {
        match line_editor.read_line(prompt) {
            Ok(Signal::Success(input)) => handle_input(&input),
            Ok(Signal::CtrlC) => continue,
            Ok(Signal::CtrlD) => break,
            event => println!("Event: {event:?}"),
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let line_editor = create_line_editor();
    let prompt = DefaultPrompt::new(DefaultPromptSegment::Empty, DefaultPromptSegment::Empty);
    repl(line_editor, &prompt)
}
