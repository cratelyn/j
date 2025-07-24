# 💐 j

j is a limited subset of J, an array programming language. this file is an accompanying essay.

this project is, in spirit, a reimagining of the ["Incunabulum"][incunabulum] J interpreter
fragment, implemented in Rust. this project is, in a sense, a work of speculative science-fiction,
examining the linguistics of programming languages.

## ✨ technical overview

the jsoftware wiki includes the following story concerning the original Incunabulum:

> One summer weekend in 1989, Arthur Whitney visited Ken Iverson at Kiln Farm
> and produced—on one page and in one afternoon—an interpreter fragment on the
> AT&T 3B1 computer.

accordingly, j does not intend to be a fully-fledged implementation of an array language.

visit <https://www.jsoftware.com/> if you would like to learn more about J. the J wiki's
["Getting Started"][j-getting-started] page has useful information about installing J, and the
["NuVoc"][j-nuvoc] page is a useful reference for J's vocabulary.

j values may be integers, arrays, or 2-dimensional matrices; higher-rank matrices are not
implemented. a small number of verbs and adverbs are provided. variables may be defined.

**monadic verbs**
* `i.` "idot" generates a value
* `|:` "transpose" rotates its argument
* `#` "tally" counts the elements in its argument
* `$` "shape" returns a value containing the dimensions of its argument
* `[` "same" returns the given value
* `]` "same" returns the given value
* `>:` increments the given value

**dyadic verbs**
* `+` returns the sum of its two arguments
* `*` returns the product of its two arguments
* `[` "left" returns the left value
* `]` "right" returns the right value

**monadic adverbs**
* `/` "insert" places a dyadic verb between items of its argument
* `\` "prefix" returns successive prefixes of its argument

**dyadic adverbs**
* `/` "table" returns a table of entries using a dyadic verb and two arguments
* `\` "infix" applies a verb to successive parts of its right-hand argument

variables are assigned using `=:`. variable names may only contain lowercase ASCII `a-z`
characters, numeric `0-9` characters, and `_` underscores. variable names must begin with a
lowercase ASCII `a-z` character.

#### 📂 project structure

the code within this project is structured like so:

```
.
├── src
│  ├── a.rs                array
│  ├── j.rs                `j` library crate
│  ├── p.rs                prelude
│  ├── r.rs                lexing and parsing
│  ├── s.rs                symbol table
│  └── x.rs                interpreter binary
└── tests
   └── t.rs                test suite
```

# 📜 implementing j; an essay

the rest of this document is an essay discussing the experience of writing software, using Rust,
in a voice inspired by Arthur Whitney's style of C.

### 🌷 an introduction

the purpose of building j was not merely to implement an array language in Rust. it was important
to me that i did not write this software in the idiomatic style enforced by `cargo fmt`.
conciseness should not come from retroactive search-and-replace, but from an honest attempt at
adopting and recreating the _mindset_ of people who write software like this.

in order to get a concrete sense of what this looks like, let's begin by reading a few snippets
from the Incunabulum, an open-source implementation of k, and from j.

#### 📜 the incunabulum

conventional formatting of the Incunabulum's `main` function would look something like this:

```c
int main()
{
	char s[99];
	Array a;
	while (gets(s))
    {
		a = execute(parse(s));
		print(a);
    }
}
```

for C programmers, this should look like a familiar structure for the main event loop of an
interpreter; we read an input, parse it, and then execute that statement.

in the Incunabulum, Arthur Whitney wrote that loop in this single line:

```c
main(){C s[99];while(gets(s))pr(ex(wd(s)));}
```

#### 🔬 ngn k

ngn k is another commonly cited example of this style at its most extreme. excluding header
imports and type aliases, its test runner fits in these 12 lines:

```c
S C*mm(C*s,C**e)_(I f=open(s,0);ST stat h;fstat(f,&h);L n=h.st_size;C*r=mmap(0,n,1,2,f,0);cl(f);*e=r+n;r)
S I nl(C*s,I n)_(C*p=s;I i=0;W(i<n,I(s[i]==10&&i<n-1&&s[i+1]==32,*p++=';';i+=2)E(*p++=s[i++]))p-s)
S I t(C*s,I n)_(/*wr(1,".",1);*/P(*s=='/'||*s==10,0)
 C*u=strstr(s," /");P(!u,wr(1,"bad test: ",10);wr(1,s,n);-1)
 C*a[]={"./k",0};I p[4];pipe(p);pipe(p+2);
 I c=fork();P(!c,dup2(*p,0);dup2(p[3],1);i(4,cl(p[i]))setrlimit(RLIMIT_CPU,(ST rlimit[]){{1,1}});exit(execve(*a,a,0));0)
 cl(*p);cl(p[3]);wr(p[1],s,u-s);wr(p[1],"\n\\m\n",4);cl(p[1]);
 C o[256];L m=0;W(1,L k=read(p[2],o+m,SZ o-1-m);B(k<=0)m+=k;B(m<SZ o-1))
 cl(p[2]);m=nl(o,m);u+=3;kill(c,SIGKILL);P(s+n==u+m&&!strncmp(o,u,m),1)
 wr(1,"\nfail: ",6);wr(1,s,n);wr(1,o,m);wr(1,"\n",1);-1)
