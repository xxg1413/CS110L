use std::collections::HashSet;
//练习

fn add_n(v: Vec<i32>, n: i32) -> Vec<i32> {
    

    let mut result: Vec<i32> = Vec::new();

    for i in v.iter() {
        result.push(i+n)
    }

    result
}

fn add_n_inplace(v: &mut Vec<i32>, n: i32) {

    let mut i = 0;

    while i < v.len() {

        v[i] = v[i] + n;
        i = i + 1;
    }
}

fn dedup(v: &mut Vec<i32>) {


    let mut hs = HashSet::new();
    let mut i = 0;

    while i < v.len() {

        if !hs.contains(&v[i]) {

            hs.insert(v[i]);
            i += 1;

        }  else {

            v.remove(i);
        }
    
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_n() {
        assert_eq!(add_n(vec![1], 2), vec![3]);
    }

    #[test]
    fn test_add_n_inplace() {
        let mut v = vec![1];
        add_n_inplace(&mut v, 2);
        assert_eq!(v, vec![3]);
    }

    #[test]
    fn test_dedup() {
        let mut v = vec![3, 1, 0, 1, 4, 4];
        dedup(&mut v);
        assert_eq!(v, vec![3, 1, 0, 4]);
    }
}



fn main()
{

    let n:i32 = 1;
    let n = 1;


    //可变类型
    let mut  n = 0;
    n = n + 1;

    //Rust有两种字符串： &str和String
    let s: &str = "hello,world"; //只读数据段

    let mut s: String = String::from("hello,");
    s.push_str("world");
    println!("{}", s);


     //动态数组
     let mut v: Vec<i32> = Vec::new();
     v.push(2);
     v.push(3);

     
     //固定大小数组
     let mut arr: [i32; 4] = [0,2,4,8];
     arr[0] = -2;
     println!("{}", arr[0]+arr[1]);


     //迭代器
     for i in arr.iter()
     {
         println!("{}",i);
     }


     //while
    let mut sum = 0;
    let mut i = 0;
     while i < 20 
     {
         i += 1;
         sum += i;

     }

     println!("sum={}",sum);


    //loop  它有助于编译器对变量初始化进行一些假设。
    let mut i = 0;
    loop {

        i += 1;

        if i == 10 {
            break;
        }
    }

    println!("i={}",i);

    //函数

    fn mysum(a: i32, b:i32) -> i32 
    {
        a + b  //Rust是一种基于表达式的 语言，不需要分号

        //a + b ; 会出错
    }
     
    println!("sum={}", mysum(1,2));

    
    //练习函数调用

}
