use std::sync::mpsc::channel;

fn main() {
    let (tx, rx) = channel();

    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");

    for i in 0.. {
        let f = format!("{}.99", i).parse::<f32>().unwrap();

        if !test(f) {
            println!("{}", f);
        }

        if rx.try_recv().is_ok() {
            println!("last tested value = {}", i);

            break;
        }
    }
}

fn test(i: f32) -> bool {
    let f = format!("{:.10}", i).parse::<f32>().unwrap();
    let d = f * 100f32;
    let a = d as i64;
    let b = format!("{:.2}", d).parse::<f32>().unwrap() as i64;

    a == b
}
