RLisp is a small toy language/interpreter written in rust and inspired by lisp

example programs:
```lisp
(
	>
	"Hello, world!"
)
```

```lisp
(:
	(= a (<))
	(= b (<))
	(= c (+ a b))
	(> c)
)
```

```lisp
(:
	(= n 10)
	(loop n
	    (= n (+ n 1))
	)
	(> n)
)
```

```lisp
(:
	(= n ($ (> "hi")))
	(n)
	(n)
	(n)
)
```

```lisp
(:
	(> "what is the best animal")
	(= i (<))
	(>
		(if (== i "cat")
			"correct"
			"wrong"
		)
	)
)
```