I main()_(setbuf(stdout,0);/*pr("unit tests\n");*/C*e,*s=mm("t/t.k",&e);I n=0,f=0;
 W(s<e,C*u=strchr(s,10)+1;I r=t(s,u-s);n+=!!r;f+=r<0;s=u)P(f,pr("\nfail %d/%d\n",f,n);1)pr("\n");0)
```

> ❗ **NB:** do not worry about understanding this snippet.

i had always been fascinated by the fact that this is technically the same language as more
orthodox dialiects of C, such as K&R, GNU, or BSD.

j follows this spirit and style, in order to investigate whether "Whitney Rust" is possible, what
it might teach us about Rust as a programming language, and to learn what writing software in this
voice feels like, and why people do it.

#### 🐣 j

**🔁 main loop**

above we looked at the Incunabulum's main event loop. here is j's equivalent entrypoint:

```rust
mod p;use{p::*,j::*,std::io::Write};
fn main()->R<()>{let mut st=ST::default();                                 // define symbol table
  let prompt  =|   |{print!("  ");std::io::stdout().flush()?;ok!()};       // (callback) print whitespace
  let read    =|_  |{let mut l=S::new();stdin().read_line(&mut l)?;Ok(l)}; // (callback) read input
  let mut eval=|s:S|{eval(&s,&mut st)};                                    // (callback) read and evaluate once
  let print   =|a:A|{println!("{a}")};                                     // (callback) print array
  loop{prompt().and_then(read).and_then(&mut eval)?.map(print);};          /* !!! main event loop !!! */ }
```

**🏃 verbs**

the core of the four dyadic verbs `[`, `]`, `+`, and `*` is shown below. the definitions of the `A`
array type and the `D` dyadic verb enum are included for reference.

```rust
pub enum D        {Plus,Mul,Left,Right}
pub struct A<X=MI>{/**columns*/ pub m:U,      /**rows*/  pub n:U,
                   /**data*/        d:*mut u8,/**layout*/l:L,
                   /**memory state*/i:PD<X>,                    }
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
```

(_NB: we will talk more about the shorthand type aliases and macros in scope later in this essay_)

## 💭 on brevity; readability depends on who is reading

thus, array programming languages are somewhat notorious for their terseness. this terseness often
extends to the underlying implementation of these languages as well. before we continue any
further, i'd like to cite a snippet from `@xpqz`'s excellent introduction to the K programming
language:

> K, like its Iversonian siblings APL and J, values conciseness, or perhaps we should say
> terseness, of representation and speed of execution.
>
> [A]ccusations of “unreadable”, “write-only” and “impossible to learn” are leveled at all
> Iversonian languages, k included. [..] **Readability is a property of the reader, not the
> language.**

- [_"Why k?"_][why-k] _(emphasis added)_

to restate this point bluntly, we will not spend time in this essay humoring questions about the
aesthetic or practical validity of these languages. real people wake up, and solve real problems
with array programming languages. whether this is the right paradigm for you is not dependent on
your first impression, but upon what problems you are trying to solve, and whom you communicate
with when solving them.

## 🌈 how broad is our imagination?

programming languages used in industry today are descendents of a few shared ancestors, most
commonly C. exceptions may draw from other languages such as ML, Lisp, Smalltalk, or Fortran, but
even these all share some unspoken consensus regarding whitespace, loop indentation, symbol names,
and the like.

many readers approach array languages with some preconceptions of what code should look like.
we as software engineers today have conservative ideas of what programming languages
can look like, compared to the variety found in written languages around the world.

### ➰ compute a cumulative sum in imperative programming languages

let's find the sum of integers `1..100` in a few programming languages.

**note** source for these can all be found in the `sums/` directory of this repository. we will exclude
unrelated syntactic overhead such as defining a `main` function, in order to focus on the core
logic of computing this sum.

**C**

```c
int sum = 0;
for (int i = 0; i <= 100; i++)
{
    sum += i;
}
printf("%d", sum);
```

**POSIX Shell**

```sh
SUM=0
for i in $(seq 100);
do
	SUM=$(expr $SUM + $i)
