// Utility macro for accessing pixels
macro_rules! pixel_pos {
    (H ; $w:expr, $h:expr, $l:expr ; $x:expr , $y:expr , $i:expr)
        => (((($x) + ($y) * ($w)) * ($l) + ($i)));

    (V ; $w:expr, $h:expr, $l:expr ; $x:expr , $y:expr , $i:expr)
        => (((($y) + ($x) * ($h)) * ($l) + ($i)));

    ($orientation:ident ; $w:expr, $h:expr, $l:expr ; $x:expr , $y:expr , $i1:expr => $i2:expr)
        => (pixel_pos!($orientation; $w, $h, $l; $x, $y, $i1) ..  pixel_pos!($orientation; $w, $h, $l; $x, $y, $i2));
}

macro_rules! pixel {
    ($name:ident [ $orientation:ident ; $w:expr, $h:expr, $l:expr ;
     $x:expr , $y:expr , $i:expr ])
        => ($name[pixel_pos!($orientation; $w, $h, $l; $x, $y, $i)]);

    ($name:ident [ $orientation:ident ; $w:expr, $h:expr, $l:expr ;
     $x:expr , $y:expr , $i1:expr => $i2:expr ])
        => ($name[pixel_pos!($orientation; $w, $h, $l; $x, $y, $i1 => $i2)]);


    ($name:ident [ $w:expr, $h:expr, $l:expr ; $x:expr , $y:expr , $i:expr ])
        => (pixel!($name[H; $w, $h, $l; $x, $y, $i]));

    ($name:ident [ $w:expr, $h:expr, $l:expr ; $x:expr , $y:expr , $i1:expr => $i2:expr ])
        => (pixel!($name[H; $w, $h, $l; $x, $y, $i1 => $i2]));


    ($name:ident [ H ; $w:expr, $h:expr ; $x:expr , $y:expr ])
        => (pixel!($name[H; $w, $h, 1; $x, $y, 0]));

    ($name:ident [ V ; $w:expr, $h:expr ; $x:expr , $y:expr ])
        => (pixel!($name[V; $w, $h, 1; $x, $y, 0]));

    ($name:ident [ $w:expr, $h:expr ; $x:expr , $y:expr ])
        => (pixel!($name[H; $w, $h; $x, $y]));
}
