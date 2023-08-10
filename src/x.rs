//! a J interpreter fragment, implemented in Rust.
#![allow(dead_code,unused_variables,unreachable_code,unused_imports,unused_parens)]
mod p; use{p::*,j::*}; fn main()->R<()>{use std::io::Write;let(mut st)=ST::default();
  let(pr)=||{print!("  ");std::io::stdout().flush()};
  let(rl)=||{let(mut l)=S::new();stdin().read_line(&mut l)?;Ok::<_,E>(l)};let(mut ev)=|l:S|eval(&l,&mut st);
  loop{pr()?;let(o)=rl().and_then(|l|ev(l))?;if let Some(o)=o{println!("{}",o)}}ur!();}
