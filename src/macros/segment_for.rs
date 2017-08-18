// Utility macro for generating segmented for-loops
macro_rules! three_segment_for {
    (for $idx:ident in $a:expr, $b:expr, $c:expr, $d:expr, {
        $ab:expr , $bc:expr , $cd:expr
    }) => {
        for $idx in $a..$b {
            $ab;
        }
        for $idx in $b..$c {
            $bc;
        }
        for $idx in $c..$d {
            $cd;
        }
    };
}
macro_rules! flipping_three_segment_for {
    (for $idx:ident in $a:expr, $b:expr, $c:expr, $d:expr, {
        $odd:expr , $even:expr
    }) => {
        three_segment_for!{
            for $idx in $a, $b, $c, $d, {
                $odd,
                $even,
                $odd
            }
        }
    }
}