done
echo $SUM
```

**Julia**

```julia
sum = 0
for i=1:100
	global sum = sum + i
end
print(sum)
```

**Rust**

```rust
let mut sum = 0;
for i in 1..=100 {
    sum += i;
}
println!("{sum}");
```

these are strikingly similar! consider that Julia appeared roughly 40 years after C, or that the
Bourne shell is a command-line interpreter rather than a programming language. despite that,
most of the differences between these examples are syntactic minutia:
* in C, we must declare the type of the variable, an `int`
* in a shell script, we must perform our arithmetic inside of a subshell, using the `expr` builtin
* in Julia, we must specify that our assignment refers to the global `sum` variable
* in C and Rust, loops are wrapped within `{` and `}` curly braces; in Julia and sh, `end` and
`done` keywords are used.

otherwise, all of these programs share a common backbone, expressed in the following pseudocode:

```
sum = 0
for i in 1..100:
    sum += i
print sum
```

#### λ find a cumulative sum in a functional language

this exercise could be repeated with a collection of various functional languages, but a similar
pattern can be found. programs declare a sequence or iterator and use a "fold" operation to find
the sum.

at its most explicit, this would look something like the following Rust program:

```rust
fn main() {
    let sum = (1..=100).fold(0, std::ops::Add::add);
    println!("{sum}");
}
```

or, using the `Iterator::sum` helper:

```rust
fn main() {
    let sum: u32 = (1..=100).sum();
    println!("{sum}");
}
```

**⤆ a refresher on function composition: you are used to reading from right to left**

suppose we have a value `x`, and two functions, `f(x)` and `g(x)`. "function composition" refers
to the act of finding the resulting value when `g` is provided the output of `f(x)`. this may be
written out as `g(f(x))`.

this notation represents the precedence of functions such that you would read this expression
from right to left. `x` is first provided to `f`, whose output is subsequently passed to `g`.

mathemeticians alternately use the `∘` operator to represent this. this composition of `f` and
`g` can also be written out as `f ∘ g`. some languages such as F# or Elixir provide a "pipeline"
operator to facilitate this style, and other languages like Haskell may be written in "point-free"
syntax such as this. Rust's `.` operator functions are similar sugar, passing its left-hand side
as the first parameter to the associated method named in the right-hand side.

functional languages' solutions to this problem will fit into one of two syntactic structures,
shown in pseudo-code below:

```
sum(seq(100))
```

```
100 |> seq |> sum
```

neither of these approaches are incorrect, but it is worth pointing out that many programmers are
already quite familiar with the experience of reading expressions from right to left! additionally,
many are also familiar with the idea of programming _tacitly,_ wherein variable names are not
assigned to intermediate values (_the `i` in the for-loops above_).

#### 󰘨 find a cumulative sum in an array language

here are two solutions to the same problem in J, and its related cousin K.

**K**

```
+/1+!100
```

**J**

```
+/1+i.100
```

_(NB: the "increment" operator is implemented later in this essay.)_

**🎳 breaking it down**

the K solution performs this same computation in 8 characters. let's break down how this works
bit-by-bit, starting with the lattermost 6 characters `1+!100`.

here are the definitions for the `!` and `+` verbs, from the `kona` K interpreter's help pages:

```
! monadic  enumerate. !4 yields 0 1 2 3
+ dyadic   plus. add numbers together
```

taken together, the expression `1+i.4` adds `1` to each element of the array `0 1 2 3`, yielding
`1 2 3 4`.

`/` is a kind of "adverb." in traditional human languages, an adverb is a part of speech used to
apply an adjective to a verb. or in other words, it describes how a verb is/was performed. this
same concept holds roughly true for J and K's adverbs.

adverbs and [gerunds][j-gerunds] are very similar to higher-order functions. a higher-order
function is a function that either accepts as an argument, or returns, another function. these
constructs provide a way for programmers to abbreviate or abstract over common control flow
patterns. J refers to the `/` adverb in this statement as "insert".

the "insert" adverb places the provided dyadic (`+`) operator between the elements of its argument.
thus, `+ / 1 2 3` is equivalent to `1+2+3`. notice that this is, syntax notwithstanding, the same
idea as saying `fold()`

so, the expression above has the following structure:

```
+ / 1 + ! 100
          ┝┳┥
           ┗━━━━━ the number 100
        ┝━┳━┥
          ┗━━━━━━ the sequence of numbers 0 through 99
    ┝━━━┳━━━┥
        ┗━━━━━━━━ the sequence of numbers 1 through 100
┝┳┥
 ┗━━━━━━━━━━━━━━━ find the sum of the given argument
