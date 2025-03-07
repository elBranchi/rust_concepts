use std::{cell::RefCell, collections::VecDeque, rc::Rc};

fn main() {
    use_shared_values();
}

fn check_restrictions(){

    let mut s = String::new();

    let s2 = &s;
    // Uncommenting this line will trigger a compilation error as at this point
    // s is borrowed by s2 as inmutable  and it's still used/live after changing s
    // s.push_str("ch");

    //Also at this point is not possible to get a mutable reference to s, as s2 is still in use
    // let s3 = &mut s;
    print!("{s2}");
    let s4 = &mut s;
    s4.push_str("w");
    s.push_str("ch");
}

fn use_shared_values() {
    let mut shared: VecDeque<Rc<RefCell<String>>> = VecDeque::new();

    {
        for i in 1..=7 {
            let rc = RefCell::new(i.to_string());
            let s = Rc::new(rc);
            let last = shared.pop_back();
            if let Some(item) = last {
                shared.push_back(s.clone());
                shared.push_front(item);
            }
            shared.push_back(s);
        }
    }

    shared.iter().for_each(|x| print!("{} ", x.borrow()));
    {
        let mut elem = shared[3].borrow_mut();

        elem.push_str("#");
    }
    println!("After:");
    for item in shared.iter() {
        print!("{} ", item.borrow());
    }
}

fn use_min() {
    let v1 = 10u16;

    {
        let v2 = 25u16;
        {
            let v3 = 0u16;

            let min_v1_v2 = min(&v1, &v2);
            let min_v2_v3 = min(&v2, &v3);

            println!("min_v1_v2 {min_v1_v2} ");
            println!("min_v2_v3 {min_v2_v3} ");
        }
    }
}

fn min<'a, 'b, 'c, T: PartialOrd>(val1: &'a T, val2: &'b T) -> &'c T
where
    'a: 'c,
    'b: 'c,
{
    if val1 < val2 { val1 } else { val2 }
}
