# JARisp: Just AnotheR lisp Interpreter in rust  
This project is just for getting familiar with rust, and is an extremely incomplete version of lisp interpreter. 

This project mainly refers to these two post:

[https://norvig.com/lispy.html](https://norvig.com/lispy.html)

[https://stopa.io/post/222](https://stopa.io/post/222)

The functions I have implemented include:
* Simple calculation: like +,-,*,/
* Condition control: only include "if"
* Function definition and calling
* Closure: I'm not sure if it's the correct way, but it works like this
    ```
    ( begin 
        (define f (
            lambda (x) (
                (lambda (z) ( + x z) )
            )
        ))
        (define g (f 2))
        (g 3)
    )
    ```
    The output of the $(g\ 3)$ is 5 in this case because running $(g\ x)$ is eval to $(+\ 2\ x)$
* Currying: it works like this:
    ```
    ( begin 
        (define f (lambda (a b c) ( * 2 a b c)))
        (f 2 3 2)
        (define g ( f 4 2 ))
        (g 3 )
    )
    ```
    here running result of $(g\ 3)$ is 48, because $(g\ x)$ is equal to $(f\ 4\ 2\ x)$
## running
The program can be build with
```
cargo build
```
and run with
```
.\target\debug\risp.exe .\examples\test_closure
```
The output of the program will be
```
Not output for ( define f ( lambda ( a b c ) ( * 2 a b c ) ) )
The result of ( f 2 3 2 ) is 24
Not output for ( define g ( f 4 2 ) )
The result of ( g 3 ) is 48
```

