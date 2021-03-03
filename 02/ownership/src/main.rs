fn om_nom_nom_string(s: String)
{

    println!("{}", s);
}


fn om_nom_nom(n: u32)
{
    println!("{} is a very nice number", n);
}

fn main() {
    let s: String = "hello,world".to_string();
    

    om_nom_nom_string(s);
    //let u = s;

    //println!("{}", s);



    let n: u32 = 110;
    let m = n; //copy trait
    
    om_nom_nom(n);
    om_nom_nom(m);

    println!("{}",m+n);
}


