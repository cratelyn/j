/* One summer weekend in 1989, Arthur Whitney visited Ken Iverson at Kiln Farm
 * and produced—on one page and in one afternoon—an interpreter fragment on the
 * AT&T 3B1 computer. I studied this interpreter for about a week for its
 * organization and programming style; and on Sunday, August 27, 1989, at about
 * four o'clock in the afternoon, wrote the first line of code that became the
 * implementation described in this document.
 *
 * - source: https://code.jsoftware.com/wiki/Essays/Incunabulum
 */

/* a summer weekend in 2023, we are taking another look at this delightful
 * single-page interpreter fragment. this program is unchanged semantically,
 * but it has had its whitespace changed s.t. it now aligns with a more
 * "conventional" notion of C formatting.
 */

/* one summer weekday, we continue our quest. next, we will replace the
 * preprocessor macros in this program. some of them, such as `printf` or
 * `return`, are simply shorthand aliases.
 *
 * we won't yet touch `V1` or `V2`, but the others can straightforwardly
 * replaced.
 */

/* now, with whitespace expanded, and other shorthand sugar expanded,
 * we will make some minor changes:
 *   - `Wimplicit-function-declaration`
 *   - `Wimplicit-int`
 *
 * we also move away from the K&R argument list style, and use ANSI parameters.
 *
 * finally, we replace the notorious `gets` function.
 *
 * at this stage, we still have some warnings remaining related to
 * integer/pointer conversion. before we dive deeper into that, we will inline
 * some single-statement functions, and tackle the remaining preprocessor
 * macros: `V1`, `V2`, and `DO`.
 */

/* next, we expand those other macros: `V1`, `V2`, and `DO`. after this
 * transformation, our remaining compiler errors (mentioned above) are no
 * longer lying within macro calls.
 *
 * this program is beginning to read like a fairly familiar, if still
 * relatively terse, piece of C code.
 */

/* so, this is where we should unveil a bit of a surprise: the
 * interpreter fragment, as provided, is a bit unreliable.
 *
 * reading the source, it looks like we have support for a small subset of J
 * verbs, whose definitions we'll pull from J software's reference[1]:
 *
 * the monads:
 *
 * `+`: id, returns the value given to it.
 * `{`: in J, this is the cartesian product. here, it is called size.
 * `~`: in J, this is called "reflex". here, it is "iota".
 * `<`: box, allows a collection of items to be treated as a single entity.
 * `#`: in J, this would be called "tally", and counts items in a collection.
 *      here, this is called "sha".
 *
 * the dyads:
 *
 * `+`: plus, sums two operands.
 * `{`: from, indexes an item from an array of items.
 * `~`: in J, this is called "reflect". here, this symbol appears to
 *      correspond to `find`.
 * `<`: in J, this would be "less than". here, this seems unimplemented.
 * `#`: in J, this would be called "copy", and replicates items in a
 *      collection. here, it seems to be called "rsh".
 * `,`: appends items to a collection.
 *
 * [1]: https://www.jsoftware.com/books/pdf/brief.pdf
 *
 * if we compile the interpreter fragment, and run it, we quickly encounter
 * segfaults:
 *
 * ```
 * ; gcc --version | head -n 2
 * gcc (GCC) 13.1.0
 * Copyright (C) 2023 Free Software Foundation, Inc.
 *
 * ; ./a.out
 * 1 + 2
 * zsh: segmentation fault (core dumped)  ./a.out
 *
 * ; ./a.out
 * < 1 2
 * zsh: segmentation fault (core dumped)  ./a.out
 *
 * ; ./a.out
 * # 1 2 3
 * zsh: segmentation fault (core dumped)  ./a.out
 * ```
 *
 * no shame in that by the way; as the original description noted, this was
 * written within a single afternoon. our purpose is not to criticise the
 * original work, but to understand it.
 *
 * instead of feeding this program "real" J statements, we can look at the 
 * source and get a picture of what this program *will* accept. `vd` and
 * `vm` contain function pointers to verbs' dyadic and monadic forms, and
 * `st` corresponds to the interpreter's symbol table.
 *
 * the body of `ex` is important for discerning that, and where we see
 * something related to variable assignment; if we see an alphabetic character
 * on the left-hand side, we assign `st[..]` to the result of the remaining
 * expression.
 *
 * so, as long as we only deal with single-letter symbols `a` through `z`,
 * and refrain from spaces, we can in fact play around a bit!
 *
 * ; ./a.out
 * a=3
 * 
 * 3
 * +a
 * 
 * 3
 * ~a
 * 3
 * 0 1 2
 *
 * b=a
 * 
 * 4
 * b
 *
 * 4
 *
 * now, even this wasn't fully consistent for me. for example, repeated use
 * of the `~` operator on a single array would crash, and variable assignment
 * seemed inconsistent as well:
 *
 * ; ./a.out
 * a=1
 * zsh: segmentation fault (core dumped)  ./a.out
 * ; ./a.out
 * a=4
 * zsh: segmentation fault (core dumped)  ./a.out
 */

/* now, for the final heresy: let's change the names of things, moving towards
 * longer, descriptive names rather than single character names.
 *
 * here's the most obvious data point i am interested in looking at first:
 *
 * ; wc --chars --lines main.c main7.c
 *    52  2255 main.c
 *   436  9175 main7.c
 *   488 11430 total
 *
 * at this point, we have roughly quadrupled the number of lines, as well as
 * the number of characters.
 *
 */

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct array
{
	long t;
	long r;
	long d[3];
	long p[2];
} *Array;

