use std::assert_matches;

use dimensional::Dimensional;

fn main() {
    print!("\n\n# ");
    println!("Addition\n");

    let x = Dimensional::mm(10.0);
    let y = Dimensional::mm(2.0);
    println!("{x} + {y} == {}", x + y);
    assert_eq!(x + y, Dimensional::mm(12.0));

    let x = Dimensional::mm(10.0);
    let y = Dimensional::mm(2.0);
    println!("{x} - {y} == {}", x - y);
    assert_eq!(x - y, Dimensional::mm(8.0));

    let x = Dimensional::mm(10.0);
    let y = Dimensional::cm(2.0);
    println!("{x} + {y} == {}", x + y);
    assert_eq!(x + y, Dimensional::mm(30.0));

    let x = Dimensional::mm(10.0);
    let y = Dimensional::inches(1.0);
    println!("{x} + {y} == {}", x + y);
    assert_eq!(x + y, Dimensional::mm(35.4));

    let x = Dimensional::degrees(360.0);
    let y = Dimensional::degrees(40.0);
    println!("{x} + {y} == {}", x + y);
    assert_eq!(x + y, Dimensional::degrees(400.0));

    let x = Dimensional::degrees(360.0);
    let y = Dimensional::radians(2.0 * std::f64::consts::PI);
    println!("{x} + {y} == {}", x + y);
    assert_eq!(x + y, Dimensional::degrees(720.0));

    print!("\n\n# ");
    println!("Incompatible additions\n");

    let x = Dimensional::mm(10.0);
    let y = Dimensional::radians(2.0);
    println!("{x} + {y} == {:?}", x.checked_add(y));
    assert_matches!(x.checked_add(y), Err(_));
    println!("{y} + {x} == {:?}", x.checked_add(y));
    assert_matches!(y.checked_add(x), Err(_));

    let x = Dimensional::cm(10.0);
    let y = Dimensional::cm2(10.0);
    println!("{x} + {y} == {:?}", x.checked_add(y));
    assert_matches!(x.checked_add(y), Err(_));

    let x = Dimensional::degrees(10.0);
    let y = Dimensional::degrees(10.0) * Dimensional::degrees(1.0);
    println!("{x} + {y} == {:?}", x.checked_add(y));
    assert_matches!(x.checked_add(y), Err(_));

    print!("\n\n# ");
    println!("Scaling a length\n");
    let x = Dimensional::mm(10.0);
    let y = Dimensional::unitless(2.0);
    println!("{x} * {y} == {}", x * y);
    let x = Dimensional::mm(10.0);
    let y = Dimensional::unitless(2.0);
    println!("{x} / {y} == {}", x / y);
    print!("\n\n# ");
    println!("Areas\n");
    let a = Dimensional::mm(2.0);
    let b = Dimensional::mm(3.0);
    println!("{a} * {b} == {}", a * b);
    let x = Dimensional::mm(10.0);
    let y = Dimensional::inches(1.0);
    println!("{x} * {y} == {}", x * y);
    let two_inches = Dimensional::mm(25.4 * 2.0);
    println!("{y} * {two_inches} == {}", y * two_inches);
    let one = Dimensional::degrees(1.0);
    let full_circle = Dimensional::radians(2.0 * std::f64::consts::PI);
    println!("{one} * {full_circle} == {}", one * full_circle);

    print!("\n\n# ");
    println!("Division removes dimensions\n");
    let a = Dimensional::mm(2.0) * Dimensional::mm(1.0);
    let b = Dimensional::mm(4.0);
    println!("{a} / {b} == {}", a / b);

    let a = Dimensional::mm(2.0);
    let b = Dimensional::mm(4.0);
    println!("{a}  / {b} == {}", a / b);

    let q = Dimensional::unitless(2.0);
    let r = Dimensional::mm(4.0);
    println!("{q}   / {r} == {}", q / r);

    print!("\n\n# ");
    println!("Mixed units\n");
    let a = Dimensional::mm(2.0);
    let b = Dimensional::degrees(4.0);
    println!("{a} * {b} == {}", a * b);
    println!("{a} * {b} / {a} == {}", a * b / a);
}