┝━━━┳━━━━━━━┥
    ┗━━━━━━━━━━━━ add each of the numbers from 1 through 100
```

these programs share the same structure, save that `i.` is the verb for generating sequences,
rather than K's `!`. importantly, we can see a shared syntactic lineage here.

###   language categories

the united states government groups languages into categories from 1-5, ranking how difficult they
are to learn. category 1 languages are considered "easy" to learn, while a category 5 language
will take many hours before a student is considered fluent.

there is a catch: this scale measures the perceived difficulty for _native English speakers_.

the true difficulty of learning a new language is fundamentally entangled with what languages a
student is previously familiar with. A Spanish speaker may be able to quickly learn Portuguese,
but a language like Cantonese might include novel concepts such as semantically meaningful tone
that would trip up such a student. in much the same way, J would be rather easy for a K programmer
to learn and vice-versa.

in truth, i think that the difficulty of learning array languages is overestimated. while this may
seem to stem from incongruities in notation, i think this reputation is ultimately about larger
differences in people's philosophies and preconceptions about human-computer interfaces.

so, why _do_ so many programming languages look so similar?

#### 🤨 "strangeness budget"

we can find an illustrative story in the development of Rust's syntax for asynchrony. to briefly
summarize, Rust opted to use "postfix" notation when awaiting a `Future`. this results in code
that looks like:

```rust
let bytes = client
    .send(request)
    .await
    .map(Response::into_body)?;
