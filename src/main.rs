fn main() {
    let a_number = 170423;
    let teste: &str = "Hello world";
    println!("{} um número {}", teste, a_number);

    let salario = 1310;

    if salario == 1309{
        println!("{} não é um salário válido", salario);
    }else{
        println!("então o salário é: {}", salario);
    }

    executa_teste();
}


fn executa_teste(){
    print!("Executando uma função apartada");
}