/* Allocates space for `n` long integers. */
long *malloc_longs(size_t n)
{
	return(long*) malloc(n * 4);
}

/* Copy `num` integers from `source` into `destination`. */
void copy_ints(long *destination, long *source, long num)
{
	for (long i = 0; i < num; ++i)
	{
		destination[i] = source[i];
	}
}

/*
 * NB: this `*d` seems like a likely seg fault culprit.
 */
long tr(long r, long *d)
{
	long z = 1;
	for (long i = 0; i < r; ++i)
	{
		z = z * d[i];
	}
	return z;
}

/* Construct a new Array. */
Array array_new(long t, long r, long *d)
{
	Array z    = (Array)malloc_longs(5 + tr(r, d));
	  z->t = t,
	  z->r = r,
	copy_ints(z->d, d, r);
	return z;
}

Array iota(Array w)
{
	long n = *w->p;
	Array z = array_new(0, 1, &n);
	for (long i = 0; i < n; ++i)
	{
		z->p[i] = i;
	}
	return z;
}

Array plus(Array a, Array w)
{
	long r = w->r,
	    *d = w->d,
	     n = tr(r, d);
	Array z = array_new(0, r, d);
	for (long i = 0; i < n; ++i)
	{
		z->p[i] = a->p[i] + w->p[i];
	}
	return z;
}

Array from(Array a, Array w)
{
	long r = w->r - 1,
	    *d = w->d + 1,
	     n = tr(r, d);
	Array z = array_new(w->t, r, d);
	copy_ints(z->p, w->p + (n**a->p), n);
	return z;
}

Array box(Array w)
{
	Array z = array_new(1, 0, 0);
	*z->p = (long)w;
	return z;
}

Array cat(Array a, Array w)
{
	long an = tr(a->r, a->d),
	     wn = tr(w->r, w->d),
	     n  = an + wn;
	Array z = array_new(w->t, 1, &n);
	copy_ints(z->p, a->p, an);
	copy_ints(z->p + an, w->p, wn);
	return z;
}

Array find(Array a, Array w)
{
}

Array rsh(Array a, Array w)
{
	long r  = a->r
		? *a->d
		: 1,
	     n  = tr(r, a->p),
	     wn = tr(w->r, w->d);
	Array z = array_new(w->t, r, a->p);
	copy_ints(z->p, w->p, wn = n>wn ? wn : n);
	if (n-=wn)
		copy_ints(z->p + wn, z->p, n);
	return z;
}

Array sha(Array w)
{
	Array z = array_new(0, 1, &w->r);
	copy_ints(z->p, w->d, w->r);
	return z;
}

Array id(Array w)
{
	return w;
}

Array size(Array w)
{
	Array z = array_new(0, 0, 0);
	*z->p = w->r ? *w->d : 1;
	return z;
}

/* Prints the given array to stdout. */
void print(Array w)
{
	long  r = w->r,
	     *d = w->d,
	      n = tr(r, d);
	for (long i = 0; i < r; ++i)
	{
		printf("%d ", d[i]);
	}
	printf("\n");
	if(w->t)
	{
		for (long i = 0; i < n; ++i)
		{
			printf("< ");
			print(w->p[i]);
		}
	}
	else
	{
		for (long i = 0; i < n; ++i)
		{
			printf("%d ", w->p[i]);
		}
	}
	printf("\n");
}

char verb_table[]="+{~<#,";

Array(*verb_dyads[]) () = {
	0,
	plus,
	from,
	find,
	0,
	rsh,
	cat
},
(*verb_monads[]) () = {
	0,
	id,
	size,
	iota,
	box,
	sha,
	0
};

long st[26];

bool is_symbol(long a)
{
	return  a >= 'a' && a <= 'z';
}

bool qv(long a)
{
	return a < 'a';
}

Array execute(long *e)
{
	long a = *e;
	if(is_symbol(a)) {
		if(e[1] == '=')
			return st[a-'a'] = execute(e+2);
		a = st[a-'a'];
	}
	return qv(a) ?
		  (*verb_monads[a])(execute(e+1))
		: e[1] ?
		  (*verb_dyads[e[1]])(a,execute(e+2))
		: (Array)a;
}

/* Returns an Array, given an integer.
 *
 * If the given character is not a number 0-9, returns NULL;
 */
Array noun(long c)
{
	Array z;
	if(c < '0' || c > '9')
		return 0;
	z = array_new(0, 0, 0);
	*z->p = c - '0';
	return z;
}

/* Returns the `symbol_table` index of `c` if it is a verb.
 *
 * Returns `NULL` if `c` is not the ASCII integer value of a verb.
 * */
long verb_table_index_of(long c)
{
	long i = 0;
	for(;verb_table[i];)
		if(verb_table[i++] == c)
			return i;
	return 0;
}

long *parse(char *input)
{
	char curr_char;
	long a,
	     n = strlen(input),
	    *e = malloc_longs(n+1);
	for (long i = 0; i < n; ++i)
	{
		curr_char = input[i];
		// If the current character is an integer `n` 0-9, create a
		// new Array...
		a = noun(curr_char);
		if (a)
		{
			e[i] = a;
		}
		else
		{
			// ...Otherwise, we may have a verb to contend with.
			a = verb_table_index_of(curr_char);
			if (a)
			{
				e[i] = a;
			}
			// If we are here, we have a symbol `a-z`.
			else
			{
				e[i] = curr_char;
			}
		}
	}

	// Leave a trailing `NULL`.
	e[n]=0;

	return e;
}

int main()
{
	char s[99];
	while(fgets(s, 99, stdin))
		print(execute(parse(s)));
}
