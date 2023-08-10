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
 * at this stage, we still have some warnings remaining related to
 * integer/pointer conversion. before we dive deeper into that, we will inline
 * some single-statement functions, and tackle the remaining preprocessor
 * macros: `V1`, `V2`, and `DO`.
 */

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct a
{
	long t, r, d[3], p[2];
} *A;

#define V1(f) A f(w)A w;
#define V2(f) A f(a,w)A a,w;
#define DO(n,x) {long i=0,_n=(n);for(;i<_n;++i){x;}}

long *ma(size_t n)
{
	return(long*) malloc(n * 4);
}

void mv(long *d, long *s, long n)
{
	DO(n, d[i] = s[i]);
}

long tr(long r, long *d)
{
	long z = 1;
	DO(r, z = z*d[i]);
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

V1(iota)
{
	long n = *w->p;
	A z = ga(0, 1, &n);
	DO(n, z->p[i] = i);
	return z;
}

V2(plus)
{
	long r = w->r,
	 *d = w->d,
	  n = tr(r, d);
	A z = ga(0, r, d);
	DO(n, z->p[i] = a->p[i] + w->p[i]);
	return z;
}

V2(from)
{
	long r = w->r - 1,
	 *d = w->d + 1,
	  n = tr(r, d);
	A z = ga(w->t, r, d);
	mv(z->p, w->p + (n**a->p), n);
	return z;
}

V1(box)
{
	A z = ga(1, 0, 0);
	*z->p = (long)w;
	return z;
}

V2(cat)
{
	long an = tr(a->r, a->d),
	  wn = tr(w->r, w->d),
	  n  = an + wn;
	A z = ga(w->t, 1, &n);
	mv(z->p, a->p, an);
	mv(z->p + an, w->p, wn);
	return z;
}

V2(find)
{
}

V2(rsh)
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

V1(sha)
{
	A z = ga(0, 1, &w->r);
	mv(z->p, w->d, w->r);
	return z;
}

V1(id)
{
	return w;
}

V1(size)
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
	DO(r, pi(d[i]));
	nl();
	if(w->t)
		DO(n, printf("< "); pr(w->p[i]))
	else
		DO(n, pi(w->p[i]));
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
	long i=0;
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
	DO(n,e[i]=(a=noun(c=s[i]))?a:(a=verb(c))?a:c);
	e[n]=0;
	return e;
}

int main()
{
	char s[99];
	while(fgets(s, 99, stdin))
		pr(ex(wd(s)));
}
