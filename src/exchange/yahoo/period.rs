use std::fmt;

/// An interval use when requesting periods of quote information.
/// ChartRange = start/end
/// a ChartRange of `5d` means data will range from **5 days ago** until **now**
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
pub enum ChartRange {
    _1d,
    _5d,
    _1mo,
    _3mo,
    _6mo,
    _1y,
    _2y,
    _5y,
    _10y,
    _ytd,
    _max,
}

impl ChartRange {
    pub fn allows_intraday(&self) -> bool {
        matches!(self, Self::_1d | Self::_5d | Self::_1mo)
    }
}

/// An interval use when requesting periods of quote information.
///
/// Since we cannot start the values with numbers (as they are normally represented),
/// we start them with underscores.
///
/// `m` is for minutes. `mo` is for months, the rest should be self explanatory
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
pub enum CandlestickInterval {
    _1m,
    _2m,
    _5m,
    _15m,
    _30m,
    _60m,
    _90m,
    _1d,
    _5d,
    _1mo,
    _3mo,
    _6mo,
    _1y,
    _2y,
    _5y,
    _10y,
    _ytd,
    _max,
}

impl CandlestickInterval {
    pub fn is_intraday(&self) -> bool {
        matches!(
            self,
            Self::_1m
                | Self::_2m
                | Self::_5m
                | Self::_15m
                | Self::_30m
                | Self::_60m
                | Self::_90m
        )
    }
}

impl fmt::Display for ChartRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("{:?}", self)[1..]) // strip the leading underscore
    }
}

impl fmt::Display for CandlestickInterval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("{:?}", self)[1..]) // strip the leading underscore
    }
}

#[cfg(test)]
mod tests {
    use super::{CandlestickInterval, ChartRange};

    // Validate that the CandlestickIntervals are all set up correctly
    // and that there are no copy-paste issues

    fn test_interval(
        interval: CandlestickInterval,
        value: &str,
        is_intraday: bool,
    ) {
        assert_eq!(format!("{}", interval), value);
        assert_eq!(interval.is_intraday(), is_intraday);
    }

    #[test]
    fn interval_1m() {
        test_interval(CandlestickInterval::_1m, "1m", true);
    }

    #[test]
    fn interval_2m() {
        test_interval(CandlestickInterval::_2m, "2m", true);
    }

    #[test]
    fn interval_5m() {
        test_interval(CandlestickInterval::_5m, "5m", true);
    }

    #[test]
    fn interval_15m() {
        test_interval(CandlestickInterval::_15m, "15m", true);
    }

    #[test]
    fn interval_30m() {
        test_interval(CandlestickInterval::_30m, "30m", true);
    }

    #[test]
    fn interval_60m() {
        test_interval(CandlestickInterval::_60m, "60m", true);
    }

    #[test]
    fn interval_90m() {
        test_interval(CandlestickInterval::_90m, "90m", true);
    }

    #[test]
    fn interval_1d() {
        test_interval(CandlestickInterval::_1d, "1d", false);
    }

    #[test]
    fn interval_5d() {
        test_interval(CandlestickInterval::_5d, "5d", false);
    }

    #[test]
    fn interval_1mo() {
        test_interval(CandlestickInterval::_1mo, "1mo", false);
    }

    #[test]
    fn interval_3mo() {
        test_interval(CandlestickInterval::_3mo, "3mo", false);
    }

    #[test]
    fn interval_6mo() {
        test_interval(CandlestickInterval::_6mo, "6mo", false);
    }

    #[test]
    fn interval_1y() {
        test_interval(CandlestickInterval::_1y, "1y", false);
    }

    #[test]
    fn interval_2y() {
        test_interval(CandlestickInterval::_2y, "2y", false);
    }

    #[test]
    fn interval_5y() {
        test_interval(CandlestickInterval::_5y, "5y", false);
    }

    #[test]
    fn interval_10y() {
        test_interval(CandlestickInterval::_10y, "10y", false);
    }

    #[test]
    fn interval_ytd() {
        test_interval(CandlestickInterval::_ytd, "ytd", false);
    }

    #[test]
    fn interval_max() {
        test_interval(CandlestickInterval::_max, "max", false);
    }

    fn test_range(interval: ChartRange, value: &str, allows_intraday: bool) {
        assert_eq!(format!("{}", interval), value);
        assert_eq!(interval.allows_intraday(), allows_intraday);
    }

    // _1d, _5d, _1mo, _3mo, _6mo, _1y, _2y, _5y, _10y, _ytd, _max
    #[test]
    fn range_1d() {
        test_range(ChartRange::_1d, "1d", true);
    }

    #[test]
    fn range_5d() {
        test_range(ChartRange::_5d, "5d", true);
    }

    #[test]
    fn range_1mo() {
        test_range(ChartRange::_1mo, "1mo", true);
    }

    #[test]
    fn range_3mo() {
        test_range(ChartRange::_3mo, "3mo", false);
    }
}
