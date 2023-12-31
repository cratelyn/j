#![allow(dead_code,unused_variables,unreachable_code,unused_imports,unused_parens)]
#[path="../src/p.rs"]mod p; use{j::*,p::*,assert_eq as eq,assert_ne as neq,assert as is};
/**test helper: evaluate with empty symbol table*/fn eval_s(i:&str)->R<A>{eval(i,&mut ST::default()).map(|i|i.unwrap())}
#[cfg(test)]mod add{use super::*;
  #[test]fn add_two_consts()->R<()>{
    let(a@A{m:1,n:1,..})=eval_s("1 + 2")? else{bail!("bad dims")};eq!(a.as_i()?,3);ok!()}
  #[test]fn add_const_to_arr()->R<()>{
    let(a@A{m:1,n:3,..})=eval_s("1 + 1 2 3")? else{bail!("bad dims")};eq!(a.as_slice()?,&[2,3,4]);ok!()}
  #[test]fn add_arr_to_const()->R<()>{
    let(a@A{m:1,n:3,..})=eval_s("1 2 3 + 1")? else{bail!("bad dims")};eq!(a.as_slice()?,&[2,3,4]);ok!()}
  #[test]fn add_arr_to_arr()->R<()>{
    let(a@A{m:1,n:3,..})=eval_s("1 2 3 + 4 5 6")? else{bail!("bad dims")};eq!(a.as_slice()?,&[5,7,9]);ok!()}
  #[test]fn add_arr_to_rotated_matrix()->R<()>{
    let(a@A{m:3,n:2,..})=eval_s("1 2 3 + i. 3 2")? else{bail!("bad dims")};eq!(a.into_matrix()?,&[&[1,2],&[4,5],&[7,8]]);ok!()}
  #[test]fn add_slice_to_rotated_slice()->R<()>{
    let(a@A{m:4,n:1,..})=eval_s("1 2 3 4 + i. 4 1")? else{bail!("bad dims")};eq!(a.into_matrix()?,&[&[1],&[3],&[5],&[7]]);ok!()}
  #[test]fn other_add_slice_to_rotated_slice_is_length_error()->R<()>{is!(eval_s("i. 4 1 + 1 2 3 4").is_err());ok!()}
} #[cfg(test)]mod increment{use super::*;
  #[test]fn increment_scalar()->R<()>{let(i)=eval_s(">: 1")?;     eq!(i.as_i()?,       2);               ok!()}
  #[test]fn increment_slice ()->R<()>{let(i)=eval_s(">: 1 2 3")?; eq!(i.as_slice()?,   &[2,3,4]);        ok!()}
  #[test]fn increment_matrix()->R<()>{let(i)=eval_s(">: i. 2 2")?;eq!(i.into_matrix()?,&[&[1,2],&[3,4]]);ok!()}
} #[cfg(test)]mod tally{use super::*;
  macro_rules! t{($f:ident,$i:literal,$o:literal)=>{
    #[test]fn $f()->R<()>{let(a@A{m:1,n:1,..})=eval_s($i)? else{bail!("bad dims")};eq!(a.as_slice()?,&[$o]);ok!()}}}
  t!(tally_scalar,"# 1",1);t!(tally_1x3,"# 1 2 3",3);t!(tally_3x3,"# i. 3 3",9);
} #[cfg(test)]mod idot{use super::*;
  #[test]fn idot_3()->R<()>{let(a)=eval_s("i. 3")?;eq!(a.m,1);eq!(a.n,3);eq!(a.as_slice()?,&[0,1,2]);ok!()}
  #[test]fn idot_2_3()->R<()>{let(a)=eval_s("i. 2 3")?;eq!(a.m,2);eq!(a.n,3);let o:&[&[I]]=&[&[0,1,2],&[3,4,5]];
    eq!(a.into_matrix()?,o);eq!(a,o);ok!()}
  #[test]fn idot_3_2()->R<()>{let(a)=eval_s("i. 3 2")?;eq!(a.m,3);eq!(a.n,2);let o:&[&[I]]=&[&[0,1],&[2,3],&[4,5]];
    eq!(a.into_matrix()?,o);eq!(a,o);ok!()}
} #[cfg(test)]mod shape{use super::*;
  #[test]fn shape_idot_2_3()->R<()>{let(a)=eval_s("$ i. 2 3")?;eq!(a.m,1);eq!(a.n,2);
    eq!(a.get(1,1)?,2);eq!(a.get(1,2)?,3);ok!()}
  #[test]fn shape_idot_3_2()->R<()>{let(a)=eval_s("$ i. 3 2")?;eq!(a.m,1);eq!(a.n,2);
    eq!(a.get(1,1)?,3);eq!(a.get(1,2)?,2);ok!()}
} #[cfg(test)]mod trans{use super::*;
  #[test]fn trans_scalar()->R<()>{let(a)=eval_s("|: 3")?;eq!(a.m,1);eq!(a.n,1);
    eq!(a.get(1,1)?,3);ok!()}
  #[test]fn trans_idot_2_3()->R<()>{let(a)=eval_s("|: i. 2 3")?;eq!(a.m,3);eq!(a.n,2);
    let o:&[&[I]]=&[&[0,3],&[1,4],&[2,5]];eq!(a.into_matrix()?,o);eq!(a,o);ok!()}
} #[cfg(test)]mod mult{use super::*;
  #[test]fn mult_two_scalars()->R<()>{let(a)=eval_s("2 * 3")?;let(i)=a.as_i()?;eq!(i,6);ok!()}
  #[test]fn mult_slice_by_scalar()->R<()>{let(a)=eval_s("1 2 3 * 3")?;eq!(a.as_slice()?,&[3,6,9]);ok!()}
  #[test]fn mult_scalar_by_slice()->R<()>{let(a)=eval_s("3 * 1 2 3")?;eq!(a.as_slice()?,&[3,6,9]);ok!()}
  #[test]fn mult_slice_by_slice()->R<()>{let(a)=eval_s("2 4 6 * 1 2 3")?;eq!(a.as_slice()?,&[2,8,18]);ok!()}
  #[test]fn mult_slice_by_rotated_slice()->R<()>{let(a@A{m:3,n:2,..})=eval_s("1 2 3 * i. 3 2")? else{bail!("bad dims")};
    eq!(a.into_matrix()?,&[&[0,1],&[4,6],&[12,15]]);ok!()}
  #[test]fn mult_slice_to_rotated_slice()->R<()>{
    let(a@A{m:4,n:1,..})=eval_s("1 2 3 4 * i. 4 1")? else{bail!("bad dims")};eq!(a.into_matrix()?,&[&[0],&[2],&[6],&[12]]);ok!()}
} #[cfg(test)]mod left_and_right{use super::*;
  #[test]fn left_same_scalar() ->R<()>{let(a)=eval_s("[ 1")?;        eq!(a.as_i()?,1);           ok!()}
  #[test]fn left_same_slice()  ->R<()>{let(a)=eval_s("[ 1 2 3")?;    eq!(a.as_slice()?,&[1,2,3]);ok!()}
  #[test]fn right_same_scalar()->R<()>{let(a)=eval_s("] 1")?;        eq!(a.as_i()?,1);           ok!()}
  #[test]fn right_same_slice() ->R<()>{let(a)=eval_s("] 1 2 3")?;    eq!(a.as_slice()?,&[1,2,3]);ok!()}
  #[test]fn left_dyad()        ->R<()>{let(a)=eval_s("1 [ 2")?;      eq!(a.as_i()?,1);           ok!()}
  #[test]fn right_dyad()       ->R<()>{let(a)=eval_s("1 ] 2")?;      eq!(a.as_i()?,2);           ok!()}
  #[test]fn left_dyad_other()  ->R<()>{let(a)=eval_s("1 [ 2 3 4")?;  eq!(a.as_i()?,1);           ok!()}
  #[test]fn right_dyad_other() ->R<()>{let(a)=eval_s("1 ] 2 3 4")?;  eq!(a.as_slice()?,&[2,3,4]);ok!()}
  #[test]fn left_dyad_does_not_rotate_slice()->R<()>{
    /*NB: other operators like + or * may rotate the left-hand argument to fit. [ does not. */
    let(a)=eval_s("1 2 3 4 [ i. 4 1")?;eq!(a.as_slice()?,&[1,2,3,4]);ok!()}
} #[cfg(test)]mod symbol_assignment{use super::*;
  #[test]fn assign_and_get_i()->R<()>{let(mut st)=ST::default();let(a)=eval("a =: 3",&mut st)?;
    eq!(st.get_s("a").unwrap().as_i().unwrap(),3);ok!()}
  #[test]fn assign_and_get_slice()->R<()>{let(mut st)=ST::default();let(a)=eval("a =: 3 2 1",&mut st)?;
    eq!(st.get_s("a").unwrap().as_slice().unwrap(),&[3,2,1]);ok!()}
  #[test]fn assign_and_get_expr()->R<()>{let(mut st)=ST::default();let(a)=eval("a =: 1 3 + 2 4",&mut st)?;
    eq!(st.get_s("a").unwrap().as_slice().unwrap(),&[3,7]);ok!()}
  #[test]fn assign_and_eval_slice()->R<()>{let(mut st)=ST::default();let(eval_)=|s|eval(s,&mut st);
    let(mut i)=["a =: 3 2 1","a"].into_iter().map(eval_);  is!(i.next().unwrap()?.is_none());
    eq!(i.next().unwrap()?.unwrap().as_slice()?,&[3,2,1]); ok!()}
  #[test]fn assign_and_eval_expr()->R<()>{let(mut st)=ST::default();let(eval_)=|s|eval(s,&mut st);
    let(mut i)=["a =: >: i. 5","i =: 3","i ] \\ a"].into_iter().map(eval_);
    is!(i.next().unwrap()?.is_none()); is!(i.next().unwrap()?.is_none());
    eq!(i.next().unwrap()?.unwrap().into_matrix()?,&[&[1,2,3],&[2,3,4],&[3,4,5]]); ok!()}
} #[cfg(test)]mod misc{use super::*;
  #[test]fn empty_statement_evaluates_to_none()->R<()>{is!(eval("",&mut ST::default())?.is_none());ok!()}
  #[test]fn slice_times_transposed_idot_2_3()->R<()>{
    let(a)=eval_s("1 2 3 * |: i. 2 3")?;eq!(a.into_matrix()?,&[&[0,3],&[2,8],&[6,15]]);ok!()}
} #[cfg(test)]mod display{use super::*;
  #[test]fn display_scalar()->R<()>{let(a)=A::from_i(666)?;eq!(a.to_string(),"666\n");ok!()}
  #[test]fn display_slice()->R<()>{let a:&[I]=&[7,8,9];let a=A::try_from(a)?;eq!(a.to_string(),"7 8 9\n");ok!()}
  #[test]fn display_matrix()->R<()>{let(a)=eval_s("i. 3 3")?;eq!(a.to_string(),"0 1 2\n3 4 5\n6 7 8\n");ok!()}
} #[cfg(test)]mod adverb{use super::*;
  // === monadic / "insert" adverb
  #[test]fn insert_sum_one_number()->R<()>{let(a)=eval_s("+ / 1")?;let(i)=a.as_i()?;eq!(i,1);ok!()}
  #[test]fn insert_sum_two_numbers()->R<()>{let(a)=eval_s("+ / 1 8")?;let(i)=a.as_i()?;eq!(i,9);ok!()}
  #[test]fn insert_sum_a_sequence()->R<()>{let(a)=eval_s("+ / i. 4")?;let(i)=a.as_i()?;eq!(i,6);ok!()}
  #[test]fn insert_sum_a_shifted_sequence()->R<()>{let(a)=eval_s("+ / 1 + i. 4")?;let(i)=a.as_i()?;eq!(i,10);ok!()}
  #[test]fn insert_product_of_a_sequence()->R<()>{let(a)=eval_s("* / i. 3")?;let(i)=a.as_i()?;eq!(i,0);ok!()}
  #[test]fn insert_product_of_a_shifted_sequence()->R<()>{let(a)=eval_s("* / 2 + i. 3")?;let(i)=a.as_i()?;eq!(i,24);ok!()}
  // === monadic \ "prefix" adverb
  #[test]fn prefix_of_scalar()->R<()>{let(a)=eval_s("] \\ 1")?;let(i)=a.as_i()?;eq!(i,1);ok!()}
  #[test]fn prefix_of_slice()  ->R<()>{let(a)=eval_s("] \\ 1 2 3")?;eq!(a.into_matrix()?,&[&[1,0,0],
                                                                                           &[1,2,0],
                                                                                           &[1,2,3],]);ok!()}
  #[test]fn prefix_of_slice_2()->R<()>{let(a)=eval_s("+ \\ 1 2 3")?;eq!(a.into_matrix()?,&[&[1,0,0],
                                                                                           &[1,2,0],
                                                                                           &[1,2,3],]);ok!()}
  #[test]fn prefix_of_slice_3()->R<()>{let(a)=eval_s("* \\ 1 2 3")?;eq!(a.into_matrix()?,&[&[1,0,0],
                                                                                           &[1,1,0],
                                                                                           &[1,1,1],]);ok!()}
  // === dyadic / "table" adverb
  #[test]fn table_of_scalars_plus()->R<()>{let(a)=eval_s("1 + / 1")?;eq!(a.as_i()?,2);ok!()}
  #[test]fn table_of_scalars_mult()->R<()>{let(a)=eval_s("1 * / 1")?;eq!(a.as_i()?,1);ok!()}
  #[test]fn table_of_scalar_plus_slice()->R<()>{let(a)=eval_s("1 + / 1 2 3")?;eq!(a.as_slice()?,&[2,3,4]);ok!()}
  #[test]fn table_of_two_slices_mult()->R<()>{let(a)=eval_s("1 2 3 * / 1 2 3")?;eq!(a.into_matrix()?,&[&[1,2,3],
                                                                                                       &[2,4,6],
                                                                                                       &[3,6,9]]);
                                                                                                       ok!()}
  #[test]fn table_of_two_diff_slices_mult()->R<()>{let(a)=eval_s("2 4 * / 1 2 3")?;eq!(a.into_matrix()?,&[&[2,4,6],
                                                                                                          &[4,8,12]]);
                                                                                                          ok!()}
  // === dyadic \ "infix" adverb
  #[test]fn infix_to_reshape_1()->R<()>{let(a)=eval_s("1 ] \\ 1 2 3")?;eq!(a.into_matrix()?,&[&[1],
                                                                                              &[2],
                                                                                              &[3]]);
                                                                                              ok!()}
  #[test]fn infix_to_reshape_2()->R<()>{let(a)=eval_s("2 ] \\ 1 2 3")?;eq!(a.into_matrix()?,&[&[1,2],
                                                                                              &[2,3]]);
                                                                                              ok!()}
  #[test]fn infix_to_reshape_3()->R<()>{let(a)=eval_s("3 ] \\ 1 2 3 4")?;eq!(a.into_matrix()?,&[&[1,2,3],
                                                                                                &[2,3,4]]);
                                                                                                ok!()}
} #[cfg(test)]mod adverb_fancy{use super::*; /*XXX: these are left unsolved for now*/
  #[ignore] #[test]fn running_sum_of_a_sequence()->R<()>{let(a)=eval_s("+ / \\ 1 2 3 4 5")?;
    let(i)=a.as_slice()?;eq!(i,&[1,3,6,10,15]);ok!()}
  #[ignore] #[test]fn running_product_of_a_sequence()->R<()>{let(a)=eval_s("* / \\ 1 2 3 4 5")?;
    let(i)=a.as_slice()?;eq!(i,&[1,2,6,24,120]);ok!()}
}
