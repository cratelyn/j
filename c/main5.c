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

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct a
{
	long t, r, d[3], p[2];
} *A;

long *ma(size_t n)
{
	return(long*) malloc(n * 4);
}

void mv(long *d, long *s, long n)
{
	for (long i = 0; i < n; ++i)
	{
		d[i] = s[i];
	}
}

long tr(long r, long *d)
{
	long z = 1;
	for (long i = 0; i < r; ++i)
	{
		z = z*d[i];
	}
	return z;
}

A ga(long t, long r, long *d)
{
	A z    = (A)ma(5 + tr(r, d));
	  z->t = t,
	  z->r = r,
	mv(z->d, d, r);
	return z;
}

A iota(A w)
{
	long n = *w->p;
	A z = ga(0, 1, &n);
	for (long i = 0; i < n; ++i)
	{
		z->p[i] = i;
	}
	return z;
}

A plus(A a, A w)
{
	long r = w->r,
	    *d = w->d,
	     n = tr(r, d);
	A z = ga(0, r, d);
	for (long i = 0; i < n; ++i)
	{
		z->p[i] = a->p[i] + w->p[i];
	}
	return z;
}

A from(A a, A w)
{
	long r = w->r - 1,
	    *d = w->d + 1,
	     n = tr(r, d);
	A z = ga(w->t, r, d);
	mv(z->p, w->p + (n**a->p), n);
	return z;
}

A box(A w)
{
	A z = ga(1, 0, 0);
	*z->p = (long)w;
	return z;
}

A cat(A a, A w)
{
	long an = tr(a->r, a->d),
	     wn = tr(w->r, w->d),
	     n  = an + wn;
	A z = ga(w->t, 1, &n);
	mv(z->p, a->p, an);
	mv(z->p + an, w->p, wn);
	return z;
}

A find(A a, A w)
{
}

A rsh(A a, A w)
{
	long r  = a->r
		? *a->d
		: 1,
	     n  = tr(r, a->p),
	     wn = tr(w->r, w->d);
	A z = ga(w->t, r, a->p);
	mv(z->p, w->p, wn = n>wn ? wn : n);
	if (n-=wn)
		mv(z->p + wn, z->p, n);
	return z;
}

A sha(A w)
{
	A z = ga(0, 1, &w->r);
	mv(z->p, w->d, w->r);
	return z;
}

A id(A w)
{
	return w;
}

A size(A w)
{
	A z = ga(0, 0, 0);
	*z->p = w->r ? *w->d : 1;
	return z;
}

void pi(int i)
{
	printf("%d ", i);
}

void nl()
{
	printf("\n");
}

void pr(A w)
{
	long  r = w->r,
	     *d = w->d,
	      n = tr(r, d);
	for (long i = 0; i < r; ++i)
	{
		pi(d[i]);
	}
	nl();
	if(w->t)
	{
		for (long i = 0; i < n; ++i)
		{
			printf("< ");
			pr(w->p[i]);
		}
	}
	else
	{
		for (long i = 0; i < n; ++i)
		{
			pi(w->p[i]);
		}
	}
	nl();
}

char vt[]="+{~<#,";

A(*vd[]) () = {
	0,
	plus,
	from,
	find,
	0,
	rsh,
	cat
},
(*vm[]) () = {
	0,
	id,
	size,
	iota,
	box,
	sha,
	0
};

long st[26];

bool qp(long a)
{
	return  a >= 'a' && a <= 'z';
}

bool qv(long a)
{
	return a < 'a';
}

A ex(long *e)
{
	long a = *e;
	if(qp(a)) {
		if(e[1] == '=')
			return st[a-'a'] = ex(e+2);
		a = st[a-'a'];
	}
	return qv(a) ?
		  (*vm[a])(ex(e+1))
		: e[1] ?
		  (*vd[e[1]])(a,ex(e+2))
		: (A)a;
}

A noun(long c)
{
	A z;
	if(c < '0' || c > '9')
		return 0;
	z = ga(0, 0, 0);
	*z->p = c - '0';
	return z;
}

long verb(long c)
{
	long i = 0;
	for(;vt[i];)
		if(vt[i++] == c)
			return i;
	return 0;
}

long *wd(char *s)
{
	long a,
	   n = strlen(s),
	  *e = ma(n+1);
	char c;
	for (long i = 0; i < n; ++i)
	{
		e[i]=(a=noun(c=s[i]))?a:(a=verb(c))?a:c;
	}
	e[n]=0;
	return e;
}

int main()
{
	char s[99];
	while(fgets(s, 99, stdin))
		pr(ex(wd(s)));
}
