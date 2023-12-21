fn main() {
    let i = 4.99;

    println!("i = {}", i);
    println!("i.2 = {:.2}", i);
    println!("i.10 = {:.10}", i);

    println!();

    let f = format!("{:.10}", i).parse::<f32>().unwrap();
    println!("f = {}", f);
    println!("f.2 = {:.2}", f);
    println!("f.10 = {:.10}", f);

    println!();

    let d = f * 100f32;
    println!("d = {}", d);
    println!("d.2 = {:.2}", d);
    println!("d.10 = {:.10}", d);

    println!();

    let a = d as i64;
    println!("a = {}", a);

    let b = format!("{:.2}", d).parse::<f32>().unwrap() as i64;
    println!("b = {}", b);
}
