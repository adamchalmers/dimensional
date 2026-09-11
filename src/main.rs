use std::assert_matches;

use dimensional::Dimensional;

fn main() {
    use std::f64::consts::PI;

    println!(
        "
# Dimensional analysis prototype

Provides a type `Dimensional` which can track lengths (metric or imperial)
and angles (degrees or radians), allowing you to do arithmetic with them.
"
    );
    println!("\n\n## Equality\n");
    println!("```");
    let a = Dimensional::mm(1.0);
    let b = Dimensional::mm(1.0);
    assert_eq!(a, b);
    println!("{a} == {b}");

    let a = Dimensional::cm(1.0);
    let b = Dimensional::mm(10.0);
    assert_eq!(a, b);
    println!("1cm == {b}");

    let a = Dimensional::cm2(1.0);
    let b = Dimensional::mm(100.0) * Dimensional::mm(1.0);
    assert_eq!(a, b);
    println!("1cm² == {b}");

    let a = Dimensional::mm(25.4);
    let b = Dimensional::inches(1.0);
    assert_eq!(a, b);
    println!("{a} == {b}");

    let a = Dimensional::feet(1.0);
    let b = Dimensional::inches(12.0);
    assert_eq!(a, b);
    println!("1ft == {b}");

    let a = Dimensional::radians(2.0 * PI);
    let b = Dimensional::degrees(360.0);
    assert_eq!(a, b);
    println!("{a} == {b}");

    let a = Dimensional::mm(1.0);
    let b = Dimensional::degrees(10.0);
    assert_ne!(a, b);
    println!("{a} != {b}");

    let a = Dimensional::mm(1.0);
    let b = Dimensional::unitless(1.0);
    assert_ne!(a, b);
    println!("{a} != {b}");

    println!("```");

    println!("\n\n## Addition\n");

    println!("```");

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
    let y = Dimensional::radians(2.0 * PI);
    println!("{x} + {y} == {}", x + y);
    assert_eq!(x + y, Dimensional::degrees(720.0));

    let x = Dimensional::radians(0.0);
    let y = Dimensional::degrees(0.0);
    println!("{x} + {y} == {}", x + y);
    assert_eq!(x + y, Dimensional::radians(0.0));

    println!("```");

    println!("\n\n## Incompatible additions\n");

    println!("```");

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

    println!("```");

    println!("\n\n## Scaling a length\n");
    println!("```");

    let x = Dimensional::mm(10.0);
    let y = Dimensional::unitless(2.0);
    println!("{x} * {y} == {}", x * y);
    let x = Dimensional::mm(10.0);
    let y = Dimensional::unitless(2.0);
    println!("{x} / {y} == {}", x / y);

    println!("```");

    println!("\n\n## Areas\n");
    println!("```");

    let a = Dimensional::mm(2.0);
    let b = Dimensional::mm(3.0);
    println!("{a} * {b} == {}", a * b);
    let x = Dimensional::mm(10.0);
    let y = Dimensional::inches(1.0);
    println!("{x} * {y} == {}", x * y);
    let two_inches = Dimensional::mm(25.4 * 2.0);
    println!("{y} * {two_inches} == {}", y * two_inches);
    let one = Dimensional::degrees(1.0);
    let full_circle = Dimensional::radians(2.0 * PI);
    println!("{one} * {full_circle} == {}", one * full_circle);
    let a = Dimensional::mm(2.0) * Dimensional::mm(1.0);
    let b = Dimensional::mm(3.0) * Dimensional::mm(1.0);
    println!("{a} + {b} == {}", a + b);

    println!("```");

    println!("\n\n## Division removes dimensions\n");
    println!("```");

    let a = Dimensional::mm(2.0) * Dimensional::mm(1.0);
    let b = Dimensional::mm(4.0);
    println!("{a} / {b} == {}", a / b);

    let a = Dimensional::mm(2.0);
    let b = Dimensional::mm(4.0);
    println!("{a}  / {b} == {}", a / b);

    let q = Dimensional::unitless(2.0);
    let r = Dimensional::mm(4.0);
    println!("{q}   / {r} == {}", q / r);
    println!("```");

    println!("\n\n## Mixed units\n");
    println!("```");
    let a = Dimensional::mm(2.0);
    let b = Dimensional::degrees(4.0);
    println!("{a} * {b} == {}", a * b);
    println!("{a} * {b} / {a} == {}", a * b / a);
    let double_ab0 = a * b + a * b;
    println!("{a} * {b} + {a} * {b} == {}", double_ab0);
    let double_ab1 = a * b * Dimensional::unitless(2.0);
    println!("{a} * {b} * 2_ == {}", double_ab1);
    assert_eq!(double_ab0, double_ab1);

    println!("```");
}
