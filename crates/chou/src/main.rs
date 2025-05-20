#![deny(clippy::all)]

use parser::parse;
use reedline::{
    ColumnarMenu, DefaultCompleter, DefaultHinter, Emacs, ExampleHighlighter, KeyCode,
    KeyModifiers, MenuBuilder, Prompt, PromptEditMode, PromptHistorySearch,
    PromptHistorySearchStatus, Reedline, ReedlineEvent, ReedlineMenu, Signal,
    default_emacs_keybindings,
};
use std::borrow::Cow;
use std::io::Result;

pub static DEFAULT_PROMPT_INDICATOR: &str = ">>> ";
pub static DEFAULT_MULTILINE_INDICATOR: &str = "... ";

#[derive(Clone, Copy, Debug)]
pub struct ChouPrompt;

impl Prompt for ChouPrompt {
    fn render_prompt_left(&self) -> Cow<str> {
        Cow::Borrowed("")
    }

    fn render_prompt_right(&self) -> Cow<str> {
        Cow::Borrowed("")
    }

    fn render_prompt_indicator(&self, _edit_mode: PromptEditMode) -> Cow<str> {
        Cow::Borrowed(DEFAULT_PROMPT_INDICATOR)
    }

    fn render_prompt_multiline_indicator(&self) -> Cow<str> {
        Cow::Borrowed(DEFAULT_MULTILINE_INDICATOR)
    }

    fn render_prompt_history_search_indicator(
        &self,
        history_search: PromptHistorySearch,
    ) -> Cow<str> {
        let prefix = match history_search.status {
            PromptHistorySearchStatus::Passing => "",
            PromptHistorySearchStatus::Failing => "failing ",
        };

        Cow::Owned(format!(
            "({}reverse-search: {}) ",
            prefix, history_search.term
        ))
    }
}

struct ChouRepl {
    editor: Reedline,
    prompt: ChouPrompt,
}

impl ChouRepl {
    fn new() -> Self {
        let editor = Self::build_editor();
        let prompt = ChouPrompt;
        Self { editor, prompt }
    }

    fn build_editor() -> Reedline {
        let keywords = vec!["fn".to_string(), "let".to_string()];
        let hinter = Box::new(DefaultHinter::default());
        let completer = Box::new(DefaultCompleter::new_with_wordlen(keywords.clone(), 2));
        let highlighter = Box::new(ExampleHighlighter::new(keywords.clone()));

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
        let completion_menu = Box::new(ColumnarMenu::default().with_name("completion_menu"));

        Reedline::create()
            .with_hinter(hinter)
            .with_completer(completer)
            .with_quick_completions(true)
            .with_highlighter(highlighter)
            .with_edit_mode(edit_mode)
            .with_menu(ReedlineMenu::EngineCompleter(completion_menu))
    }

    fn handle_input(&self, input: &str) {
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

    fn run(&mut self) -> Result<()> {
        loop {
            match self.editor.read_line(&self.prompt) {
                Ok(Signal::Success(input)) => self.handle_input(&input),
                Ok(Signal::CtrlC) => continue,
                Ok(Signal::CtrlD) => break,
                event => println!("Event: {event:?}"),
            }
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    ChouRepl::new().run()
}
