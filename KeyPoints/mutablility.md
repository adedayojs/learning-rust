In rust you can have a mutable value, an immutable value, a mutable reference to an immutable value or a mutable reference to a mutable value. A string literal is immutable value, `let mut s = "hello"` creates a mutable reference to an immutable value, which allows you to change what s refers to (which is what you do in the next line of your code).

Consider this piece of code:

`   
    let mut number = 3;
    number = 4;
    println!("{}", number);
`

This prints 4. Would you also say that in this case, you mutated 3 and it became 4? Of course not. You mutated the contents of the number variable.

Changing the contents of a variable doesn't magically change whatever it was initialized with in the first place. The initializer value is a separate entity from the contents of the variable. It is moved (or copied, if possible) into the storage represented by the variable upon initialization or assignment. It can even cease existing afterwards. There's nothing special about string literals here.