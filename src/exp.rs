#[derive(Clone)]
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


pub enum Body {
    SeEclNut,
    SeSun,
    SeMoon,
    SeMercury,
    SeVenus,
    SeMars,
    SeJupiter,
    SeSaturn,
    SeUranus,
    SeNeptune,
    SePluto,
    SeMeanNode,
    SeTrueNode,
    SeMeanApog,
    SeOscuApog,
    SeEarth,
    SeChiron,
    SePholus,
    SeCeres,
    SePallas,
    SeJuno,
    SeVesta,
    SeIntpApog,
    SeIntpPerg,
    SeNplanets,
    SeFictOffset, // offset for fictitious objects
    SeNfictElem,
    SePlmoonOffset, // offset for planetary moons
    SeAstOffset,    // offset for asteroids
    /* Hamburger or Uranian "planets" */
    SeCupido,
    SeHades,
    SeZeus,
    SeKronos,
    SeApollon,
    SeAdmetos,
    SeVulkanus,
    SePoseidon,
    /* other fictitious bodies */
    SeIsis,
    SeNibiru,
    SeHarrington,
    SeNeptuneLeverrier,
    SeNeptuneAdams,
    SePlutoLowell,
    SePlutoPickering,
}
impl From<&Body> for i32 {
    fn from(value: &Body) -> Self {
        match value {
            Body::SeEclNut => -1,
            Body::SeSun => 0,
            Body::SeMoon => 1,
            Body::SeMercury => 2,
            Body::SeVenus => 3,
            Body::SeMars => 4,
            Body::SeJupiter => 5,
            Body::SeSaturn => 6,
            Body::SeUranus => 7,
            Body::SeNeptune => 8,
            Body::SePluto => 9,
            Body::SeMeanNode => 10,
            Body::SeTrueNode => 11,
            Body::SeMeanApog => 12,
            Body::SeOscuApog => 13,
            Body::SeEarth => 14,
            Body::SeChiron => 15,
            Body::SePholus => 16,
            Body::SeCeres => 17,
            Body::SePallas => 18,
            Body::SeJuno => 19,
            Body::SeVesta => 20,
            Body::SeIntpApog => 21,
            Body::SeIntpPerg => 22,
            Body::SeNplanets => 23,
            Body::SeFictOffset => 40, // offset for fictitious objects
            Body::SeNfictElem => 15,
            Body::SePlmoonOffset => 9000, // offset for planetary moons
            Body::SeAstOffset => 10000,   // offset for asteroids
            /* Hamburger or Uranian "planets" */
            Body::SeCupido => 40,
            Body::SeHades => 41,
            Body::SeZeus => 42,
            Body::SeKronos => 43,
            Body::SeApollon => 44,
            Body::SeAdmetos => 45,
            Body::SeVulkanus => 46,
            Body::SePoseidon => 47,
            /* other fictitious bodies */
            Body::SeIsis => 48,
            Body::SeNibiru => 49,
            Body::SeHarrington => 50,
            Body::SeNeptuneLeverrier => 51,
            Body::SeNeptuneAdams => 52,
            Body::SePlutoLowell => 53,
            Body::SePlutoPickering => 54,
        }
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

#[derive(Clone)]
pub enum HouseSystem {
    B, // Alcabitus
    Y, //         APC houses
    X, //         Axial rotation system / Meridian system / Zariel
    H, //         Azimuthal or horizontal system
    C, //         Campanus
    F, //         Carter "Poli-Equatorial"
    A, // or ‘E’  Equal (cusp 1 is Ascendant)
    E,
    D, //         Equal MC (cusp 10 is MC)
    N, // Equal/1=Aries
    G, /* Gauquelin sector
        * Goelzer -> Krusinski
        * Horizontal system -> Azimuthal system */
    I,       // Sunshine (Makransky, solution Treindl)
    ILowwer, // Sunshine (Makransky, solution Makransky)
    K,       // Koch
    U,       /* Krusinski-Pisa-Goelzer
             Meridian system -> axial rotation */
    M, /* Morinus
        * Neo-Porphyry -> Pullen SD
        * Pisa -> Krusinski */
    P, /* Placidus
        * Poli-Equatorial -> Carter */
    T, // olich/Page (“topocentric” system)
    O, // Porphyrius
    L, // Pullen SD (sinusoidal delta) – ex Neo-Porphyry
    Q, // Pullen SR (sinusoidal ratio)
    R, // Regiomontanus
    S, /* Sripati
       “Topocentric” system -> Polich/Page */
    V, // Vehlow equal (Asc. in middle of house 1)
    W, /* Whole sign
        * Zariel -> Axial rotation system */
}

impl From<&HouseSystem> for char {
    fn from(value: &HouseSystem) -> Self {
        match value {
            HouseSystem::B => 'B',
            HouseSystem::Y => 'Y',
            HouseSystem::X => 'X',
            HouseSystem::H => 'H',
            HouseSystem::C => 'C',
            HouseSystem::F => 'F',
            HouseSystem::A => 'A',
            HouseSystem::E => 'E',
            HouseSystem::D => 'D',
            HouseSystem::N => 'N',
            HouseSystem::G => 'G',
            HouseSystem::I => 'I',
            HouseSystem::ILowwer => 'i',
            HouseSystem::K => 'K',
            HouseSystem::U => 'U',
            HouseSystem::M => 'M',
            HouseSystem::P => 'P',
            HouseSystem::T => 'T',
            HouseSystem::O => 'O',
            HouseSystem::L => 'L',
            HouseSystem::Q => 'Q',
            HouseSystem::R => 'R',
            HouseSystem::S => 'S',
            HouseSystem::V => 'V',
            HouseSystem::W => 'W',
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::exp::Body;

    #[test]
    fn test_planet_to_i32() {
        assert_eq!(i32::from(&Body::SeSun), 0);
        assert_eq!(i32::from(&Body::SeMoon), 1);
        assert_eq!(i32::from(&Body::SeMercury), 2);
        assert_eq!(i32::from(&Body::SeVenus), 3,);
        assert_eq!(i32::from(&Body::SeMars), 4);
        assert_eq!(i32::from(&Body::SeJupiter), 5);
        assert_eq!(i32::from(&Body::SeSaturn), 6);
        assert_eq!(i32::from(&Body::SeUranus), 7);
        assert_eq!(i32::from(&Body::SeNeptune), 8);
        assert_eq!(i32::from(&Body::SePluto), 9);
        assert_eq!(i32::from(&Body::SeMeanNode), 10);
        assert_eq!(i32::from(&Body::SeTrueNode), 11);
        assert_eq!(i32::from(&Body::SeMeanApog), 12);
        assert_eq!(i32::from(&Body::SeOscuApog), 13);
        assert_eq!(i32::from(&Body::SeEarth), 14,);
        assert_eq!(i32::from(&Body::SeChiron), 15);
        assert_eq!(i32::from(&Body::SePholus), 16);
        assert_eq!(i32::from(&Body::SeCeres), 17);
        assert_eq!(i32::from(&Body::SePallas), 18);
        assert_eq!(i32::from(&Body::SeJuno), 19);
        assert_eq!(i32::from(&Body::SeVesta), 20);
    }
}
