use super::raw;

use super::exp::HouseSystem;

pub fn swe_houses(
    tjd_ut: f64,
    geolat: f64,
    geolon: f64,
    hsys: &HouseSystem,
) -> Result<(Vec<f64>, [f64; 10]), ()> {
    let ascmc = [0.0; 10];
    let cusps = [0.0; 37];
    let house: char = hsys.into();
    let error = unsafe {
        raw::swe_houses(
            tjd_ut,
            geolat,
            geolon,
            house as i32,
            cusps.as_ptr(), /* array for 13 (or 37 for system G) doubles, explained further below */
            ascmc.as_ptr(), /* array for 10 doubles, explained further below */
        )
    };

    if error == -1 {
        return Err(());
    }

    let cups = if house == 'G' {
        cusps[0..37].to_vec()
    } else {
        cusps[0..13].to_vec()
    };

    Ok((cups, ascmc))
}
