/**input lexing*/mod lex{use crate::*;
  /**syntax token*/ #[derive(CL,DBG,PE)] pub(crate) enum T {I(I),V(S)}
  pub(crate) fn lex(input:&str)->R<V<T>>{input.split_whitespace().map(str::parse::<T>).collect()}
  impl FS for T{type Err=E;fn from_str(s:&str)->R<T>{Ok(if let Ok(i)=s.parse::<I>(){T::I(i)}else{T::V(s.to_owned())})}}
  #[cfg(test)]mod t{use super::*;
    #[test]fn lex_single_scalar()->R<()>{let ts=lex("1")?;eq!(ts.as_ref(),[T::I(1)]);ok!()}
    #[test]fn lex_1x3()->R<()>{let ts=lex("1 2 3")?;eq!(ts.as_ref(),[T::I(1),T::I(2),T::I(3)]);ok!()}
    #[test]fn lex_1plus2()->R<()>{let ts=lex("1 + 2")?;eq!(ts.as_ref(),[T::I(1),T::V(S::from("+")),T::I(2)]);ok!()}
  }
}/**input parsing*/pub(crate) use parse::{D,M,N,parse};mod parse{use {crate::*,super::lex::{T,lex}};
  /**dyadic verb       */ #[derive(DBG,PE,PO)] pub enum D {Plus,Mul,  Left, Right         }
  /**monadic verb      */ #[derive(DBG,PE,PO)] pub enum M {Idot,Shape,Tally,Transpose,Same}
  /**ast node          */ #[derive(DBG,     )] pub enum N {/**array literal*/    A{a:A},
                                                           /**dyadic verb*/      D{d:D,l:B<N>,r:B<N>},
                                                           /**monadic verb*/     M{m:M,o:B<N>},
                                                           /**symbol*/           S{sy:SY},
                                                           /**symbol assignment*/V{sy:SY,e:B<N>}}
  /**read array values from token stream; continue until verb or symbol is found, or stream is empty.*/
  fn reada(ts:&mut V<T>)->R<O<B<N>>>{let mut a=VD::new();loop{match ts.pop(){Some(T::I(i))=>{a.push_front(i);continue}
    Some(t)=>ts.push(t),None=>{}}if(a.is_empty()){return Ok(None);}else{
      a.make_contiguous();rro!(b!(N::A{a:a.as_slices().0.try_into()?}));}}}
  fn parse_(ts:&mut V<T>,ctx:&mut V<B<N>>)->R<()>{if let Some(a)=reada(ts)?{ctx.push(a);r!(ok!())}
            let(v)=match ts.pop(){Some(T::V(v))=>v,Some(T::I(_))=>ur!(),None=>{r!(ok!())}};
         if let Some(m)=M::new(&v){let(o)=ctx.pop().ok_or(err!("no operand {m:?}"))?;ctx.push(b!(N::M{m,o}))}
    else if let Some(d)=D::new(&v){let(r)=ctx.pop().ok_or(err!("no rhs {d:?}"))?;parse_(ts,ctx)?;
      let(l)=ctx.pop().ok_or(err!("no lhs"))?;ctx.push(b!(N::D{d,l,r}))}
    else if v == "=:" {let(e)=ctx.pop().ok_or(err!("assignment requires an expression"))?;
        let(sy)=match(ts.pop()){Some(T::V(sy))=>sy,_=>bail!("assignment must be bound to a variable")}.parse::<SY>()?;
        ctx.push(b!(N::V{sy,e}));}
    else if let Ok(sy)=v.parse::<SY>(){ctx.push(b!(N::S{sy}))}
    else{bail!("unrecognized verb / invalid symbol {v}")}
    ok!()}
  pub(crate) fn parse(input:&str)->R<O<B<N>>>{const MAX:u32=128;let(mut ts,mut ctx,mut i)=(lex(input)?,V::new(),0);
    while(!ts.is_empty()){if(i>MAX){bail!("max iterations")}parse_(&mut ts,&mut ctx)?;i+=1;}
    /*debug*/debug_assert!(ts.is_empty());if(!input.trim().is_empty()){debug_assert_eq!(ctx.len(),1);}/*debug*/
    Ok(ctx.pop())}
  impl M{fn new(s:&str)->O<M>{use M::*;Some(match s{"i."=>Idot,"$"=>Shape,"#"=>Tally,"|:"=>Transpose,"["|"]"=>Same,_=>r!(None)})}}
  impl D{fn new(s:&str)->O<D>{use D::*;Some(match s{"+"=>Plus,"*"=>Mul,"["=>Left,"]"=>Right,_=>r!(None)})}}
  #[cfg(test)]mod t{use super::*;
    macro_rules! t{($f:ident,$i:literal)=>{#[test]fn $f()->R<()>{let ast=parse($i)?;ok!()}}}
    macro_rules! tf{($f:ident,$i:literal)=>{#[test] #[should_panic]fn $f(){parse($i).unwrap();}}}
    t!(parse_1x1,"1"); t!(parse_1x3,"1 2 3"); t!(parse_tally_1,"# 1"); t!(parse_tally_1x3,"# 1 2 3");
    tf!(parse_tally_as_dyad_fails, "1 # 2"); tf!(parse_tally_with_no_operand, "#");
    tf!(parse_idot_as_dyad_fails, "1 # 2"); tf!(parse_idot_with_no_operand, "i.");
    t!(parse_idot_1,"i. 1"); t!(parse_idot_1x2,"i. 4 3"); t!(parse_1plus2,"1 + 2");
    t!(parse_1x3_times_1x3,"1 2 3 * 4 5 6"); t!(parse_tally_tally_1x3,"# # 1 2 3");
    t!(parse_symbol,"a"); t!(parse_symbol_plus_symbol,"a + b"); t!(parse_tally_symbol,"# a");
    t!(parse_symbol_times_symbol,"a * b"); t!(parse_tally_tally_symbol,"# # a");
    tf!(parse_symbol_times_symbol_numbers,"a * b 1"); tf!(parse_tally_tally_symbol_symbol,"# # a b");
    t!(assign_symbol_scalar,"a =: 1"); t!(assign_symbol_slice,"a =: 1 2 3"); t!(assign_symbol_idot,"a =: i. 2 3");
    t!(assign_symbol_slice_plus_slice,"a =: 1 2 3 + 1 2 3"); t!(parse_empty,"");
  }
}