```

in other languages that use a traditional `await` keyword such as JavaScript, this might look
something like:

```javascript
let bytes = (await client.send(request)).into_body();
```

many discussions about the validity of this approach have already been borne out at
[great][rust-57640] [length][rust-50547]. i'll point to [this][ceej-postfix] excellent write-up
about the benefits of postfix `await` if you are interested in reading further. suffice to say,
this decision was controversial, because it strayed outside of what some people considered
reasonable syntax for asynchrony.

Steve Klabnik [wrote][klabnik-budget] about this phenomenon:

> [I]t’s important to be considerate of how many things in your language will be strange for your
> target audience, because if you put too many strange things in, they won’t give it a try.
>
> You can see us avoiding blowing the budget in Rust with many of our syntactic choices. We chose
> to stick with curly braces, for example, because one of our major target audiences, systems
> programmers, is currently using a curly brace language. Instead, we spend this strangeness
> budget on our major, core feature: ownership and borrowing.

in other words, in order to define and manage a strangeness budget, as with the idea of
"categories" for human languages, you must identify who your prospective students are. as Steve
noted, it was pragmatic and reasonable for Rust to focus on this demographic because many of its
early adopters would be C or C++ programmers.

our summing exercise above illustrated how similar different programming languages' looping
constructs are. ultimately, each new programming language is incentivized _not_ to challenge
people's notions of loops.

Aaron Hsu has remarked in his [talks][hsu-apl] about APL that the people who have the most
trouble learning languages like APL are often _computer science students_. we learn more than
syntax or grammer when studying computer science: for better and for worse, we also internalize
conventions of _thought._

readability is a property of the reader, indeed!

## 🍄 my experience

so, you ask, how was it?

hopefully at this point i've convinced you that this is an internally consistent programming
paradigm, even if it does not seem like your personal cup of tea. i'll be honest, at the outset
of this project it did not seem like my cup of tea either.

after spending time building a piece of software in this style however, i grew to like it
far more than i expected i would. in no particular order, let's gloss through some thoughts about
this experience.

### 🌐 brevity amplifies local reasoning

codebases for real-world production software are often quite large. most include tens of thousands
lines of code, and it is not uncommon for this number to reach the hundreds of thousands, or even
millions.

**lexical sprawl introduces a high amount of non-local reasoning into our systems.** with that,
we introduce a need for other specialized tooling: text editors with the ability to "fold"
sections of text out of view, terminal multiplexers with tab and window management facilities,
language servers to help find references to a given variable, formatting tools to maintain a
consistent syntactic style, the list goes on.

when working on j, i found that i spent about the same amount of time reading
through my existing code to introduce a new feature, but reading became a passive activity. i
no longer needed to scroll up and down, or jump to function definitions elsewhere. **my cognitive
function was no longer divided between reading and traversal.** i could instead open a
file, lean back, and read through the entirety of a subsystem without needing to manually interact
further.

in contrast, concise code has the effect of maximizing the amount of code that may be included in
local reasoning.

### 🐘 sufficient brevity implies a DSL

software written in this style includes a "_prelude_" of sorts, defining various shorthand forms.
these preludes go beyond just type aliases like `typedef char C` or `typedef long I`, however.
C's preprocessor is often leveraged to provide abstractions for keywords, function signatures,
or even **control flow**.

the Incunabulum's prelude is shown below. it defines an `R` shorthand for `return`ing a value,
`DO` to perform an operation across the length of an array.

```c
#define P printf
#define R return
#define V1(f) A f(w)A w;
#define V2(f) A f(a,w)A a,w;
#define DO(n,x) {I i=0,_n=(n);for(;i<_n;++i){x;}}
```

Kona, an open-source implementation of the k3 programming language, includes an almost verbatim
copy of this same macro:

```c
#define DO(n,x) {I i=0,_i=(n);for(;i<_i;++i){x;}}
#define DO2(n,x){I j=0,_j=(n);for(;j<_j;++j){x;}}
#define DO3(n,x){I k=0,_k=(n);for(;k<_k;++k){x;}}
```

as another example, Kona defines some control-flow macros for early returns, based on some predicate
condition.

```c
#define R return
#define P(x,y) {if(x)R(y);}
#define U(x) P(!(x),0)
```

shorthand for common control-flow like early returns is tremendously useful. Rust has the `?`
operator for this very reason! the next snippet shows some ngn k's equivalent shorthand notation
for loops, conditional statements, and switch statements.

```c
#define  W(x,a...) while(x){a;}
#define  B(x,a...) I(x,a;break)
#define  P(x,a...) I(x,_(a))
#define  I(x,a...) if(x){a;}
#define    J(a...) else I(a)
#define    E(a...) else{a;}
#define SW(x,a...) switch(x){a}
```

Rust was just as capable of defining such a prelude: `r!()` could be used to perform early
returns, `b!()` could perform heap allocations, `R<T>` served as a shorthand for a fallible
operation resulting in `T`, and similar type aliases `C` or `I` were defined for characters and
integers.

j's prelude looks like this:

```rs
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
```

in my practical experience, this was also a pleasant toolbox to maintain. "Don't Repeat Yourself"
is an old adage among programmers, and this worked well to that effect. when i began to recognize
a pattern of some sort, i could define a shorthand for it.

LISP programmers have a mantra that code is data, and data is code; indeed, Kona's README notes
that LISP was an important influence on the K language. along the same lines, this style puts a
heavy emphasis on metaprogramming facilities. code is also syntax, and syntax is code.

**brevity mandates the construction of a domain-specific language (DSL) in which a piece of
software can then be written.** this style of hyper-succint code is ultimately a dialect to be
embedded _within_ a "host" language.

### 🦀 brevity is not a mutually exclusive property

writing concise Rust code did not detract from the traditional benefits of the language.

rather than writing this as a 1:1 direct translation of the [Incunabulum][incunabulum], i was
able to follow familiar idioms when implementing j, and found myself enjoying the usual benefits
of writing software in Rust.

**NB:** this next section assumes some previous familiarity with Rust's type system.

**🌳 abstract syntax tree**

as a straightforward example, this snippet of `src/r.rs` shows the definition of j's abstract
syntax tree (AST). an AST is the structured representation of statements in a language, which
most compilers and interpreters implement in some form or another.

`SY` is a newtype wrapper around a `String`. they're tremendously useful. see
["New Type Idiom"][api-guidelines-newtype] in the Rust API guidelines for more information on this
pattern.
the `D` and `M` structures represent dyadic and monadic verbs: `+`, `*`, `i.`, and so forth.
the `Yd` and `Ym` are structures are monadic and dyadic adverbs, `/` and `\`.

`N` is a recursive structure that uses these to represent a statement in j.

```rust
/**symbol            */pub struct SY(S);
/**dyadic verb       */pub enum D {Plus,Mul,  Left, Right             }
/**monadic verb      */pub enum M {Idot,Shape,Tally,Transpose,Same,Inc}
/**dyadic adverb     */pub enum Yd{/**dyadic `/` */      Table ,
                                   /**dyadic `\` */      Infix }
/**monadic adverb    */pub enum Ym{/**monadic `/`*/      Insert,
                                   /**monadic `\`*/      Prefix}
/**ast node          */pub enum N {/**array literal*/    A{a:A},
                                   /**dyadic verb*/      D{d:D,l:B<N>,r:B<N>},
                                   /**monadic verb*/     M{m:M,o:B<N>},
                                   /**dyadic adverb*/    Yd{yd:Yd,d:D,l:B<N>,r:B<N>},
                                   /**monadic adverb*/   Ym{ym:Ym,d:D,o:B<N>},
                                   /**symbol*/           S{sy:SY},
                                   /**symbol assignment*/E{sy:SY,e:B<N>}}
