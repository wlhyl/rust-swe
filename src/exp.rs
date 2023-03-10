pub enum Calendar {
    Gregorian,
    Julian,
}

impl From<Calendar> for i32 {
    fn from(value: Calendar) -> Self {
        match value {
            Calendar::Gregorian => 1,
            Calendar::Julian => 0,
        }
    }
}
pub enum Planet {
    SUN = 0,
    MOON = 1,
    MERCURY = 2,
    VENUS = 3,
    MARS = 4,
    JUPITER = 5,
    SATURN = 6,
    URANUS = 7,
    NEPTUNE = 8,
    PLUTO = 9,
    MeanNode = 10,
    TrueNode = 11,
    MeanApog = 12,
    OscuApog = 13,
    EARTH = 14,
    CHIRON = 15,
    PHOLUS = 16,
    CERES = 17,
    PALLAS = 18,
    JUNO = 19,
    VESTA = 20,
}
impl From<Planet> for i32 {
    fn from(value: Planet) -> Self {
        value as i32
    }
}
pub enum Flag {
    SeflgJpleph,      // use JPL ephemeris
    SeflgSwieph,      // use SWISSEPH ephemeris, default
    SeflgMoseph,      // use Moshier ephemeris
    SeflgHelctr,      // return heliocentric position
    SeflgTruepos,     // return true positions, not apparent
    SeflgJ2000,       // no precession, i.e. give J2000 equinox
    SeflgNonut,       // no nutation, i.e. mean equinox of date
    SeflgSpeed3, // speed from 3 positions (do not use it, SEFLG_SPEED is faster and more precise.)
    SeflgSpeed,  // high precision speed (analyt. comp.)
    SeflgNogdefl, // turn off gravitational deflection
    SeflgNoaberr, // turn off 'annual' aberration of light
    SeflgAstrometric, // astrometric positions
    SeflgEquatorial, // equatorial positions are wanted
    SeflgXyz,    // cartesian, not polar, coordinates
    SeflgRadians, // coordinates in radians, not degrees
    SeflgBaryctr, // barycentric positions
    SeflgTopoctr, // topocentric positions

    SeflgOrbelAa, /* used for Astronomical Almanac mode in calculation of Kepler elipses */
    SeflgTropical, /* tropical position (default) */
    SeflgSidereal, // sidereal positions
    SeflgIcrs,    // ICRS (DE406 reference frame)
    SeflgDpsideps1980, // reproduce JPL Horizons 1962 - today to 0.002 arcsec.

    SeflgJplhor,

    SeflgJplhorApprox, // approximate JPL Horizons 1962 - today

    SeflgCenterBody, // calculate position of center of body (COB) of planet, not barycenter of its system of planet, not barycenter of its system
    SeflgTestPlmoon, /* test raw data in files sepm9* */
}

impl From<Flag> for i32 {
    fn from(value: Flag) -> Self {
        (&value).into()
    }
}

impl From<&Flag> for i32 {
    fn from(value: &Flag) -> Self {
        match value {
            Flag::SeflgJpleph => 1, /* use JPL ephemeris */
            Flag::SeflgSwieph => 2, /* use SWISSEPH ephemeris */
            Flag::SeflgMoseph => 4, /* use Moshier ephemeris */

            Flag::SeflgHelctr => 8,     /* heliocentric position */
            Flag::SeflgTruepos => 16,   /* true/geometric position, not apparent position */
            Flag::SeflgJ2000 => 32,     /* no precession, i.e. give J2000 equinox */
            Flag::SeflgNonut => 64,     /* no nutation, i.e. mean equinox of date */
            Flag::SeflgSpeed3 => 128, /* speed from 3 positions (do not use it, SEFLG_SPEED is faster and more precise.) */
            Flag::SeflgSpeed => 256,  /* high precision speed  */
            Flag::SeflgNogdefl => 512, /* turn off gravitational deflection */
            Flag::SeflgNoaberr => 1024, /* turn off 'annual' aberration of light */
            Flag::SeflgAstrometric => i32::from(Flag::SeflgNoaberr) | i32::from(Flag::SeflgNogdefl), /* astrometric position, i.e. with light-time, but without aberration and light deflection */
            Flag::SeflgEquatorial => 2 * 1024, /* equatorial positions are wanted */
            Flag::SeflgXyz => 4 * 1024,        /* cartesian, not polar, coordinates */
            Flag::SeflgRadians => 8 * 1024,    /* coordinates in radians, not degrees */
            Flag::SeflgBaryctr => 16 * 1024,   /* barycentric position */
            Flag::SeflgTopoctr => 32 * 1024,   /* topocentric position */
            Flag::SeflgOrbelAa => Flag::SeflgTopoctr.into(), /* used for Astronomical Almanac mode in calculation of Kepler elipses */
            Flag::SeflgTropical => 0,                        /* tropical position (default) */
            Flag::SeflgSidereal => 64 * 1024,                /* sidereal position */
            Flag::SeflgIcrs => 128 * 1024,                   /* ICRS (DE406 reference frame) */
            Flag::SeflgDpsideps1980 => 256 * 1024, /* reproduce JPL Horizons 1962 - today to 0.002 arcsec. */
            Flag::SeflgJplhor => Flag::SeflgDpsideps1980.into(),
            Flag::SeflgJplhorApprox => 512 * 1024, /* approximate JPL Horizons 1962 - today */
            Flag::SeflgCenterBody => 1024 * 1024, /* calculate position of center of body (COB) of planet, not barycenter of its system */
            Flag::SeflgTestPlmoon => {
                2 * 1024 * 1024
                    | i32::from(Flag::SeflgJ2000)
                    | i32::from(Flag::SeflgIcrs)
                    | i32::from(Flag::SeflgHelctr)
                    | i32::from(Flag::SeflgTruepos)
            } /* test raw data in files sepm9* */
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::exp::Planet;

    #[test]
    fn test_planet_to_i32() {
        assert_eq!(i32::from(Planet::SUN), 0);
        assert_eq!(i32::from(Planet::MOON), 1);
        assert_eq!(i32::from(Planet::MERCURY), 2);
        assert_eq!(i32::from(Planet::VENUS), 3,);
        assert_eq!(i32::from(Planet::MARS), 4);
        assert_eq!(i32::from(Planet::JUPITER), 5);
        assert_eq!(i32::from(Planet::SATURN), 6);
        assert_eq!(i32::from(Planet::URANUS), 7);
        assert_eq!(i32::from(Planet::NEPTUNE), 8);
        assert_eq!(i32::from(Planet::PLUTO), 9);
        assert_eq!(i32::from(Planet::MeanNode), 10);
        assert_eq!(i32::from(Planet::TrueNode), 11);
        assert_eq!(i32::from(Planet::MeanApog), 12);
        assert_eq!(i32::from(Planet::OscuApog), 13);
        assert_eq!(i32::from(Planet::EARTH), 14,);
        assert_eq!(i32::from(Planet::CHIRON), 15);
        assert_eq!(i32::from(Planet::PHOLUS), 16);
        assert_eq!(i32::from(Planet::CERES), 17);
        assert_eq!(i32::from(Planet::PALLAS), 18);
        assert_eq!(i32::from(Planet::JUNO), 19);
        assert_eq!(i32::from(Planet::VESTA), 20);
    }
}
