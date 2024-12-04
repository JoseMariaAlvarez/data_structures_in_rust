# Rust-tutorial

## Variables and mutabilidad

Declaración de una variable cuyo contenido no se puede cambiar y de otra variable cuyo contenido sí se puede cambiar. El tipado de las variables es estático, pero en este caso son declaraciones con tipado implícito. En ambos casos el tipo es `i32`.

~~~{.rust}
    fn main() {
        let x = 5;
        println!("The value of x is: {x}");
        let mut y = 5;
        y = 6;
    println!("The value of y is: {y}");
    }
~~~

Se puede incluir el tipo explícitamente en la declaración. De hecho, a veces es necesario porque el compilador no tiene suficiente información.

~~~{.rust}
    fn main() {
        let x : i32 = 5;
        println!("The value of x is: {x}");
        let mut y: i32 = 5;
        y = 6;
        println!("The value of y is: {y}");
    }
~~~

También se pueden definir constantes:

~~~{.rust}
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
~~~

## Ensombrecimiento

La declaración de una variable puede *ensombrecer* u *ocultar* la declaración previa de una variable con el mismo nombre en el mismo ámbito o en un ámbito más externo.

~~~{.rust}
    fn main() {
        let x: i32 = 5;

        let x = String::from("hello");

        {
            let x = 2;
            println!("The value of x in the inner scope is: {x}");
        }

        println!("The value of x is: {x}");
    }
~~~

## Tipo de datos

### Tipos escalares

Tipos enteros: 

|Length	| Signed  | Unsigned|
|-------|---------|---------|
|8-bit	| i8      | u8      |
|16-bit	| i16     | u16     |
|32-bit | i32     | u32     |
|64-bit	| i64     | u64     |
|128-bit| i128    | u128    |
|arch   | isize   |usize    |

Tipos reales: `f32`y `f64`.

Tipo `bool`y `char`.

### Tipos compuestos

Las *tuplas* son colecciones de varios valores de tipos diferentes y de tamaño fijo tras su definición.

~~~{.rust}
    fn main() {
        let tup = (500, 6.4, 1);

        let (x, y, z) = tup;

        println!("The value of y is: {y}");
        println!("The value of first component is: {}", tup.0);
    }
~~~

Los *arrays son colecciones de varios valores del mismo tipo accesibles por índice. Son de tamaño fijo tras su definición.

~~~{.rust}
    fn main() {
        let a = [1, 2, 3, 4, 5];

        let first = a[0];
        let second = a[1];
    }
~~~

Si se accede a una posición del array fuera de sus límites, se produce un error en tiempo de ejecución y se aborta el programa.

## Funciones

Similares a las de Java o C++. Por defecto, se devuelve como resultado de la función el valor de la evaluación de la última expresión de la función, aunque también se puede usar la instrucción `return`.

~~~{.rust}
    fn main() {
        let x = plus_one(5);
        let y = plus_two(5);

        println!("The value of x is: {x}");
        println!("The value of y is: {y}");
    }

    fn plus_one(x: i32) -> i32 {
        x + 1
    }

    fn plus_two(x: i32) -> i32 {
        return x + 2;
    }
~~~

## Estructuras de control

### Sentencia condicional

La sintaxis de la sentencia condicional es muy similar a la de Java/C++:

~~~{.rust}
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
~~~

### Sentencias bucle

El bucle *mientras* también es muy similar al de Java/C++:

~~~{.rust}
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}~~~

El bucle *for* es distinto al tradicional de Java/C++, se parece más al *foreach* de Java:

~~~{.rust}
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // (1..4) genera un intervalo de 1 a 3.
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
~~~

## Propiedad

La *propiedad* de un valor es la característica más definitoria del lenguaje Rust, en particular en la gestión de la memoria y el acceso seguro a los datos.

### Qué es la propiedad

Las reglas que controlan la propiedad de un valor son:
- Cada valor en Rus tiene un propietario.
- El propietario de un valor es único en cada momento, aunque puede variar a lo largo de la vida del valor.
- Cuando se termina el ámbito del propietario, se libera el valor.

### Asignación de variables con copia

Para entender la propiedad, primero es necesario conocer cómo se copian variables en Rust. Hay que distinguir en primer lugar la copia de los tipos simples (enteros, reales, `char` y `bool`). Para estos tipos, cuando se asignan variables, se crea una nueva copia del valor del tipo correspondiente, por lo que no hay conflictos de propiedad:

~~~{.rust}
    fn main() {
        let x = 5;
        let y = x;

        println!("x = {x}, y = {y}");
    }