```

while the monadic and dyadic verbs are simple enumerations, notice that the the `N` node contains
heterogenous payloads for each variant. outlining the benefits of pattern matching and algebraic
data types (ADTs) is out-of-scope for this essay, but in short: this approach helps prevent
invalid fields from being accessed or written to. interacting with the `l` and `r` fields will
cause a compilation failure, _unless_ we are within a block of code that has properly matched
against an `A::N` value.

**🔐 interfaces with guardrails**

here are some abbreviated snippets of `src/a.rs`. first, we define a collection of "marker"
types, to indicate whether the memory of an array has been initialized yet. this uses a "sealed"
trait; see the [API Guidelines][api-guidelines-futureproof] for more information on this pattern.

```rust
// src/a.rs
/**sealed trait, memory markers*/pub trait MX{} impl MX for MU {} impl MX for MI {}
/**marker: memory uninitialized*/#[derive(CL,DBG)]pub struct MU;
/**marker: memory initialized*/  #[derive(CL,DBG)]pub struct MI;
```

next, we use these marker types and `PhantomData` to mark an array as having memory that is either
initialized (`MI`), or uninitialized (`MU`). if no generic is given to `A<T>`, i.e. `A`, it will
use `MI` by default.

```rust
use super::*; use std::marker::PhantomData as PD;
#[derive(DBG)]pub struct A<X=MI>{/**columns*/ pub m:U,      /**rows*/  pub n:U,
                                 /**data*/        d:*mut u8,/**layout*/    l:L,
                                 /**memory state*/i:PD<X>,                    }
```

now, what benefits does this provide?

it means that we can use `impl A<MI>{}` blocks to "gate" public interfaces, so that array access
cannot be performed until an array has been initialized. `impl<X:MX> A<X>` may in turn be used to
provide interfaces that apply to _both_ uninitialized and initialized arrays.

```rust
impl A<MI>{
 pub fn get(&self,i:U,j:U)->R<I>{Ok(unsafe{self.ptr_at(i,j)?.read()})}
}
impl<X:MX> A<X>{
  pub fn set(&mut self,i:U,j:U,v:I)->R<()>{unsafe{self.ptr_at(i,j)?.write(v);}Ok(())}
  pub(crate)fn ptr_at(&self,i:U,j:U)->R<*mut I>{self.ptr_at_impl(i,j,Self::index)}
}
```

...and finally, it means that we may mark certain interfaces as safe, or unsafe. for example,
`A::init_with` provides a safe interface to initialize the memory of an array, using a callback
that is given the position `(i,j)` of each cell.

conversely, `A::set` may be used to manually initialize each position of the array, but places the
onus upon the caller to determine whether or not each cell has been initialized. thus, `A::finish`
is an unsafe interface, and must be called within an `unsafe{}` block.

```rust
impl A<MU>{
  pub fn new(m:U,n:U)->R<Self>{Self::alloc(m,n).map(|(l,d)|A{m,n,d,l,i:PD})}
  pub fn init_with<F:FnMut(U,U)->R<I>>(mut self,mut f:F)->R<A<MI>>{let(A{m,n,..})=self;
    for(i)in(1..=m){for(j)in(1..=n){let(v)=f(i,j)?;self.set(i,j,v)?;}}Ok(unsafe{self.finish()})}
  pub unsafe fn finish(self)->A<MI>{std::mem::transmute(self)}
}
```

now, we can compare this to how this code might be formatted in common Rust, without such compact
formatting and such short symbols:

```rust
impl Array<MemoryUninit>{
    pub fn new(m: usize, n: usize) -> Result<Self, anyhow::Error>{
        let (l, d) = Self::alloc(m, n)?;
        let a = A { m, n, d, l, i:PD };
        Ok(a)
    }

    pub fn init_with<F>(mut self, mut f: F) -> Result<Array<MemoryInit>, anyhow::Error>
    where
        F: FnMut(usize, usize) -> Result<u32, anyhow::Error>
    {
        for i in 1..=self.m {
            for j in 1..=self.n {
                let v = f(i, j)?;
                self.set(i, j, v)?;
            }
        }
        let a = unsafe{ self.finish() };
        Ok(a)
    }

