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
```
(:
	(= a 1)
	(loop 5 (= a (+ a 1)))
	(> a)
)
```
