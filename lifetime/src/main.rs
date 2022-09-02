/*
这是一个比较复杂的生命周期实际案例。
实现一个生成多项式的函数，参数为各项的参数，返回值是一个计算多项式的闭包函数。
*/

fn main() {
    let v: Vec<f32> = (0..=3).map(|x| x as f32).collect();
    let poly_fn = polynomial(&v);  // 生成y = 0 + 1*x + 2*x^2 + 3*x^3
    println!("{}", poly_fn(2.0));
}

/*
编译下面的函数，实现上述功能，但是会报如下错误：
error: lifetime may not live long enough
  --> src\main.rs:11:5
   |
10 |   fn polynomial(paras: &[f32]) -> Box<dyn Fn(f32) -> f32> {
   |                        - let's call the lifetime of this reference `'1`
11 | /     Box::new(
12 | |         |x| {
13 | |             paras.iter().zip(0..paras.len()).fold(0., |acc, (para, i)| acc + x.powf(i as f32) * para)
14 | |         }
15 | |     )
   | |_____^ returning this value requires that `'1` must outlive `'static`
   |
help: to declare that the trait object captures data from argument `paras`, you can add an explicit `'_` lifetime bound
   |
10 | fn polynomial(paras: &[f32]) -> Box<dyn Fn(f32) -> f32 + '_> {
   |                                                        ++++
编译器显示，返回的闭包存活周期要长于参数paras（因为paras是个引用，而闭包中的计算依赖paras，所以会出现生命周期问题）
help提示，如果trait从paras捕获值，那么改为fn polynomial(paras: &[f32]) -> Box<dyn Fn(f32) -> f32 + '_>即可。
*/

// fn polynomial(paras: &[f32]) -> Box<dyn Fn(f32) -> f32> {
//     Box::new(
//         |x| {
//             paras.iter().zip(0..paras.len()).fold(0., |acc, (para, i)| acc + x.powf(i as f32) * para)
//         }
//     )
// }


/*
按照编译器提示进行修改，发现可以运行。
*/

// fn polynomial(paras: &[f32]) -> Box<dyn Fn(f32) -> f32 + '_> {
//     Box::new(|x| {
//         paras.iter().zip(0..paras.len()).fold(0., |acc, (para, i)| acc + x.powf(i as f32) * para)
//     })
// }


/*
为了深入理解生命周期，我们可以这样写。
意思就是必须使paras的生命周期和返回闭包的生命周期一样，即他俩至少要存活一样的时间。
因为闭包的计算依赖于paras，若paras存活时间短于闭包，那么闭包计算时会遇到野指针，编译报错
*/

fn polynomial<'a>(paras: &'a [f32]) -> Box<dyn Fn(f32) -> f32 + 'a> {
    Box::new(|x| {
        paras.iter().zip(0..paras.len()).fold(0., |acc, (para, i)| acc + x.powf(i as f32) * para)
    })
}

/*
为了使闭包函数的使用脱离paras的生命周期限制，可将paras改为Rc指针，完整程序如下
*/

// use std::rc::Rc;


// fn main() {
//     let poly_fn = polynomial(
//         Rc::new((0..3).map(|x| x as f32).collect::<Vec<f32>>())
//     );
//     println!("{}", poly_fn(3.0));
// }

// fn polynomial(paras: Rc<Vec<f32>>) -> Box<dyn Fn(f32) -> f32> {
//     Box::new(move |x| {
//         paras.iter().zip(0..paras.len()).fold(0., |acc, (para, i)| acc + x.powf(i as f32) * para)
//     })
// }


