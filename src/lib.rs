mod exp;
mod house;
mod raw;
mod sweph;
mod swephlib;

pub use crate::exp::Calendar;
pub use exp::{Body, Flag, HouseSystem};
pub use house::swe_houses;
pub use sweph::swe_calc_ut;
pub use swephlib::{swe_cotrans, swe_degnorm};

use std::{ffi::CString, os::raw::c_char};

pub fn swe_version() -> String {
    let mut s = [0; 255];
    let p = s.as_mut_ptr();
    let s = unsafe {
        raw::swe_version(p);
        std::ffi::CStr::from_ptr(p)
    };
    s.to_str().unwrap().to_string()
}
pub fn swe_set_ephe_path(path: &str) {
    let path = CString::new(path).unwrap();
    unsafe {
        raw::swe_set_ephe_path(path.as_ptr());
    }
}

pub fn swe_close() {
    unsafe {
        raw::swe_close();
    }
}

pub fn swe_date_conversion(
    y: i32,
    m: i32,
    d: i32,
    hour: f64,
    c: Calendar, /* calendar ‘g’[regorian]|’j’[ulian] */
) -> Result<f64, ()> {
    let c: c_char = match c {
        Calendar::Gregorian => 103,
        Calendar::Julian => 106,
    };
    let mut tjd = 0.0;
    let error = unsafe { raw::swe_date_conversion(y, m, d, hour, c, &mut tjd) };
    if error == 0 {
        Ok(tjd)
    } else {
        Err(())
    }
}

pub fn swe_julday(year: i32, month: i32, day: i32, hour: f64, gregflag: Calendar) -> f64 {
    unsafe { raw::swe_julday(year, month, day, hour, gregflag.into()) }
}

pub fn swe_revjul(tjd: f64, gregflag: Calendar) -> (i32, i32, i32, f64) {
    let year = 0;
    let month = 0;
    let day = 0;
    let hour = 0.0;

    unsafe {
        raw::swe_revjul(tjd, gregflag.into(), &year, &month, &day, &hour);
    };

    (year, month, day, hour)
}

pub fn swe_utc_time_zone(
    iyear: i32,
    imonth: i32,
    iday: i32,

    ihour: i32,
    imin: i32,
    dsec: f64,

    d_timezone: f64,
) -> (i32, i32, i32, i32, i32, f64) {
    let iyear_out = 0;
    let imonth_out = 0;
    let iday_out = 0;

    let ihour_out = 0;
    let imin_out = 0;
    let dsec_out = 0.0;
    unsafe {
        raw::swe_utc_time_zone(
            iyear,
            imonth,
            iday,
            ihour,
            imin,
            dsec,
            d_timezone,
            &iyear_out,
            &imonth_out,
            &iday_out,
            &ihour_out,
            &imin_out,
            &dsec_out,
        );
    }

    (
        iyear_out, imonth_out, iday_out, ihour_out, imin_out, dsec_out,
    )
}

pub fn swe_utc_to_jd(
    iyear: i32,
    imonth: i32,
    iday: i32,
    ihour: i32,
    imin: i32,
    dsec: f64,
    gregflag: Calendar,
) -> Result<[f64; 2], String> {
    let dret = [0.0; 2];

    let serr = [0; 255];

    let error = unsafe {
        raw::swe_utc_to_jd(
            iyear,
            imonth,
            iday,
            ihour,
            imin,
            dsec,
            gregflag.into(),
            dret.as_ptr(),
            serr.as_ptr(),
        )
    };

    if error == 0 {
        Ok(dret)
    } else {
        let serr = unsafe { std::ffi::CStr::from_ptr(serr.as_ptr()) };
        let serr = serr.to_str().unwrap().to_string();
        Err(serr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swe_version() {
        assert_eq!(swe_version(), "2.10.03");
    }
}
