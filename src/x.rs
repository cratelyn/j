/**a J interpreter fragment, implemented in Rust.*/
mod p;use{p::*,j::*,std::io::Write};
/// main interpreter entrypoint and event-loop.
fn main()->R<()>{let mut st=ST::default();                                 // define symbol table
  let prompt  =|   |{print!("  ");std::io::stdout().flush()?;ok!()};       // (callback) print whitespace
  let read    =|_  |{let mut l=S::new();stdin().read_line(&mut l)?;Ok(l)}; // (callback) read input
  let mut eval=|s:S|{eval(&s,&mut st)};                                    // (callback) read and evaluate once
  let print   =|a:A|{println!("{a}")};                                     // (callback) print array
  loop{prompt().and_then(read).and_then(&mut eval)?.map(print);};          /* !!! main event loop !!! */ }
