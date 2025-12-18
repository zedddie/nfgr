# NFGR - Newton Fractal Generator written in Rust

I was bored so I decided to write nfgr and document how it works under
the hood.

To start with, You need to know what Newton's Method is, formula looks like:
$`
    z_{n+1} = z_n - frac\{f(z_n)}{f'(z_n)}
`$
you can click here if you want to learn more about Newton's Method and how it works.

When you take z (root approximation) you can substract f(z)/f'(z) from it to get
more accurate approximation to the closest root, thus by iterating you'll get closer
and closer to root.