~~~

En el ejemplo anterior, hay dos variables de tipo `i32`y ambas estás asociadas a dos valores distintos, es decir, dos copias del valor `5i32`.

Esa regla también se aplica a las tuplas y arrays si, recursivamente, la regla anterior de puede aplicar a los tipos de sus componentes.

### Asignación de variables con movimiento

Hay tipos que no tienen implementada la operación de copia, como el tipo predefinido `String`. Para esos tipos, una asignación de variables significa que se *mueve* la propiedad del valor de la variable que está en la derecha de la asignación a la variable que está a la izquierda de la asignación. A partir de ese momento, la variable a la derecha ya no se puede usar.

~~~{.rust}
    fn main() {
    let s1 = String::from("hello");
    let mut s2 = s1;

    s2.push_str(" world!");

    println!("{s2}");

    }
~~~

Desde la asignación `let mut s2 = s1;`, la variable `s1` ya no es utilizable, porque el valor que tenía en propiedad se ha movido a otra variable.

### Asignación de variables a parámetros de funciones

La regla a la hora de aplicar parámetros reales a parámetros formales de funciones es la misma que en la asigación de variables. Para los tipos con copia (los escalares, tuplas y arrays de elementos copiables), se crea una copia del parámetro real y se le asigna al parámetro formal. Para los tipos sin copia, como `String`, se hace un movimiento del valor y el parámetro formal deja de ser utilizable.

~~~{.rust}
    fn main() {

        let s1 = String::from("hello");
        let x = 5;

        my_function(x, s1);
        println!("{x}");

    }

    fn my_function(i:i32, ss: String){
        println!("{i} {ss}");
    }
~~~

### Referencias. Préstamo de propiedad

La regla de mover un valor e inutilizar una variable cuando se asigna con movimiento es muy restrictiva. Las referencias proporcionan la posibilidad de pedir prestada la propiedad de un valor sin inutilizar una variable.

Se pueden usar referencias en la asignación entre variables o definir un parámetro formal de una función como una referencia. También se puede usar como valor devuelto por una función.

~~~{.rust}
    fn main() {

        let s1 = String::from("hello");

        let s2 = &s1;

        my_function(s2);
        my_function(&s1);

        println!("s1: {s1}");
        println!("s2: {s2}");

    }

    fn my_function(s: & String) {
        println!("my_function: {s}");
    }
~~~

Para pedir prestada la propiedad de un valor con la intención de modificarlo, hay que usar una referencia mutable.

~~~{.rust}
    fn main() {
        let mut s = String::from("hello");

        change(&mut s);
    }

    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }
~~~

Para evitar condiciones de carrera en el acceso por referencia a un valor, el uso de referencias está restringido según las siguientes reglas:

- Pueden haber múltiples referencias no mutables activas a la vez.
- Una referencia mutable no puede estar activa a la vez que otra referencia, ya sea mutable o no.

### Referencias colgantes

Tener una referencia que apunte a un valor cuyo ámbito se ha borrado y que el compilador ha retirado de memoria es una situación errónea que debemos evitar. Esas referencias son conocidas como *colgantes* o *huérfanas* (traducción del término *dangling*). Una referencia, por tanto, no puede tener una vida más larga que el valor al que apunta. Una posible situación de este tipo, prohibida por el compilador, es que una función devuelva una referencia a una variable creada localmente.

~~~{.rust}
    fn main() {
        let reference_to_nothing = dangle();
    }

    fn dangle() -> &String {
        let s = String::from("hello");

        &s
    }
~~~

### El tipo `slice`

Una `slice` en Rust es una referencia a una secuencia continua de elementos en una colección. Un *slice* se define indicando cuál es el rango al que va a hacer referencia. Para ello, se indica la posición de inicio y la de fin separadas por dos caracteres punto. Si la posición de inicio es la primera (la 0), se puede omitir. Si la posición de final es la última de la variable, también se puede omitir.

## Estructuras

Las estructuras de Rust permiten crear datos con componentes de tipos distintos. La visibilidad de los componentes se puede limitar (que sean públicos o privados). También se pueden definir operaciones para efectuar sobre las variables de un tipo estructura.

~~~{.rust}
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    fn main() {
        let user1 = User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
        };
    }
~~~

### Métodos

