fn primeiro_i32(lista: &Vec<i32>) -> &i32 {
    lista.get(0).expect("A lista está vazia")
}
fn primeiro_string(lista: &[String]) -> &String {
    lista.get(0).expect("A lista está vazia")
}

fn main() {
    let numeros = vec![1, 2, 3];
    let primeiro_numero = primeiro_i32(&numeros);
    println!("Primeiro número: {:?}", primeiro_numero);

    let palavras = vec!["olá".to_string(), "mundo".to_string()];
    let primeira_palavra = primeiro_string(&palavras);
    println!("Primeira palavra: {:?}", primeira_palavra);
}