    pub unsafe fn finish(self) -> Array<MemoryInit> {
        std::mem::transmute(self)
    }
}
```

**these two snippets are not any mechanically different!** it bears consideration that this
terse style did not prevent me from using familiar idioms. "_Whitney C_" may be a grand departure
from other variants of C, but it _is_ still ultimately a dialect of C. "_Whitney Rust_" is also,
at the end of the day, a dialect of Rust.

## 🏠 brevity is an architectural principal

a core lesson i learned by building j is that this is extensive pursuit of brevity is about much
more than syntactic brevity for cosmetic reasons. this is a kind of brevity that is also an
architectural principal, and a mode of cognition.

working like this had a perceptible impact on how i worked. it affected the tools i used, how i
used them, and how i thought.

**🔨 simple workflows**

working with succinct code meant that i could rely on succinct workflows. when files are this
short, you can print them with `cat`. remarkably simple. let's look at array indexing as another
example.

m×n arrays `A` have m rows and n columns, and are indexed with 1-based coordinates `(i,j)`. the
top-left corner serves as the origin `(1,1)`. there is a documentation comment in `src/a.rs` on
line 4 that shows this simple diagram:

```
[a_11, a_12, ... a_1n]
[a_21, a_22, ... a_2n]
[...., ...., ... ....]
[a_m1, a_m2, ... a_mn]
```

when implementing new features, _especially_ when interacting with raw memory, i found that it was
tremendously helpful to open this comment in a split window within my text editor, neovim.

neovim has a `-c` option that may be used to run commands after opening a file. `+n` may be used
to open a file at a particular line number. thus, nvim `+4 src/a.rs -c split -c res 4` would open
the core `a.rs` array logic, with a split pane showing me this reference for my memory indexing
strategy.

rust's inline unit tests allowed me to often work with an entire subsystem _and_ its
respective test suite on my screen at the same time. it is hard to overstate how novel and exciting
this felt.

**🌲 seeing the forest**

i found that brevity changed the economy of space in my project away from _lines_ of code, into a
two-dimensional economy of characters. while this use of horizontal alignment echoed what i have
personally seen in plenty of C codebases, within succinct code i found that i was able to highlight
commonalities and differences in higher-level parts of my software architecture: control flow,
functions, etc.

as an example, the `A` array type has two methods, `index` and `index_uc`, to convert 1-based
`(i,j)` coordinates to the corresonding index of the raw allocated memory. `index` will check that
the coordinates are in bounds. for performance reasons, the "_unchecked_" variant will elide this
bounds check.

look how clearly a succinct style highlights that difference:

```rust
impl A{
  fn index   (&self,i:U,j:U)->R<U>{self.oob(i,j)?;let A{m,n,..}=*self;let(i,j)=(i-1,j-1);Ok((i*n)+j)}
  fn index_uc(&self,i:U,j:U)->R<U>{               let A{m,n,..}=*self;let(i,j)=(i-1,j-1);Ok((i*n)+j)}
}
```

**🔎 brief reviews**

cratelyn/j#10 introduces a new monadic verb, `>:`, to increment its given noun.

`git show` has an option `--word-diff-regex` that can be used to control what a "word" is in the
diff output. so `git show --word-diff-regex=.` is a way to see a per-character diff. using this,
and the `-w --ignore-all-space` flag to ignore whitespace changes, this PR fits in one screen:

![`62edb1f8061cdcf859f9335e13fb6c104718a4e5`](./10-diff.png)

> ❗ you can run `git show 62edb1f --ignore-all-space --word-diff-regex=.` if you would like to
> see this in a local clone of this repository.

you might naturally as a reviewer take some time to read through this change, and decide whether
or not it looks good to you. this style does not mean that there is somehow _less code to review._
it does mean however, that you can see all of these changes at once.

**💔 tooling incongruities**

cratelyn/j#3 is an example of a very simple bugfix, fixing an off-by-one error for the `i.` verb.
it replaces a `j` with a `(j-1)` in a particular expression. here is the diff, again using
`--word-diff-regex=.`:

![`; git show --word-diff-regex=. --oneline fb72462
fb72462 (origin/idot-should-start-from-zero, idot-should-start-from-zero) 🐛 bug: i. sequences should start from zero
diff --git a/src/a.rs b/src/a.rs
index 38f1d7b..f535c2c 100644
--- a/src/a.rs
+++ b/src/a.rs
@@ -125,7 +125,7 @@ use super::*; use std::marker::PhantomData as PD;
/**monadic verbs*/impl A{
  pub fn m_idot(self)->R<A>{let(a@A{m,n,..})=self;let gi=|i,j|a.get(i,j)?.try_into().map_err(E::from);
    if let(1,1)=(m,n){let(m,n)=(1,gi(1,1)?);let(mut o)=A::new(1,n)?;
      for(j)in(1..=n){o.set(1,j,{+(+}j{+-1)+}.try_into()?)?;}Ok(unsafe{o.finish()})}
    else if let(1,2)=(m,n){let(m,n)=(gi(1,1)?,gi(1,2)?);
      let(mut v)=0_u32;let(f)=move |_,_|{let(v_o)=v;v+=1;Ok(v_o)};A::new(m,n)?.init_with(f)}
    else{bail!("i. {m}x{n} not supported")}}
diff --git a/tests/t.rs b/tests/t.rs
index bb3b691..fa86b41 100644
--- a/tests/t.rs
+++ b/tests/t.rs
@@ -20,7 +20,7 @@
    #[test]fn $f()->R<()>{let(a@A{m:1,n:1,..})=eval_s($i)? else{bail!("bad dims")};eq!(a.as_slice()?,&[$o]);ok!()}}}
  t!(tally_scalar,"# 1",1);t!(tally_1x3,"# 1 2 3",3);t!(tally_3x3,"# i. 3 3",9);
} #[cfg(test)]mod idot{use super::*;
  #[test]fn idot_3()->R<()>{let(a)=eval_s("i. 3")?;eq!(a.m,1);eq!(a.n,3);eq!(a.as_slice()?,&[{+0,+}1,2[-,3-]]);ok!()}
  #[test]fn idot_2_3()->R<()>{let(a)=eval_s("i. 2 3")?;eq!(a.m,2);eq!(a.n,3);let o:&[&[I]]=&[&[0,1,2],&[3,4,5]];
    eq!(a.into_matrix()?,o);eq!(a,o);ok!()}
  #[test]fn idot_3_2()->R<()>{let(a)=eval_s("i. 3 2")?;eq!(a.m,3);eq!(a.n,2);let o:&[&[I]]=&[&[0,1],&[2,3],&[4,5]];`](./3-diff.png)

