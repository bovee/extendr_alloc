use extendr_api::prelude::*;

struct Test {}

#[extendr]
impl Test {
    #[allow(clippy::new_ret_no_self)]
    fn new() -> Result<Robj> {
        Ok(Test {}.into())
    }
}

#[extendr]
fn as_data_frame(_test: &mut Test) -> Result<Robj> {
    const COUNT: u64 = 2_000_000u64;
    let mut data: Vec<Robj> = vec![];
    for i in 1u64..COUNT {
        data.push(i.into());
    }

    for (ix, v) in data.iter().enumerate() {
        if Some((ix + 1) as f64) != v.as_real() {
            println!("BAD {} {}", ix + 1, v.as_real().unwrap());
        }
    }

    let obj: Robj = List::from_names_and_values(&["A"], vec![data]).into();
    obj.set_attrib(
        row_names_symbol(),
        (1i32..=COUNT as i32).collect_robj(),
    )?;
    obj.set_class(&["data.frame"])?;
    Ok(obj)
}

extendr_module! {
    mod extendr_alloc;
    impl Test;
    fn as_data_frame;
}
