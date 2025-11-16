// mod examples;

fn fibo(nacci:u32) -> u32 {
    if nacci <= 1{
        nacci
    } else {
        fibo(nacci-1) + fibo(nacci-2)
    }
}

fn main() {
    // examples::variables_and_mutability();
    // examples::data_types();
    // examples::statements_and_expressions();
    // examples::control_flow();
    println!("Fibonacci of 6 is: {}", fibo(6));

}