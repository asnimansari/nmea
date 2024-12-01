use nom::{character::complete::char, combinator::opt, number::complete::float};

use crate::{Error, NmeaSentence, SentenceType};

/// DBS - Depth Below Surface
///
/// <https://gpsd.gitlab.io/gpsd/NMEA.html#_dbs_depth_below_surface>
/// ```text
///         1   2 3   4 5   6 7
///         |   | |   | |   | |
///  $--DBT,x.x,f,x.x,M,x.x,F*hh<CR><LF>
/// Field Number:
///     1. Water depth, feet
///     2. f = feet
///     3. Water depth, meters
///     4. M = meters
///     5. Water depth, Fathoms
///     6. F = Fathoms
///     7. Checksum
/// In real-world sensors, sometimes not all three conversions are reported. So you might see something like $SDDBT,,f,22.5,M,,F*cs
/// Example: $SDDBT,7.8,f,2.4,M,1.3,F*0D

/// ```
///
pub struct DbsData {
    pub water_depth_feet: Option<f32>,
    pub water_depth_meters: Option<f32>,
    pub water_depth_fathoms: Option<f32>,
}

pub fn parse_dbs(sentence: NmeaSentence) -> Result<DbsData, Error> {
    if sentence.message_id != SentenceType::DBS {
        Err(Error::WrongSentenceHeader {
            expected: SentenceType::DBS,
            found: sentence.message_id,
        })
    } else {
        Ok(do_parse_dbs(sentence.data)?)
    }
}

fn do_parse_dbs(i: &str) -> Result<DbsData, Error> {
    let (i, water_depth_feet) = opt(float)(i)?;
    let (i, _) = char(',')(i)?;
    let (i, unit_feet) = char('f')(i)?;
    // todo->should we check for unit_feet?

    let (i, water_depth_meters) = opt(float)(i)?;
    let (i, _) = char(',')(i)?;
    let (i, unit_meters) = char('M')(i)?;
    // todo->should we check for unit_meters?

    let (i, water_depth_fathoms) = opt(float)(i)?;
    let (i, _) = char(',')(i)?;
    let (i, unit_fathoms) = char('F')(i)?;
    // todo->should we check for unit_fathoms?

    Ok(DbsData {
        water_depth_feet,
        water_depth_meters,
        water_depth_fathoms,
    })
}

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;

    use super::*;
    use crate::{parse_nmea_sentence, SentenceType};

    fn parse_dbs_with_nmea_sentence_struct() {
        let data = parse_dbs(NmeaSentence {
            talker_id: "SD",
            message_id: SentenceType::DBS,
            data: "7.8,f,2.4,M,1.3,F",
            checksum: 0x0,
        })
        .unwrap();
    }
}
