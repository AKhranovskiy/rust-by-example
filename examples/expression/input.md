В Rust почти все фигурные скобки являются выражением, это значит, 
что они могут вернуть какой-то результат. Такое поведение не всегда нужно,
чтобы ничего не возвращать добавьте `;` в конец.

Выражения в блоке могут использоваться в качестве [r-values](https://en.wikipedia.org/wiki/Value_%28computer_science%29#lrvalue) значений, а последнее будет назначено как [l-value](https://en.wikipedia.org/wiki/Value_%28computer_science%29#lrvalue).
> Что такое Rvalue и Lvalue читайте [здесь](http://msdn.microsoft.com/ru-ru/library/f90831hc.aspx).

Но, если последнее выражение в блоке будет точкой с запятой, результат будет равен пустому кортежу: `()`.

{expression.play}
