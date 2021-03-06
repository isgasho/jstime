use rusty_v8 as v8;

use rustyline::error::ReadlineError;
use rustyline::Editor;

pub fn start() {
  let isolate = &mut v8::Isolate::new(Default::default());
  let scope = &mut v8::HandleScope::new(isolate);
  let context = v8::Context::new(scope);
  let scope = &mut v8::ContextScope::new(scope, context);

  let mut rl = Editor::<()>::new();

  println!("\nWelcome to jstime!\n");
  loop {
    let readline = rl.readline(">> ");
    match readline {
      Ok(line) => {
        let code = v8::String::new(scope, &line).unwrap();

        let script = v8::Script::compile(scope, code, None).unwrap();
        let result = script.run(scope).unwrap();
        let result = result.to_string(scope).unwrap();
        println!("{}", &result.to_rust_string_lossy(scope));
      },
      Err(ReadlineError::Interrupted) => {
        println!("Thanks for stopping by!");
        break
      },
      Err(ReadlineError::Eof) => {
        println!("Eof'd");
        break
      },
      Err(err) => {
        eprintln!("Error: {:?}", err);
        break
      }
    }
  }
}
