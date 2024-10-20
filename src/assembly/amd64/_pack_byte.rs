#[macro_export] macro_rules! pack_byte {
    ($b7:expr, $b6:expr, $b5:expr, $b4:expr, $b3:expr, $b2:expr, $b1:expr, $b0:expr) => {{
        (((($b7 as u8) & 1) << 7)
            | ((($b6 as u8) & 1) << 6)
            | ((($b5 as u8) & 1) << 5)
            | ((($b4 as u8) & 1) << 4)
            | ((($b3 as u8) & 1) << 3)
            | ((($b2 as u8) & 1) << 2)
            | ((($b1 as u8) & 1) << 1)
            | ((($b0 as u8) & 1) << 0)) as u8
    }};
}
