/**input lexing*/pub(crate) use lex::lex;mod lex{use crate::*;
  /**syntax token*/ #[derive(CL,DBG,PE)] pub(crate) enum T {/*array literal*/A(V<I>),
  /* NB: this does not identify whether possible verbs  */  /*(ad)verb*/     V(S)   ,
  /* are monadic or dyadic. that is done during parsing.*/  /*symbol*/       SY(SY) }
  impl T{pub(super) fn is_noun(&self)->bool{use T::*;matches!(self,A(_)|SY(_))}}
  pub(crate) fn lex(input:&str)->R<V<T>>{use std::ops::Deref;
    let(mut ts)=input.split_whitespace().peekable(); let(mut o)=V::with_capacity(ts.size_hint().0);
    while     let Some(t)    =ts.next(){
           if let Some(sy)   =t.parse().ok().map(T::SY){o.push(sy);}          // symbol
      else if let Some(mut v)=t.parse().ok().map(|i|vec![i]){                 // array literal
              macro_rules! peek{()=>{ts.peek().and_then(|t|t.parse().ok())}}  // ..is the next token a number?
              macro_rules! put{($i:ident)=>{ts.next().map(drop);v.push($i);}} // ..append to our array literal
              while let Some(i)=peek!(){put!(i);} o.push(T::A(v));}
      else {o.push(T::V(S::from(t)))}                                         // otherwise, a verb or adverb
    } r!(Ok(o)) }
  #[cfg(test)]mod t{use super::{*,T::A as TA,T::V as TV,T::SY as TSY};
    /// test helper: lex an expression and check the output
    macro_rules! t{($f:ident,$i:literal,$o:expr)=>{#[test]fn $f()->R<()>{eq!(lex($i)?,$o);ok!()}}}
    // === lexing unit tests ===
    t!(lex_1,         "1",            v![TA(v![1])]                                                                  );
    t!(lex_9,         "9",            v![TA(v![9])]                                                                  );
    t!(lex_1to3,      "1 2 3",        v![TA(v![1,2,3])]                                                              );
    t!(lex_monad,     "# 1 2 3",      v![TV(S::from("#")), TA(v![1,2,3])]                                            );
    t!(lex_dyad,      "1 + 2",        v![TA(v![1]),        TV(S::from("+")), TA(v![2])]                              );
    t!(lex_two_verbs, "1 + # 1 2 3",  v![TA(v![1]),        TV(S::from("+")), TV(S::from("#")), TA(v![1,2,3])]        );
    t!(lex_symbol,    "abc",          v![TSY("abc".parse().unwrap())]                                                );
  }
}/**input parsing*/pub(crate) use parse::{D,M,N,parse};mod parse{use {crate::*,super::lex::{T,lex}};
  /**dyadic verb       */ #[derive(DBG,PE,PO)] pub enum D {Plus,Mul,  Left, Right         }
  /**monadic verb      */ #[derive(DBG,PE,PO)] pub enum M {Idot,Shape,Tally,Transpose,Same}
  /**ast node          */ #[derive(DBG,     )] pub enum N {/**array literal*/    A{a:A},
                                                           /**dyadic verb*/      D{d:D,l:B<N>,r:B<N>},
                                                           /**monadic verb*/     M{m:M,o:B<N>},
                                                           /**symbol*/           S{sy:SY},
                                                           /**symbol assignment*/V{sy:SY,e:B<N>}}
  impl From<SY> for N{fn from(sy:SY)->N{N::S{sy}}}
  impl TF<Vec<I>> for N{type Error=E; fn try_from(a:Vec<I>)->R<N>{a.try_into().map(|a|N::A{a})}}
  /**parse a sequence of tokens into an abstract syntax tree.*/
  pub(crate) fn parse(ts:&mut V<T>)->R<O<B<N>>>{const MAX:u32=128;let(mut ctx,mut i)=(V::new(),0);
    while(!ts.is_empty()){if(i>MAX){bail!("max iterations")}parse_(ts,&mut ctx)?;i+=1;}
    /*debug*/debug_assert!(ts.is_empty());debug_assert!(ctx.len() <= 1,"AST needs a root node: {ctx:?}");/*debug*/
    Ok(ctx.pop())}
  fn parse_(ts:&mut V<T>,ctx:&mut V<B<N>>)->R<()>{
    macro_rules! step{($n:expr)=>{ctx.push(b!($n));r!(ok!());}}
    let(v):S=match ts.pop(){
      Some(T::V(v))  =>v, /*take the next verb, or return if done*/ None=>r!(ok!()),
      Some(T::A(v))  =>{let(n)=v.try_into()?;step!(n);}   // array literal
      Some(T::SY(sy))=>{let(n)=sy.into();    step!(n);}}; // symbol name
    let(rhs)=ctx.pop().ok_or(err!("no right-hand operand for `{v:?}`"))?;
    if ts.last().map(T::is_noun).unwrap_or(false){ // dyadic verbs
      let(l)=match(ts.pop()).unwrap(/*we just peeked this*/){T::V(v)=>ur!(),/*..and checked it isn't a verb*/
        T::A(a)=>                                         {a.try_into().map(|a|b!(a))?}
        T::SY(sy)=>if(v=="=:"){step!(N::V{sy,e:rhs});}else{b!(sy.into())}             }; // handle variable assignment
          let(d)=D::new(&v).ok_or(err!("invalid dyadic verb {v:?}"))?;  step!(N::D{d,l,r:rhs});
    }else{let(m)=M::new(&v).ok_or(err!("invalid monadic verb {v:?}"))?; step!(N::M{m,o:rhs}); }}
  impl M{fn new(s:&str)->O<M>{use M::*;Some(match s{"i."=>Idot,"$"=>Shape,"#"=>Tally,"|:"=>Transpose,"["|"]"=>Same,_=>r!(None)})}}
  impl D{fn new(s:&str)->O<D>{use D::*;Some(match s{"+"=>Plus,"*"=>Mul,"["=>Left,"]"=>Right,_=>r!(None)})}}
  #[cfg(test)]mod t{use super::*;
    macro_rules! t{($f:ident,$i:literal)=>{#[test]fn $f()->R<()>{let(mut ts)=lex($i)?;let ast=parse(&mut ts)?;ok!()}}}
    macro_rules! tf{($f:ident,$i:literal)=>{#[test] #[should_panic]fn $f(){let(mut ts)=lex($i).unwrap();let ast=parse(&mut ts).unwrap();}}}
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
