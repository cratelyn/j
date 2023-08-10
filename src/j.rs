#![allow(dead_code,unused_variables,unreachable_code,unused_imports,unused_parens)]
/**prelude*/mod p;use p::*;#[cfg(test)]use p::tp::*;
/**array*/mod a; /**read input*/mod r; /**symbol table*/mod s;
pub use self::{a::*,r::*,s::*};
pub fn eval(input:&str,st:&mut ST)->R<O<A>>{parse(input)?.ok_or(err!("empty")).and_then(|ast|eval_(ast,st))}
fn eval_(ast:B<N>,st:&mut ST)->R<O<A>>{use{M::*,D::*};
  let(mut rec)=|a|->R<A>{match(eval_(a,st)){Ok(Some(a))=>Ok(a),Err(e)=>Err(e), // recursively evaluate subexpression.
    Ok(None)=>Err(err!("expression did not result in a value"))}};
  match *ast{
  N::A{a}    =>Ok(a),
  N::M{m,o}  =>{let(a)=rec(o)?;            match m{Idot=>a.m_idot(),  Shape=>a.m_shape(),
                                                   Tally=>a.m_tally(),Transpose=>a.m_trans()}}
  N::D{d,l,r}=>{let(l,r)=(rec(r)?,rec(l)?);match d{Plus=>l.d_plus(r), Mul=>l.d_mul(r)}}
  N::S{sy}   =>{st.get(&sy).ok_or(err!("undefined symbol: {sy:?}"))?;todo!("symbol value clone")}
  N::V{sy,e} =>{let(a)=rec(e)?;st.insert(sy,a);r!(Ok(None))}
}.map(O::Some)}
