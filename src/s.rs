use super::*; use std::ops::Not;
/**symbol*/      #[derive(PE,DBG,PO,Ord,Eq,CL)]pub struct SY(S);
/**symbol table*/#[derive(Default)]            pub struct ST{st:BM<SY,A>}

/**symbol parsing*/mod syfs{use super::*;
  impl FS for SY{type Err = E;fn from_str(s:&str)->R<SY>{
    let(sc)=s.chars().collect::<V<_>>();let(sf)=sc.first().ok_or(err!("empty symbol"))?;
    if(sf.is_ascii_lowercase().not()){bail!("symbols must start with a-z")}
    let(sv)=|c:&char|c.is_ascii_lowercase()||c.is_ascii_digit()||*c=='_'; // validate
    if(sc.iter().all(sv).not()){bail!("symbols may only contain a-z, 0-9, or `_`")}
    Ok(SY(s.to_owned()))}}
  #[test] fn simple_symbol_succeeds()         {is!(SY::from_str("abc")    .is_ok())}
  #[test] fn underscore_symbol_succeeds()     {is!(SY::from_str("abc_def").is_ok())}
  #[test] fn trailing_number_symbol_succeeds(){is!(SY::from_str("a1")     .is_ok())}
  #[test] fn empty_symbol_fails()             {is!(SY::from_str("")       .is_err())}
  #[test] fn number_symbol_fails()            {is!(SY::from_str("1")      .is_err())}
  #[test] fn leading_number_symbol_fails()    {is!(SY::from_str("1a")     .is_err())}
}

impl ST{
  pub fn get(&self,sy:&SY)->O<&A>{self.st.get(sy)}
  pub fn get_s(&self,sy:&str)->O<&A>{self.st.get(&sy.parse::<SY>().expect("valid symbol"))}
  pub fn insert(&mut self,sy:SY,a:A){self.st.insert(sy,a);}
}
