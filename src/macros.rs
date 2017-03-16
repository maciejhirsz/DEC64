/// Shorthand for constructing Dec64 from raw value.
macro_rules! dec64_raw {
    ( $value:expr ) => (
		Dec64 { value: $value }
	)
}

/// Shorthand for constructing Dec64 from raw parts.
macro_rules! dec64_parts {
    ( $coefficient:expr, $exponent:expr ) => (
        // Double casting on exponent so we don't end up with bunch
        // of `1` bits on the left if the exponent is negative
		dec64_raw!(($coefficient << 8) | ($exponent as u8 as i64))
	)
}
