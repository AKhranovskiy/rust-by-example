fn main() {
    // Использование локального вывода, компилятор знает, что `elem` имеет тип `u8`
    let elem = 5u8;

    // Создадим пустой вектор (расширяемый массив)
    let mut vec = Vec::new();
    // В этот момент компилятор не знает точный тип `vec`, он
    // просто знает, что это вектор `Vec<_>`

    // Вставим `elem` в вектор
    vec.push(elem);
    // Ага! Теперь компилятор знает, что `vec` это вектор `u8` (`Vec<u8>`)
    // Попробуйте закомментировать строку `vec.push(elem)`

    println!("{}", vec);
}
