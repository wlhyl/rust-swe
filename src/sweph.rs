use std::ffi::c_char;

use crate::{Body, Flag};

mod raw {
    use std::os::raw::{c_char, c_double, c_int};
    #[link(name = "swe")]
    extern "C" {
        // tjd_ut   = Julian day, Universal Time
        // ipl      = body number
        // iflag    = a 32 bit integer containing bit flags that indicate what kind of computation is wanted
        // xx       = array of 6 doubles for longitude, latitude, distance, speed in long., speed in lat., and speed in dist.
        // serr[256] = character string to return error messages in case of error.
        pub(crate) fn swe_calc_ut(
            tjd_ut: c_double,
            ipl: c_int,
            iflag: c_int,
            xx: *const c_double,
            serr: *const c_char,
        ) -> c_int;

        // star name, returned star name 40 bytes
        // tjd_ut   = Julian day in Universal Time (swe_fixstar_ut())，这个是utc的jd
        // iflag     = an integer containing several flags that indicate what kind of computation is wanted
        // xx           = array of 6 doubles for longitude, latitude, distance, speed in long., speed in lat., and speed in dist.
        // serr[256] = character string to contain error messages in case of error.
        pub(crate) fn swe_fixstar2_ut(
            star: *const c_char,
            tjd_ut: c_double,
            iflag: c_int,
            xx: *const c_double,
            serr: *const c_char,
        ) -> c_int;

    }
}

pub fn swe_calc_ut(tjd_ut: f64, ipl: &Body, iflag: &[Flag]) -> Result<[f64; 6], String> {
    let xx = [0.0; 6];
    let serr = [0; 255];
    let iflag = iflag.iter().fold(0, |acc, x| acc | i32::from(x));
    let iflgret =
        unsafe { raw::swe_calc_ut(tjd_ut, ipl.into(), iflag, xx.as_ptr(), serr.as_ptr()) };
    let serr = unsafe { std::ffi::CStr::from_ptr(serr.as_ptr()) };
    let serr = serr.to_str().unwrap().to_string();
    if iflgret < 0 || !serr.is_empty() {
        Err(serr)
    } else {
        Ok(xx)
    }
}

pub fn swe_fixstar2_ut(
    star: &str,
    tjd_ut: f64,
    iflag: &[Flag],
) -> Result<(String, [f64; 6]), String> {
    let mut star_name = [0; 41];
    let star = star.as_bytes();
    star_name[..star.len()].copy_from_slice(star);

    let xx = [0.0; 6];
    let serr = [0; 255];
    let iflag = iflag.iter().fold(0, |acc, x| acc | i32::from(x));

    let iflgret = unsafe {
        raw::swe_fixstar2_ut(
            star_name.as_ptr() as *const c_char,
            tjd_ut,
            iflag,
            xx.as_ptr(),
            serr.as_ptr(),
        )
    };

    let star_name = unsafe { std::ffi::CStr::from_ptr(star_name.as_ptr() as *const c_char) };
    let star_name = star_name.to_str().unwrap().to_string();

    let serr = unsafe { std::ffi::CStr::from_ptr(serr.as_ptr()) };
    let serr = serr.to_str().unwrap().to_string();

    if iflgret < 0 || !serr.is_empty() {
        Err(serr)
    } else {
        Ok((star_name, xx))
    }
}