A las estructuras de Rust se les pueden añadir *métodos* para definir su funcionamiento. Los métodos son similares a las funciones pero dentro del ámbito de la estructura. Los métodos pueden ser generales al tipo estructura o para una variable concreta. En este último caso, el primer parámetro del método es el parámetro especial `self`, que puede ser pasado por referencia. La sintaxis de llamada a los métodos generales es con el nombre del tipo de la estructura, seguidos de dos pares de caracteres dos puntos y el nombre del método. La sintaxis para los métodos asociados a una variable particular es con el nombre de la variable, seguido de un punto y el nombre del método. 

~~~{.rust}
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }

    fn main() {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        let rect2 = Rectangle::square(20);

        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
        );
        
        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    }
~~~

## Enumerados

Los tipos enumerados en Rust sirven para implementar tipos con un número limitado de valores, una idea similar a la de Java y la de C++, pero permite la definición de una estrucutura y un funcionamiento distinto para cada valor del tipo enumerado, además del uso de instrucciones de control que hacen uso de *pattern matching* para procesar los distintos valores. Hay dos tipos enumerados predefinidos destacables, *Option*, que permite tratar el valor *nulo* de forma más coherente que en otros lenguajes y *Result*, que permite hacer un tratamiento de errores distinto al que se hace en otros lenguajes.

A continuación se muestra un ejemplo de la forma más sencilla de definir un tipo enumerado:

~~~{.rust}
    enum IpAddrKind {
        V4,
        V6,
    }

    fn main(){
        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;
    }
~~~ 

Se puede particularizar cada valor del tipo enumerado para que tenga sus propios campos:

~~~{.rust}
    enum IpAddrKind {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    fn main(){
        let home = IpAddr::V4(127, 0, 0, 1);
        let loopback = IpAddr::V6(String::from("::1"));
    }
~~~ 

### El tipo enumerado `Option` y los valores nulos

`Option<T>` es un tipo enumerado predefinido de Rust. Además es un tipo genérico, se puede instanciar sobre el tipo del parámetro (`T`). `Option<T>` tiene dos posibles valores:

~~~{.rust}
    enum Option<T> {
        None,
        Some(T),
    }
~~~ 

El primer valor sirve para indicar que no hay un valor válido. El segundo para indicar que sí lo hay. 

~~~{.rust}
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
~~~

# La instrucción `match`

La instrucción `match` permite ejecutar código distinto en función del valor de un tipo enumerado, pudiendo acceder a su estructura interna porque usa *pattern matching* cuando selecciona la opción correspondiente.

~~~{.rust}
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(String),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {state}!");
                25
            }
        }
    }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    fn main(){
        let coin = Coin::Quarter("Alabama".to_string());
        let value = value_in_cents(coin);
        println!("The value of the coin is: {}", value);

        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);

        println!("Six: {six:?}, None: {none:?}");
    }
~~~

### El tipo `Result`

 `Result`<T, S> es otro tipo enumerado genérico predefinido de Rust que se usa para devolver valores en funciones donde se pueden producir errores. 

~~~{.rust}
    fn divide_i32_with_result(x: i32, y: i32) -> Result<i32, String> {
        if y == 0 {
            return Err("Cannot divide by zero".to_string());
        }
        Ok(x / y)
    }

    fn main(){

        let i = 8;
        let j = 2;
        let k = 0;
        match divide_i32_with_result(i, j) {
            Ok(result) => println!("{} divided by {} is {}", i, j, result),
            Err(err) => println!("Error: {}", err),
        }
        match divide_i32_with_result(i, k) {
            Ok(result) => println!("{} divided by {} is {}", i, k, result),
            Err(err) => println!("Error: {}", err),
        }
        
    }
~~~

## Vectores

El tipo genérico predefinido `Vec` (vector) permite almacenar en una estructura lineal una colección de elementos del mismo tipo. Los elementos están ordenados y son accesibles por la posición que ocupan en el vector. Es análogo a un `List` de Java o al `vector` de C++.

Se puede crear un vector vacío a través de la operación globla `new` del tipo `Vec` o se puede crear un vector inicializado con datos con la macro `vec!`

~~~{.rust}
    fn main(){
        let v: Vec<i32> = Vec::new();
        let v2 = vec![1, 2, 3];
    }
~~~

Algunas de las operaciones más usuales permiten insertar un elemento después de los elementos que ya están insertados (`push`), eliminar el último elemento (`pop`), saber si el vector está vacío (`is_empty`) o el número de elementos (`len`)

~~~{.rust}
    fn main(){
        let mut v: Vec<i32> = Vec::new();
        let v2 = vec![1, 2, 3];

        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
        v.pop();
        println!("{:?}", v);
        println!("{:?}", v.is_empty());
        println!("{:?}", v2.len());
    }
