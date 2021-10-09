use std::collections::HashMap;
use std::time::{SystemTime};

fn main() {
    let before = SystemTime::now();

    let seqs = &mut coll_seqs(999999);
    let max_seq = max_size_seq(seqs);
    println!("{:?}, {:?}", max_seq.0, max_seq.1);

    println!("enlapsed time: {:?}", before.elapsed().expect("Cannot calculate time"));
}

fn max_size_seq(seqs: &mut HashMap<usize, usize>) -> (usize, usize) {
    let mut seqv: Vec<(&usize, &usize)> = seqs.iter().collect();
    seqv.sort_by(|x, y| y.1.partial_cmp(&x.1).unwrap());
    let first = seqv.first().expect("Error");
    (*first.0, *first.1)
}

fn coll_seqs(max: usize) -> HashMap<usize, usize> {
    let mut res = HashMap::new();
    for n in 1..max {
        let r = coll_seq(n, &res);
        res.insert(n, r);
    } 
    res
}

fn coll_seq(n: usize, cache: &HashMap<usize, usize>) -> usize {
    // let mut acc = 1;
    // let mut x = n;
    // while x > 1 {
    //     if let Some(y) = cache.get(&x) {
    //         acc = acc + y;
    //         break;
    //     }
    //     acc = acc + 1; 
    //     x = cool_number(&x);
    // }

    // acc
    if let Some(x) = cache.get(&n) {
        *x
    } else {
        if n > 1 {
            1 + coll_seq(cool_number(n), cache)
        } else {
            1
        }
    }
}

fn cool_number(n: usize) -> usize {
    if n % 2 == 0 {
        n/2
    } else {
        3 * n + 1
    }
}



