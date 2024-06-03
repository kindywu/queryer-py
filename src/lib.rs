#![allow(clippy::needless_option_as_deref)]
use pyo3::prelude::*;

#[pyfunction]
pub fn example_sql() -> PyResult<String> {
    Ok(example_sql_inner())
}

#[pyfunction]
pub fn query(sql: &str, suffix: &str) -> PyResult<String> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut data = rt.block_on(async { queryer_sql_polars::query(sql, suffix).await.unwrap() });

    Ok(data.to_csv().unwrap())
}

#[pymodule]
fn queryer_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(query, m)?)?;
    m.add_function(wrap_pyfunction!(example_sql, m)?)?;
    Ok(())
}

fn example_sql_inner() -> String {
    let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";

    let sql = format!(
        "SELECT location name, total_cases, new_cases, total_deaths, new_deaths \
        FROM {} where new_deaths >= 10 ORDER BY new_cases DESC LIMIT 6 OFFSET 5",
        url
    );

    sql
}
