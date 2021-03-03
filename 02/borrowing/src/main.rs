

fn change_it_up( s: &mut String)
{

    *s = "goodbye".to_string();
}

fn make_it_plural(word: &mut String)
{
    word.push('s');
}

fn let_me_see(s: &String)
{
    println!("{}", s);
}



fn main() {
    
    let mut s = "Hello".to_string();

    change_it_up(&mut s);

    let_me_see(&s);

    make_it_plural(&mut s);
    let_me_see(&s);

    s.push('s');
    let_me_see(&s);


    let mut v = vec![1,2,3];
    for i in v.iter_mut() {

        *i = 5;
    }

    for i in v.iter() {
        println!("{}", i);
    }


}
