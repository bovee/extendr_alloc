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
    let row_names = ["A", "B", "C"];
    let mut data: Vec<Vec<Robj>> = vec![vec![]; row_names.len()];
    for i in 1..1_000_000 {
        for col_ix in 0..row_names.len() {
            data[col_ix].push(((col_ix + 1) * i).into());
        }
    }

    let mut vectors: Vec<Robj> = vec![];
    for v in data {
        vectors.push(v.into());
    }
    let obj: Robj = List::from_names_and_values(&row_names, &vectors).into();
    obj.set_attrib(
        row_names_symbol(),
        (1i32..=vectors[0].len() as i32).collect_robj(),
    )?;
    obj.set_class(&["data.frame"])?;
    Ok(obj)
}

extendr_module! {
    mod extendr_alloc;
    impl Test;
    fn as_data_frame;
}
