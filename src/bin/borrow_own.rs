// Show/study details about ownership and borrowing variables

#[derive(Debug, Default)]
struct PointNoCopy {
    x: i16,
    y: i16,
}


// virtually identical struct but with Copy trait
// as Copy is extending Clone, Clone needs to be derived too 
// or implemented too, but if Clone needs an implementation and type should still be Copy
// there might be something not ok with the type
#[derive(Debug, Default, Clone, Copy)]
struct Point {
    x: i16,
    y: i16,
}

impl PointNoCopy {
    fn new(x: i16, y: i16) -> PointNoCopy {
        PointNoCopy { x, y }
    }
}

impl From<(i16, i16)> for PointNoCopy {
    fn from(value: (i16, i16)) -> Self {
        PointNoCopy::new(value.0, value.1)
    }
}

impl Point {
    fn new(x: i16, y: i16) -> Point{

        Point {x, y}
    }
}
impl From<(i16, i16)> for Point{
    fn from(value: (i16, i16)) -> Self {
        Point { x: value.0, y: value.1 }
    }
}
fn main() {
    no_copy_trait_samples();
    copy_trait_samples();

    multiple_readonly_references();
    ownership_and_functions();
    references_as_params_to_functions();
}

fn ownership_and_functions() {
    let p1 = PointNoCopy::new(10, 12);
    let p2 = PointNoCopy::new(10, 14);
    let p3 = Point::new(5, 10);

    let r1 = apply_func_on_reference(&p1);

    // here p2 is "moved" into apply_func_on_value
    // ownership of p2 is transferred to the function
    let r2 = apply_func_on_value_no_copy(p2);
    
    let r3 = apply_func_on_value_copy(p3);


    println!("r1 = {r1} / r2={r2} / r3={r3}");
    println!("p1 = {p1:?}");
    // As p3 type (Point) implements Copy, no move occurs, a copy of p3 was used by the function
    // and p3 can be still used afterwards
    println!("p3 = {p3:?}");

    //as p2 was moved into do_something this won't compile
    // println!("p2 = {p2:?}")

    fn apply_func_on_value_no_copy(v: PointNoCopy) -> i16 {
        v.x - v.y
    }
    fn apply_func_on_value_copy(v: Point) -> i16 {
        v.x * v.y
    }
    fn apply_func_on_reference(v: &PointNoCopy) -> i16 {
        v.x + v.y
    }
}


fn no_copy_trait_samples() {

    no_copy_trait_move();
    single_mutable_reference_no_copy();
}

fn copy_trait_samples() {

    copy_trait_move();
    single_mutable_reference_copy();
}

fn no_copy_trait_move() {
    let a = PointNoCopy::default();

    println!("{a:?}");

    // PointNoCopy does not implement Copy trait, so the following assignment
    // "moves" the point from a to b, a is no longer valid after the assignment
    let b = a;
    println!("{b:?}");

    // Next println won't compile: previous assignment "moved" the information from a to b
    // and a no longer "contains" a value

    // println!("{a:?}");
}

fn copy_trait_move(){

    // When a type has Copy trait the values do not get moved as 
    // copies can be created and used instead
    let a = Point::default();

    println!("{a:?}");

    //unlike on no_copy_trait_move a does not get moved, and we can still use a later
    let b = a;
    println!("{b:?}");
    // a still usable
    println!("{a:?}");
}

fn multiple_readonly_references() {
    let mut c = PointNoCopy::new(2, 3);

    // many read-only reference to a single value can coexist
    // likely doing something like this in real code is likely to be problematic
    let refs = [&c; 5];
    for v in refs {
        print!("{v:?}");
    }

    // cannot borrow a mutable reference at this point (as refs is still used afterwards)
    // let mut_c = &mut c;

    println!("{:?}", refs.first());

    //But we can do it here as refs is no longer in use
    let mut_c = &mut c;
    mut_c.y += 10;
    print!("{mut_c:?}\n");
}

