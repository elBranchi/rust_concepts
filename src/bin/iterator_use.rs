fn main() {
    // array_iteration();
    // vector_iteration();

    // iterator_combine();
    // position_use();
    palindrome_check();
}

#[allow(dead_code)]
fn array_iteration() {
    let mut a16 = [0i16; 300];

    // init array contents
    for i in 0..a16.len() {
        a16[i] = i as i16;
        // print!("{:?} ", a16[i]);
    }
    let mut sum: i32 = 0;
    for v in a16.iter() {
        sum += i32::from(*v);
    }
    println!("Sum of references {sum}");

    //using into_iter consumes the collection (not affecting arrays?)
    for v in a16.into_iter() {
        sum -= i32::from(v);
    }
    println!("After substraction of values {sum}");

    for v in a16.iter().step_by(3) {
        println!("{v:?}");
    }
}

#[allow(dead_code)]
fn vector_iteration() {
    let mut v8 = Vec::<i8>::with_capacity(120);

    let _v8cap = v8.capacity();
    for p in 0.._v8cap {
        let v = if p % 2 == 0 {
            1 + (p / 2)
        } else {
            _v8cap - p / 2
        };
        v8.push(v as i8);
        // v8[idx] = v;
    }

    // Collection not consumed
    for v in v8.iter() {
        print!("{v:?},");
    }
    // Collection consumed?
    for v in v8.into_iter() {
        print!("{v:?},");
    }
    println!("\ninto_iter used");
    // This will not compile as previous into_iter will consume the collection

    // for v in v8.iter() {
    //     print!("{v:?},");
    // }
}

#[allow(dead_code)]
fn iterator_combine() {
    let r1 = &[3i8, -7, 6];
    let r2 = &[-4i8, 1, 0];

    println!("r1 chain r2");
    for v in r1.iter().chain(r2) {
        print!("{v:?} ");
    }
    println!("");
    let (negatives, others) = r1
        .iter()
        .chain(r2)
        .partition::<Vec<i8>, _>(|x| x.is_negative());

    println!("negatives: {negatives:?}");
    println!("others: {others:?}");

    println!("\nr1 zip r2");
    for v in r1.iter().zip(r2) {
        print!("{v:?} ");
    }
}

#[allow(dead_code)]
fn position_use() {
    let a0 = [10, -2, 12, 14, 15, 22, -2];

    let p15 = a0
        .iter()
        .inspect(|x| println!("p15 checking {x}"))
        .position(|v| *v == 15);
    let pmin2 = a0
        .iter()
        .inspect(|x| println!("pmin2 checking {x}"))
        .position(|v| *v == -2);
    let p_r_min2 = a0
        .iter()
        .inspect(|x| println!("p_r_min2 checking {x}"))
        .rposition(|v| *v == -2);

    println!("p15={p15:?} pmin2={pmin2:?} p_r_min2={p_r_min2:?}");
}

#[allow(dead_code)]
fn palindrome_check() {
    let texts = ["some", "oso", "acurruca", "Acurruca", "la usa como casual"];

    for text in texts {
        let result = is_palindrome(text);
        println!("Text \"{text}\" result = {result}");
    }
}

fn is_palindrome(text: &str) -> bool {
    let chars: Vec<_> = text.chars().filter(|c| c.is_alphanumeric()).collect();

    let mut palindrome = true;
    let mid = chars.len() / 2;

    for i in 0..=mid {

        if ! chars[i].eq_ignore_ascii_case(&chars[chars.len() - (i + 1)]) {
            palindrome = false;
            break;
        }
        // if chars[i] != chars[chars.len() - (i + 1)] {
        //     palindrome = false;
        //     break;
        // }
    }

    palindrome
}
