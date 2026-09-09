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

```lisp
(:
	(= addoneandprint 
		($
			(> (+ a 1))
			a
		)
	)	
	(addoneandprint, 5)
)
```
```lisp
(:
	(= alist (list 1 5 4))
	(> (nth alist 1))
)
```
```lisp
(:
	(= alist (list 1 5 4))
	(= i 0)
	(loop (len alist) (:
		(> (nth alist i))
		(= i (+ i 1))
	))
)
```