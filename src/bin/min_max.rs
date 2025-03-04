fn min<E: PartialOrd>(elems: &[E]) -> Option<&E> {
    let first = elems.first();

    if first.is_none() {
        Option::None
    } else {
        let mut min = first?;
        for next in elems[1..].iter() {
            if next < min {
                min = next;
            }
        }
        Some(min)
    }
}

fn max<E: PartialOrd>(elems: &[E]) -> Option<&E> {
    let first = elems.first();

    if first.is_none() {
        Option::None
    } else {
        let mut max = first?;
        for next in elems[1..].iter() {
            if next > max {
                max = next;
            }
        }
        Some(max)
    }
}

fn main() {
    let mut a8 = [0i8; 10];
    let mut a16 = [0i16; 16];
    let empty32: [i32; 0] = [];
    let single = [10i8; 1];

    let mut i = 0;
    while i < a8.len() {
        let value = 1i8 + i as i8;
        a8[i] = value;
        i += 1;
    }
    i = 0;
    while i < a16.len() {
        let value = 0i16 - (i as i16);
        a16[i] = value;
        i += 1;
    }
    let min_a8 = min(&a8);
    let max_a8 = max(&a8);
    let min_a16 = min(&a16);
    let max_a16 = max(&a16);
    let min_empty32 = min(&empty32);
    let max_empty32 = max(&empty32);
    let min_single = min(&single);
    let max_single = max(&single);

    println!(
        "min a8 {min_a8:?} / min a16 {min_a16:?} / empty {min_empty32:?} / single {min_single:?}"
    );
    println!(
        "max a8 {max_a8:?} / max a16 {max_a16:?} / empty {max_empty32:?} / single {max_single:?}"
    );
}
