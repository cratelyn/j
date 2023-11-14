use super::*; use std::marker::PhantomData as PD;
/// === mxn array === (a_ij) : 1<=i<=m, 1<=j<=n
/// ```text
/// [a_11, a_12, ... a_1n]
/// [a_21, a_22, ... a_2n]
/// [...., ...., ... ....]
/// [a_m1, a_m2, ... a_mn]
/// ```
#[derive(DBG)]pub struct A<X=MI>{/**columns*/ pub m:U,      /**rows*/  pub n:U,
                                 /**data*/        d:*mut u8,/**layout*/l:L,
                                 /**memory state*/i:PD<X>,                    }

/**memory indexing*/mod i{use super::*;
  impl<X:MX> A<X>{
    fn oob(&self,i:U,j:U)->R<()>{let A{m,n,..}=*self;
      if(i==0||j==0||i>m||j>n){bail!("({i},{j}) is out-of-bounds of ({m},{n})")}ok!()}
    /// returns the scalar `A_ij` within this array. returns an error if position is out-of-bounds.
    pub fn index(&self,i:U,j:U)->R<U>   {self.oob(i,j)?;let A{m,n,..}=*self;let(i,j)=(i-1,j-1);Ok((i*n)+j)}
    /// returns the scalar `A_ij` within this array. does not check if the position is in bounds.
    pub fn index_uc(&self,i:U,j:U)->R<U>{               let A{m,n,..}=*self;let(i,j)=(i-1,j-1);Ok((i*n)+j)}
  }
  // === test helpers ===
  /// `A::index` test case generator. check that indexing at a position returns the expected value.
  #[macro_export] macro_rules! ti{($f:ident,$a:ident,$i:literal,$j:literal,$o:literal)=>
    {#[test]fn $f()->R<()>{eq!($a()?.index($i,$j)?,$o);ok!()}}}
  /// `A::index` test case generator. check that indexing an out-of-bounds position returns an error.
  #[macro_export] macro_rules! toob{($f:ident,$a:ident,$i:literal,$j:literal)=>
    {#[test] fn $f()->R<()>{is!($a()?.index($i,$j).is_err());ok!()}}}
  // === 1x1 array indexing ===
  fn sca()->R<A>{let Ok(a@A{m:1,n:1,..})=A::from_i(42)else{bail!("bad dims")};Ok(a)}
  toob!(i00_for_scalar,sca,0,0);toob!(i10_for_scalar,sca,1,0);toob!(i01_for_scalar,sca,0,1);
  toob!(i21_for_scalar,sca,2,1);toob!(i12_for_scalar,sca,1,2);toob!(i22_for_scalar,sca,2,2);
  ti!(i11_for_scalar,sca,1,1,0);
  // === 2x3 array indexing ===
  fn arr2x3()->R<A>{let Ok(a@A{m:2,n:3,..})=A::zeroed(2,3)else{bail!("bad dims")};Ok(a)}
  toob!(i00_for_2x3,arr2x3,0,0);toob!(i01_for_2x3,arr2x3,0,1);toob!(i10_for_2x3,arr2x3,1,0);
  ti!(i11_for_2x3,arr2x3,1,1,0);ti!(i12_for_2x3,arr2x3,1,2,1);ti!(i13_for_2x3,arr2x3,1,3,2);
  ti!(i21_for_2x3,arr2x3,2,1,3);ti!(i22_for_2x3,arr2x3,2,2,4);ti!(i23_for_2x3,arr2x3,2,3,5);
  // === 3x3 array indexing ===
  fn arr3x3()->R<A>{let Ok(a@A{m:3,n:3,..})=A::zeroed(3,3)else{bail!("bad dims")};Ok(a)}
  toob!(i00_for_3x3,arr3x3,0,0);toob!(i01_for_3x3,arr3x3,0,1);toob!(i14_for_3x3,arr3x3,1,4);
  toob!(i41_for_3x3,arr3x3,4,1);toob!(i44_for_3x3,arr3x3,4,4);
  ti!(i11_for_3x3,arr3x3,1,1,0);ti!(i21_for_3x3,arr3x3,2,1,3);ti!(i22_for_3x3,arr3x3,2,2,4);
  ti!(i31_for_3x3,arr3x3,3,1,6);ti!(i33_for_3x3,arr3x3,3,3,8);
}

/**array allocation*/mod alloc{use{super::*,std::alloc::{alloc,alloc_zeroed,dealloc}};
  /**sealed trait, memory markers*/pub trait MX{} impl MX for MU {} impl MX for MI {}
  /**marker: memory uninitialized*/#[derive(CL,DBG)]pub struct MU;
  /**marker: memory initialized*/  #[derive(CL,DBG)]pub struct MI;
  impl A<MU>{
    pub fn new(m:U,n:U)->R<Self>{Self::alloc(m,n).map(|(l,d)|A{m,n,d,l,i:PD})}
    // TODO: use `A::iter`
    // TODO: use `set_unchecked` here.
    pub fn init_with<F:FnMut(U,U)->R<I>>(mut self,mut f:F)->R<A<MI>>{let(A{m,n,..})=self;
      for(i)in(1..=m){for(j)in(1..=n){let(v)=f(i,j)?;self.set(i,j,v)?;}}Ok(unsafe{self.finish()})}
    pub unsafe fn finish(self)->A<MI>{std::mem::transmute(self)}
  }
  impl A<MI>{
    pub fn zeroed(m:U,n:U)->R<Self>{let(l,d)=Self::allocz(m,n)?;Ok(A{m,n,d,l,i:PD})}
    pub fn from_i(i:I)->R<Self>{let mut a=A::new(1,1)?;a.set(1,1,i)?;Ok(unsafe{a.finish()})}}
  impl TF<I> for A<MI>{type Error=E;fn try_from(i:I)->R<Self>{A::from_i(i)}}
  impl<X:MX> A<X>{
    fn alloc(m:U,n:U)->R<(L,*mut u8)>{let(l)=Self::l(m,n)?;let d=unsafe{alloc(l)};Ok((l,d))}
    fn allocz(m:U,n:U)->R<(L,*mut u8)>{let(l)=Self::l(m,n)?;let d=unsafe{alloc_zeroed(l)};Ok((l,d))}
    fn l(m:U,n:U)->R<L>{L::array::<I>(m*n).map_err(E::from)}
  }
  impl<M> Drop for A<M>{fn drop(&mut self){let(A{d,l,..})=self;unsafe{dealloc(*d,*l)}}}
  // TODO: add compile_fail checks to ensure that e.g. `get` cannot be used on an uninitialized array
} pub use self::alloc::{MI,MU,MX};

/**array access*/mod access{use{super::*,std::mem::size_of};
  impl A<MI>{ // only allow read access for initialized arrays
    pub fn get(&self,i:U,j:U)->R<I>{Ok(unsafe{self.ptr_at(i,j)?.read()})}
    pub fn get_uc(&self,i:U,j:U)->R<I>{Ok(unsafe{self.ptr_at_uc(i,j)?.read()})}
    /// returns an iterator whose elements are tuples (i,j) across the array's positions.
    pub fn iter(&self)->impl IT<Item=(U,U)>{let A{m,n,..}=*self;(1..=m).flat_map(move|i|(1..=n).map(move|j|(i,j)))}
    pub fn vals<'a>(&'a self)->impl IT<Item=I> + 'a{let A{m,n,..}=*self;
      (1..=m).flat_map(move|i|(1..=n).map(move|j|self.get_uc(i,j).expect("reads work")))}}
  impl<X:MX> A<X>{
    /// sets the value at the given position.
    pub fn set(&mut self,i:U,j:U,v:I)->R<()>{unsafe{self.ptr_at(i,j)?.write(v);}Ok(())}
    /// returns a raw pointer to the underlying array memory at (i,j). returns an error if position is out-of-bounds.
    pub(crate)fn ptr_at(&self,i:U,j:U)->R<*mut I>{self.ptr_at_impl(i,j,Self::index)}
    /// returns a raw pointer to the underlying array memory at (i,j). does not check if position is in bounds.
    pub(crate)fn ptr_at_uc(&self,i:U,j:U)->R<*mut I>{self.ptr_at_impl(i,j,Self::index_uc)}
    fn ptr_at_impl<F:Fn(&Self,U,U)->R<U>>(&self,i:U,j:U,f:F)->R<*mut I>{
      let(o):isize=f(self,i,j).map(|x|x*size_of::<I>())?.try_into()?;let(d)=unsafe{(self.d.offset(o) as *mut I)};
      Ok(d)}
  }}