> ❗ you can run `git show fb72462 --oneline --word-diff-regex=.` if you would like to see this in
> a local clone of this repository.

this diff, when shown with `--oneline` and `--word-diff-regex=.`, is 1442 characters.
only 24 characters are the changes themselves (_including `{+` and `+}` markers_). only 16
characters would be needed to print the names of the files shown.

this is of course, napkin math, but that means that more than 97% of the diff is _context._ the
homogeneity of so many programming languages in industry today doesn't just affect the way we
write code. these preconceptions become assumptions that are baked into the tools we use.
thus, git is showing me "context" that presumes we are measuring of code in terms of "lines".

## 🖤 conclusion

overall, i enjoyed working in this style. it provided me with a fresh perspective about how i
use the computer, what language can look like, and how notation affects the way we think.
if you have ever been curious about array programming languages, i recommend you give them a try.

---

### 🔗 works cited

* ["Future Proofing"](https://rust-lang.github.io/api-guidelines/future-proofing.html)
* ["New Type Idiom"](https://doc.rust-lang.org/rust-by-example/generics/new_types.html)
* ["Incunabulum"](https://code.jsoftware.com/wiki/Essays/Incunabulum)
* ["J: Gerunds And Atomic Representation"](https://code.jsoftware.com/wiki/Vocabulary/GerundsAndAtomicRepresentation)
* ["J: Getting Started"](https://code.jsoftware.com/wiki/Guides/Getting_Started)
* ["J: NuVoc"](https://code.jsoftware.com/wiki/NuVoc)
* ["The language strangeness budget"](https://steveklabnik.com/writing/the-language-strangeness-budget)
* ["Why K"](https://xpqz.github.io/kbook/Introduction.html#why-k)
* ["Why Rust’s postfix await syntax is good"](https://blog.ceejbot.com/posts/postfix-await/)
* rust-lang/rust#57640
* https://github.com/rust-lang/rust/issues/57640
* https://github.com/rust-lang/rust/issues/50547
* ["Does APL Need A Type System?"](https://www.youtube.com/watch?v=z8MVKianh54)

[api-guidelines-futureproof]: https://rust-lang.github.io/api-guidelines/future-proofing.html
[api-guidelines-newtype]: https://doc.rust-lang.org/rust-by-example/generics/new_types.html
[incunabulum]: https://code.jsoftware.com/wiki/Essays/Incunabulum
[j-gerunds]: https://code.jsoftware.com/wiki/Vocabulary/GerundsAndAtomicRepresentation
[j-getting-started]: https://code.jsoftware.com/wiki/Guides/Getting_Started
[j-nuvoc]: https://code.jsoftware.com/wiki/NuVoc
[klabnik-budget]: https://steveklabnik.com/writing/the-language-strangeness-budget
[why-k]: https://xpqz.github.io/kbook/Introduction.html#why-k
[ceej-postfix]: https://blog.ceejbot.com/posts/postfix-await/
[rust-57640]: https://github.com/rust-lang/rust/issues/57640
[rust-50547]: https://github.com/rust-lang/rust/issues/50547
[hsu-apl]: https://www.youtube.com/watch?v=z8MVKianh54
