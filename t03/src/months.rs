use anyhow::anyhow;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Copy)]
pub(crate) enum Months {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl TryFrom<&str> for Months {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value.trim().to_lowercase().as_str() {
            "january" | "jan" => Ok(Months::January),
            "february" | "feb" => Ok(Months::February),
            "march" | "mar" => Ok(Months::March),
            "april" | "apr" => Ok(Months::April),
            "may" => Ok(Months::May),
            "june" | "jun" => Ok(Months::June),
            "july" | "jul" => Ok(Months::July),
            "august" | "aug" => Ok(Months::August),
            "september" | "sep" => Ok(Months::September),
            "october" | "oct" => Ok(Months::October),
            "november" | "nov" => Ok(Months::November),
            "december" | "dec" => Ok(Months::December),
            _ => Err(anyhow!("invalid value for month: {}", value)),
        }
    }
}