/**scalar conversion/comparison*/mod scalars{use super::*; /*todo...*/
  impl A<MI>{pub fn as_i(&self)->R<I>{let a@A{m:1,n:1,..}=self else{bail!("not a scalar")};a.get(1,1)}}
  #[test]fn scalar_get()->R<()>{let a=A::from_i(42)?;eq!(a.get(1,1)?,42);drop(a);ok!()}
  #[test]fn scalar_set()->R<()>{let mut a=A::from_i(42)?;a.set(1,1,420)?;eq!(a.get(1,1)?,420);drop(a);ok!()}}

/**slices conversion/comparison*/mod slices{use super::*;
  impl A<MI>{ // only allow slice access for initialized arrays
    pub fn as_slice(&self)->R<&[I]>{let(A{m:1,n:l,d,..}|A{m:l,n:1,d,..})=(self)else{bail!("not a slice")};
      Ok(unsafe{from_raw_parts(*d as *mut I,*l as U)})}}
  impl PE<&[I]> for A{fn eq(&self,r:&&[I])->bool{self.as_slice().map(|s|s.eq(*r)).unwrap_or(false)}}
  impl TF<&[I]> for A{type Error=E;fn try_from(s:&[I])->R<A>{
    let(m,n)=(1,s.len());let(mut a)=A::new(m,n)?;
    for(i,v)in(s.iter().enumerate()){let(i)=(i+1).try_into()?;a.set(1,i,*v)?;}Ok(unsafe{a.finish()})}}
  impl TF<V<I>> for A{type Error=E;fn try_from(v:V<I>)->R<A>{v.as_slice().try_into()}}
  #[test]fn scalars_can_be_a_slice()->R<()>{let(a)=A::from_i(420)?;let _:&[I]=a.as_slice()?;ok!()}
  #[test]fn from_empty()->R<()>{let a:&[I]=&[];let _=A::try_from(a)?;ok!()}
  #[test]fn from_one()->R<()>{let a:&[I]=&[42];let a=A::try_from(a)?;eq!(a.get(1,1)?,42);ok!()}
  #[test]fn from_three()->R<()>{let a:&[I]=&[7,8,9];let a=A::try_from(a)?;
    eq!(a.get(1,1)?,7);eq!(a.get(1,2)?,8);eq!(a.get(1,3)?,9);is!(a.get(1,0).is_err());is!(a.get(1,4).is_err());ok!()}
  #[test]fn eq_three()->R<()>{let(s):&[I]=&[1,2,3];let(a)=(A::try_from(s)?);eq!(a,s);ok!()}
  #[test]fn neq_three()->R<()>{let(s,o):(&[I],&[I])=(&[1,2],&[2,3]);let(a)=(A::try_from(s)?);neq!(a,o);ok!()}
  #[test]fn neq_prefix()->R<()>{let(s,o):(&[I],&[I])=(&[1,2],&[1,2,3]);let(a)=(A::try_from(s)?);neq!(a,o);ok!()}
  #[test]fn column_slice_can_be_a_slice()->R<()>{let(a):&[I]=&[7,8,9];
    let(a@A{m:3,n:1,..})=A::try_from(a)?.m_trans()?else{bail!("bad dims")};eq!(a.as_slice()?,&[7,8,9]);ok!()}}

