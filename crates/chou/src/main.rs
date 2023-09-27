use parser::parse;
use reedline::{DefaultPrompt, DefaultPromptSegment, Reedline, Signal};
use std::io::Result;

fn main() -> Result<()> {
    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::new(DefaultPromptSegment::Empty, DefaultPromptSegment::Empty);

    loop {
        match line_editor.read_line(&prompt) {
            Ok(Signal::Success(input)) => {
                let parse = parse(&input);
                println!("{}", parse.debug_tree());

                let syntax = parse.syntax();

                for error in ast::validation::validate(&syntax) {
                    println!("{}", error);
                }

                let root = ast::Root::cast(syntax).unwrap();
                let var_defs = root
                    .stmts()
                    .filter_map(|stmt| {
                        if let ast::Stmt::VariableDef(def) = stmt {
                            Some(def)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                dbg!(var_defs);

                dbg!(hir::lower(&root));
            }
            Ok(Signal::CtrlC) => {
                continue;
            }
            Ok(Signal::CtrlD) => {
                break;
            }
            event => {
                println!("Event: {:?}", event);
            }
        }
    }

    Ok(())
}
