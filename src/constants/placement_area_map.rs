use crate::components::TileType;


pub const PLACEMENT_AREA_MAP: &[&[TileType]] = &[
    // Top 3 rows (13 tiles wide)
    &[
        TileType::MinusOnePoint, TileType::MinusOnePoint, TileType::MinusOnePoint, TileType::MinusOnePoint, TileType::MinusOnePoint, 
        TileType::MinusOnePoint, TileType::MinusOnePoint, TileType::MinusOnePoint, TileType::MinusOnePoint, TileType::MinusOnePoint, 
        TileType::MinusOnePoint, TileType::IncomeDiagonal, TileType::Empty,
    ],
    &[
        TileType::MinusOnePoint, TileType::MinusOnePoint, TileType::MinusOnePoint, TileType::MinusOnePoint, TileType::MinusOnePoint, 
        TileType::MinusOnePoint, TileType::MinusOnePoint, TileType::MinusOnePoint, TileType::MinusOnePoint, TileType::MinusOnePoint, 
        TileType::IncomeDiagonal, TileType::MinusOnePoint, TileType::Empty,
    ],
    &[
        TileType::MinusOnePoint, TileType::MinusOnePoint, TileType::MinusOnePoint, TileType::MinusOnePoint, TileType::MinusOnePoint, 
        TileType::MinusOnePoint, TileType::MinusOnePoint, TileType::MinusOnePoint, TileType::MinusOnePoint, TileType::IncomeDiagonal, 
        TileType::MinusOnePoint, TileType::MinusOnePoint, TileType::Empty,
    ],

    // Next 8 rows (12 tiles wide)
    &[
        TileType::MinusOnePoint, TileType::MinusOnePoint, TileType::MinusOnePoint, TileType::MinusOnePoint, TileType::MinusOnePoint, 
        TileType::MinusOnePoint, TileType::MinusOnePoint, TileType::MinusOnePoint, TileType::IncomeDiagonal, TileType::MinusOnePoint, 
        TileType::MinusOnePoint, TileType::MinusOnePoint,
    ],
    &[
        TileType::MinusOnePoint, TileType::MinusOnePoint, TileType::MinusOnePoint, TileType::MinusOnePoint, TileType::MinusOnePoint, 
        TileType::MinusOnePoint, TileType::MinusOnePoint, TileType::IncomeDiagonal, TileType::MinusOnePoint, TileType::MinusOnePoint, 
        TileType::MinusOnePoint, TileType::MinusOnePoint,
    ],
    &[
        TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, 
        TileType::Empty, TileType::Empty, TileType::IncomeDiagonal, TileType::MinusOnePoint, TileType::MinusOnePoint, 
        TileType::MinusOnePoint, TileType::MinusOnePoint, TileType::MinusOnePoint,
    ],
    &[
        TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, 
        TileType::Empty, TileType::IncomeDiagonal, TileType::Empty, TileType::MinusOnePoint, TileType::MinusOnePoint, 
        TileType::MinusOnePoint, TileType::MinusOnePoint, TileType::MinusOnePoint,
    ],
    &[
        TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, 
        TileType::IncomeDiagonal, TileType::Empty, TileType::Empty, TileType::MinusOnePoint, TileType::MinusOnePoint, 
        TileType::MinusOnePoint, TileType::MinusOnePoint, TileType::MinusOnePoint,
    ],
    &[
        TileType::Empty, TileType::Empty, TileType::Empty, TileType::IncomeDiagonal, 
        TileType::Empty, TileType::Empty, TileType::Empty, TileType::MinusOnePoint, TileType::MinusOnePoint, 
        TileType::MinusOnePoint, TileType::MinusOnePoint, TileType::MinusOnePoint,
    ],
    &[
        TileType::Empty, TileType::Empty, TileType::IncomeDiagonal, TileType::Empty, 
        TileType::Empty, TileType::Empty, TileType::Empty, TileType::MinusOnePoint, 
        TileType::MinusOnePoint, TileType::MinusOnePoint, TileType::MinusOnePoint, TileType::MinusOnePoint,
    ],
    &[
        TileType::Empty, TileType::IncomeDiagonal, TileType::Empty, TileType::Empty, 
        TileType::Empty, TileType::Bonus, TileType::Empty, TileType::MinusOnePoint, 
        TileType::MinusOnePoint, TileType::MinusOnePoint, TileType::MinusOnePoint, TileType::MinusOnePoint,
    ],

    // Bottom row (8 tiles wide)
    &[
        TileType::IncomeDiagonal, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, 
        TileType::Empty, TileType::Empty, TileType::MinusOnePoint
    ],
];