/**matrix conversion/comparison*/mod matrices{use super::*;
  impl A<MI>{ // only allow matrix access for initialized arrays
    pub fn into_matrix(&self)->R<Vec<&[I]>>{let(A{m,n,..})=*self;
      (0..m).map(|m|{self.ptr_at(m+1,1)}).map(|r|r.map(|d|unsafe{from_raw_parts(d as *mut I, n as U)})).collect()}}
  impl TF<&[&[I]]> for A{type Error=E;fn try_from(s:&[&[I]])->R<A>{todo!("TF<&[I]> for A")}}
  impl PE<&[&[I]]> for A{fn eq(&self,r:&&[&[I]])->bool{let(A{m,n,..})=self;
    if(r.len()!=self.m){r!(false)}for(i,r_i)in(r.into_iter().enumerate()){
      if(r_i.len()!=self.n){r!(false)}for(j,r_ij)in(r_i.into_iter().enumerate()){
        let(i,j)=(i+1,j+1);let(a_ij)=match(self.get(i,j)){Ok(v)=>v,Err(_)=>r!(false)};
        if(a_ij)!=(*r_ij){r!(false)}}}true}}
}

/**monadic verbs*/impl A{
  pub fn m_same(self)->R<A>{Ok(self)}
  pub fn m_idot(self)->R<A>{let(a@A{m,n,..})=self;let gi=|i,j|a.get(i,j)?.try_into().map_err(E::from);
    if let(1,1)=(m,n){let(m,n)=(1,gi(1,1)?);let(mut o)=A::new(1,n)?;
      for(j)in(1..=n){o.set(1,j,(j-1).try_into()?)?;}Ok(unsafe{o.finish()})}
    else if let(1,2)=(m,n){let(m,n)=(gi(1,1)?,gi(1,2)?);
      let(mut v)=0_u32;let(f)=move |_,_|{let(v_o)=v;v+=1;Ok(v_o)};A::new(m,n)?.init_with(f)}
    else{bail!("i. {m}x{n} not supported")}}
  pub fn m_shape(self)->R<A>{let(A{m,n,..})=self;let(a):&[I]=&[m as I,n as I];A::try_from(a)}
  pub fn m_tally(self)->R<A>{let A{m,n,..}=self;let(i)=I::try_from(m*n)?;A::from_i(i)}
  pub fn m_trans(self)->R<A>{let(i@A{m:m_i,n:n_i,..})=self;let(m_o,n_o)=(n_i,m_i);
    let(f)=|i_o,j_o|{i.get(j_o,i_o)};A::new(m_o,n_o)?.init_with(f)}
}

