//Escribe una función que acepte una matriz
//de 10 números enteros (entre 0 y 9),
//que devuelva una cadena de esos números
//en forma de número de teléfono.
// create_phone_number(&[1,2,3,4,5,6,7,8,9,0]); // returns "(123) 456-7890"

// Solución 1
fn create_phone_number(numbers: &[u8]) -> String {
    if numbers.len() != 10 {
        return "La longitud de la matriz es diferente a 10".to_string();
    }

    for i in numbers {
        if i / 10 > 0 {
            return format!("{i} es mayor que 10");
        }
    }

    let mut phone_number = format!("(");

    for i in &numbers[0..3] {
        phone_number += &i.to_string();
    }

    phone_number += ") ";

    for i in &numbers[3..6] {
        phone_number += &i.to_string();
    }

    phone_number += "-";

    for i in &numbers[6..] {
        phone_number += &i.to_string();
    }

    phone_number
}

// solución 2, (ya no se comprobarán la cantidad de numeros correctos)
fn create_phone_number_2(numbers: &[u8]) -> String {
    let numbers = numbers
        .into_iter()
        .map(|i| i.to_string())
        .collect::<String>();

    format!("({}) {}-{}", &numbers[0..3], &numbers[3..6], &numbers[6..])
}

fn main() {
    println!("{}", create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]));
    println!("{}", create_phone_number_2(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]));
}

#[test]
fn returns_expected() {
    assert_eq!(
        create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]),
        "(123) 456-7890"
    );
    assert_eq!(
        create_phone_number(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
        "(111) 111-1111"
    );
    assert_eq!(
        create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]),
        "(123) 456-7899"
    );
}