fn single_mutable_reference_no_copy() {
    let mut c = PointNoCopy::new(2, 3);
    let d = PointNoCopy::new(33, -2);

    //cannot create a mutable reference from a non-mutable variable, the following won't compile
    // let mut_d = &mut d;
    let mut_c = &mut c;
    // When a mutable reference exists/has been borrowed, no additional reference
    // to same location can exist neither read only nor mutable, this following line won't compile
    // let mut_c2 = &mut c;

    // when mutable reference borrow exists, cannot borrow readonly at same time
    {
        // let refs = [&c; 5];
        // for v in refs {
        //     print!("{v:?}");
        // }
    }

    // with this modification, we keep mut_c alive at this point and prevents the ref borrow for refs value to be possible
    mut_c.x += 1;
    print!("{c:?} {d:?}\n");
}

fn single_mutable_reference_copy(){

    // for references, the fact that the value has Copy trait or not, changes hardly anything
    let mut c = Point::new(0, 2);
    let d = Point::new(-6, 2);

    let mut_c = &mut c;
    // as mut_c is being still used, following line causes compiler error
    // let mut_c2 = &mut c;

    

    // as in single_mutable_reference_no_copy no readonly reference can be borrowed
    // while a mutable reference is considered alive
    {
        // let refs = [&c; 3];
        // for v in refs {
        //     print!("{v:?} ");
        // }
    }

    println!("{mut_c:?} ");
    mut_c.x += 33;
    // take into account that if the 1st mutable is not used after 2nd one has been initialized
    // then compiler does not complain :)
    let mut_c = &mut c;
    let mut_c2 = &mut c; //at this point mut_c can be thought as no longer existing 
    
    mut_c2.y += 10;
}

fn references_as_params_to_functions() {
    let p1 = PointNoCopy::new(10, 12);
    let p2 = PointNoCopy::new(10, 14);

    //variable shadowing, we can redeclare variables
    // with same name, even changing the type! something to be careful about
    let mut p2 = PointNoCopy::default();
    let mut p3 = PointNoCopy::new(9, 99);
    let mut p2 = PointNoCopy::default();
    let mut p3 = PointNoCopy::new(9, 99);
    let p4 = PointNoCopy::new(-10, 20);

    let a = &p1;

    let b = &mut p2;
    let c = &mut p3;
    let d = &p4;

    println!("pre mut_play\n\ta={a:?}\n\tb={b:?}\n\tc={c:?}\n\td={d:?}");
    
    apply_no_specified_lifetimes(a, b, c, d);
    println!("post mut_play / pre mut_lifetime_play\n\ta={a:?}\n\tb={b:?}\n\tc={c:?}\n\td={d:?}");
    
    apply_with_lifetimes(b, c);
    println!("post mut_lifetime_play\n\ta={a:?}\n\tb={b:?}\n\tc={c:?}\n\td={d:?}");
    
    fn apply_no_specified_lifetimes(
        mut a: &PointNoCopy,
        b: &mut PointNoCopy,
        mut c: &mut PointNoCopy,
        d: &PointNoCopy,
    ) {
        let rep_a = PointNoCopy::new(-1, -1);
        let rep_b = PointNoCopy::new(-1, -1);
        // a is mutable and holds a non mutable reference, so the object contents cannot be modified

        // this won't compile
        // a.x += 1;

        // but as it is mutable, we could try to change a to point to some other PointNoCopy
        // a = &rep_a;
        // previous line won't compile as rep_a might not live enough
        // but even when using a value without problematic lifetime, changes on a itself wouldn't be
        // visible/noticeable outside the function

        println!("\nrep_a.x = {:?}", rep_a.x);
        //but as a is mutable, we could make it point to d (as soon as we provide additional lifetime on function signature)
        // a = d;

        // we can change contents of Point referred/pointed by b
        (b.x, b.y) = (rep_b.x, rep_b.y);

        //But we cannot change b to point somewhere else as its not mutable
        // b = c;

        //can we mess with b and c ?
        // we need lifetimes in order to be able to get this to compile

        // c = b;
        c.x += 10;
        c.y += -15;
    }

    fn apply_with_lifetimes<'b, 'c>(b: &'b mut PointNoCopy, mut c: &'c mut PointNoCopy)
    where
        'b: 'c, //lifetime 'b contains/envelopes 'c
    {
        c.x += b.x;
        c.y += b.y;
        c = b; //just don't get confused, whatever c was used when invoking mut_lifetime_play, it will stay unaltered
        // only the object pointed by c will be altered
        // b.x += 10; // this will not compile as b has been borrowed on previous assignment
        c.y += -15;
    }
}
