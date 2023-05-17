use std::error::Error;
use std::io;

fn main() {
    let mut input: Vec<f64> = Vec::new();
    loop{
        println!("add another number to the dataset, or type \"done\" to calculate");
        let mut num = String::new();
        match io::stdin().read_line(&mut num){
            Ok(_) => (),
            Err(_) => {println!("failed to read input");continue;}
        }
        match num.to_lowercase().as_str().trim(){
            "done" => break,
            _ => match num.trim().parse::<f64>(){
                Ok(num) => input.push(num),
                Err(_) => {println!("bad input: {num}");continue;}
            }
        }
    }
    match geoth_mean(input.as_mut_slice()){
        Ok(n) => println!("{n}"),
        Err(txt) => print!("{txt}")
    };
}

fn geoth_step(vec: &mut [f64]) -> Result<[f64; 3], &str>{
    if vec.len() < 1{
        return Err("cannot operate on empty array");
    }
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut iter = vec.iter();
    let arith: f64 = iter.clone().sum::<f64>() as f64 /vec.len() as f64;
    let median = match vec.len() {
        n if n%2!=0 => vec[(n+1)/2 - 1] as f64,
        n => (vec[(n)/2] + vec[(n)/2 - 1]) as f64/2.0
    };
    let mut geo = *iter.next().unwrap();
    for i in iter{
        geo = geo * i;
    }
    
    geo = geo.powf(1.0/vec.len() as f64);
    Ok([arith,median,geo])
}

fn geoth_mean(vec: &mut[f64]) -> Result<f64, Box<dyn Error>>{
    let mut step = geoth_step(vec)?;
    if step[0] - step[1] <= 0.001 && step[1] - step[2] <= 0.001 && step[0] - step[2] <= 0.0000001 {
        return Ok((step[0] + step[1] + step[2])/3.0);
    }else{
        return geoth_mean(&mut step);
    }
    
}