/**dyadic verbs*/impl D{
  /*return dyad function**/ pub fn f(&self)->fn(I,I)->I{use D::*;
    match(self){Plus=>D::add, Mul=>D::mul, Left=>D::left, Right=>D::right} }
  /*add two numbers*/fn add (x:I,y:I)->I{x+y} /*multiply two numbers*/fn mul  (x:I,y:I)->I{x*y}
  /*left           */fn left(x:I,y:I)->I{x  } /*right               */fn right(x:I,y:I)->I{  y}
} impl A{
  pub fn d_left (self,r:A)->R<A>{Ok(self)                }
  pub fn d_right(self,r:A)->R<A>{Ok(r)                   }
  pub fn d_plus(self,r:A) ->R<A>{A::d_do(self,r,D::add)}
  pub fn d_mul (self,r:A) ->R<A>{A::d_do(self,r,D::mul)}
  pub fn d_do(l@A{m:ml,n:nl,..}:A,r@A{m:mr,n:nr,..}:A,f:impl Fn(I,I)->I)->R<A<MI>>{
            let(li,ri)=(l.as_i().ok(),r.as_i().ok());let(ls,rs)=(l.as_slice().ok(),r.as_slice().ok());
         if let(Some(li),Some(ri))=(li,ri){r!(A::from_i(f(li,ri)))}                                                     // two scalars
    else if let(_,Some(s),None,a@A{m,n,..})|(a@A{m,n,..},None,Some(s),_)=(&l,li,ri,&r)                                  // scalar and array
      {let(f)=|i,j|{Ok(f(a.get(i,j)?,s))};r!(A::new(*m,*n)?.init_with(f))}
    else if let(_,Some(s),None,a@A{m,n,..})|(a@A{m,n,..},None,Some(s),_)=(&l,ls,rs,&r)                                  // slice and array
      {if(s.len()==*m){let(f)=|i,j|{let(x)=a.get(i,j)?;let(y)=(s[i-1]);Ok(f(x,y))};r!(A::new(*m,*n)?.init_with(f))}}
    else if (ml==mr)&&(nl==nr){let(m,n)=(ml,nl);r!(A::new(m,n)?.init_with(                                              // matching arrays
      |i,j|{let(l,r)=(l.get(i,j)?,r.get(i,j)?);Ok(f(l,r))}))}
    else if (ml==nr)&&(nl==mr) /*NB: inherit the dimensions of the right-hand operand.*/                                // rotation
      {let(f)=|i,j|{let(x)=l.get(j,i)?;let(y)=r.get(i,j)?;Ok(f(x,y))};r!(A::new(mr,nr)?.init_with(f))}
    bail!("length error");
  }
}

/**deep-copy*/impl A<MI>{
  pub fn deep_copy(&self)->R<A>{let A{m,n,l:li,d:di,i:_}=*self;A::new(m,n)?.init_with(|i,j|{self.get(i,j)})}
}

/**display*/mod fmt{use super::*;
  impl DS for A<MI>{
    // TODO: buffer stdout, flush after loops
    // TODO: use `unchecked` to elide bounds checks in printing
    fn fmt(&self,fmt:&mut FMT)->FR{let A{m,n,..}=*self;for(i,j)in(self.iter())
      {let(x)=self.get_uc(i,j).map_err(|_|std::fmt::Error)?;write!(fmt,"{x}{}",if(j==n){'\n'}else{' '})?;}ok!()}}}
