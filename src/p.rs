//! prelude; shorthand aliases for common types and traits, macros for common patterns.
pub(crate)use{Box as B,char as C,u32 as I,usize as U,Option as O,String as S,TryFrom as TF,TryInto as TI,Vec as V};
pub(crate)use{std::{alloc::Layout as L,clone::Clone as CL,cmp::{PartialEq as PE,PartialOrd as PO},
  collections::{BTreeMap as BM,VecDeque as VD},fmt::{Debug as DBG,Display as DS,Formatter as FMT,Result as FR},
  iter::{FromIterator as FI,IntoIterator as IIT,Iterator as IT},io::stdin,
  slice::{from_raw_parts,from_raw_parts_mut},str::FromStr as FS}};
pub(crate)use{anyhow::{Context,Error as E,anyhow as err,bail}};
#[macro_export] /**`return`*/              macro_rules! r   {()=>{return};($e:expr)=>{return $e};}
#[macro_export] /**`return Ok(Some(..))`*/ macro_rules! rro {($e:expr)=>{r!(Ok(Some($e)))}}
#[macro_export] /**`Ok(())`*/              macro_rules! ok {()=>{Ok(())}}
#[macro_export] /**`Box::new(..)`*/        macro_rules! b   {($e:expr)=>{B::new($e)};}
#[macro_export] /**`unreachable!()`*/      macro_rules! ur  {()=>{unreachable!()}}
/**`Result<T, anyhow::Error>`*/            pub type R<T> = Result<T,E>;
#[cfg(test)]/**test prelude*/pub(crate) mod tp{
  pub(crate) use{assert_eq as eq,assert_ne as neq,assert as is};
}
// todo: extension trait for abbreviated `try_into`, `try_from`
// todo: extension trait for abbreviated `map`, `and_then`, `unwrap`
