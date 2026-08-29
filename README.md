RLisp is a small toy language/interpreter written in rust and inspired by lisp

example program:
```lisp
(:
	(= a (<))
	(= b (<))
	(= c (+ a b))
	(> c)
)
```

another:
```lisp
(:
	(= n 10)
	(loop n
	    (= n (+ n 1))
	)
	(> n)
)
```