~~~

Hay dos formas de acceder a los elementos de un vector. En ambos casos se indica la posición del elemento en el vector, cuyo rango empieza en 0 y termina en el número de elementos menos uno. Se puede acceder con el operador `[]`. Si se le pasa una posición incorrecta, se produce un error en tiempo de ejecución y el programa termina. La otra forma es con el método `get`, que toma como parámetro la posición, y devuelve un valor de tipo `Option<&T>`, donde `T` es el tipo de los elementos del vector. Si la posición es correcta, devuelve un valor del tipo `Some(&T)`. Si la posición es incorrecta, devuelve `None`. 
~~~{.rust}
    fn main(){
        let mut v: vec![1, 2, 3];
        let j = v.get(2);
        println!("{:?}", j);
    }
~~~

La referencia devuelta por `get` es de solo lectura, si queremos acceder a los elementos de los vectores para modificación, podemos conseguir una referencia mutable con la operación `get_mut`.

~~~{.rust}
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    fn main() {
        let user1 = User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
        };
        let user2 = User {
            active: true,
            username: String::from("someusername456"),
            email: String::from("someoneelse@example.com"),
            sign_in_count: 2,
        };

        let mut v = vec![user1, user2];
        let first = v.get_mut(0);
        if let Some(user) = first {
            user.active = false;
            user.username.push_str("something else");
        }
        let mut second = & mut v[1];
        second.active = false;
        second.username.push_str("something else");
        println!("v is {:#?}", v);

        let im = v.iter_mut();
        for user in im {
            user.email = user.email.replace("example.com", "gmail.com");
        }
        println!("v is {:#?}", v);

    }
~~~

## Genericidad

Rust permite la definición de elementos genéricos que se pueden aplicar a diferentes tipos, como se hace en Java y C++.

### Funciones genéricas

Se pueden definir funciones genéricas, que acepten parámetros de un tipo genérico que se instanciará en cada llamada concreta:

~~~{.rust}
    fn largest<T>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    fn main() {
        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest(&number_list);
        println!("The largest number is {result}");

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest(&char_list);
        println!("The largest char is {result}");
    }
~~~

### Estructuras genéricas

Las estructuras también se pueden definir como genéricas, de tal manera que los tipos de los atributos pueden ser de tipos genéricos, que se instanciarán para cada variable de ese tipo estructura. Se puede definir la genericidad sobre varios tipos

~~~{.rust}
    struct Point<T, U> {
        x: T,
        y: U
    }

    fn main() {
        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 1, y: 4.0 };
    }
~~~

## Tipos enumerados genéricos

También los tipos enumerados se pueden definir genéricos, como hemos visto con los ejemplos de `Option` y `Result`.

~~~{.rest}
    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
~~~

## Métodos genéricos

Se pueden definir métodos genéricos dentro del contexto de una estructura, de forma similar a como se definen funciones genéricas.

~~~{.rust}
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    fn main() {
        let p = Point { x: 5, y: 10 };

        println!("p.x = {}", p.x());
    }
~~~

## Definición de funcionalidad con `trait`s

El concepto de interfaz de Java se implementa de manera similar en Rust con los `trait`s, que definen métodos que los tipos deben implementar para cumplir ese `trait`.

~~~{.rust}
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    fn main() {
        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                hockey team in the NHL.",
            ),
        };

        println!("New article available! {}", article.summarize());
    }
~~~

Los `trait`s se usan habitualmente en la definición de restricciones en los tipos genéricos. Por ejemplo, en la función `largest` que se presentó antes, solo los tipos que implementen la comparación entre elementos con el operador `<` pueden instanciar esa función.

~~~{.rust}
    pub fn notify<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }

    use std::fmt::Display;

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
~~~

## Validación de referencias con *lifetimes*

Como se mencionó en un apartado anterior, hay que evitar que una referencia se quede colgada (*dangled*), es decir, que apunte a un valor que ha sido eliminado de memoria. Se puede forzar a que el intervalo de vida del valor al que apunte la referencia sea igual o más amplio que el de la referencia, para evitar que se quede colgada.

~~~{.rust}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}
~~~

En el ejemplo anterior, la función `longest` se hace genérica sobre la *lifetime* `'a`. Con la definición anterior, se exige que el resultado de la función tenga un intervalo de vida menor o igual que el menor de los intervalos de vida de los parámetros con los que se llame a la función.

## Documentación
[The Rust Book](https://doc.rust-lang.org/book/title-page.html)