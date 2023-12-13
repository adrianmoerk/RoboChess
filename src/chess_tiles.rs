#![allow(dead_code)]
use super::robot::ChessTilePosition;
const CHESS_TILE_POS_A1: ChessTilePosition = ChessTilePosition {
    field_char: 'a',
    field_num: 1,
};
const CHESS_TILE_POS_A2: ChessTilePosition = ChessTilePosition {
    field_char: 'a',
    field_num: 2,
};
const CHESS_TILE_POS_A3: ChessTilePosition = ChessTilePosition {
    field_char: 'a',
    field_num: 3,
};
const CHESS_TILE_POS_A4: ChessTilePosition = ChessTilePosition {
    field_char: 'a',
    field_num: 4,
};
const CHESS_TILE_POS_A5: ChessTilePosition = ChessTilePosition {
    field_char: 'a',
    field_num: 5,
};
const CHESS_TILE_POS_A6: ChessTilePosition = ChessTilePosition {
    field_char: 'a',
    field_num: 6,
};
const CHESS_TILE_POS_A7: ChessTilePosition = ChessTilePosition {
    field_char: 'a',
    field_num: 7,
};
const CHESS_TILE_POS_A8: ChessTilePosition = ChessTilePosition {
    field_char: 'a',
    field_num: 8,
};
const CHESS_TILE_POS_B1: ChessTilePosition = ChessTilePosition {
    field_char: 'b',
    field_num: 1,
};
const CHESS_TILE_POS_B2: ChessTilePosition = ChessTilePosition {
    field_char: 'b',
    field_num: 2,
};
const CHESS_TILE_POS_B3: ChessTilePosition = ChessTilePosition {
    field_char: 'b',
    field_num: 3,
};
const CHESS_TILE_POS_B4: ChessTilePosition = ChessTilePosition {
    field_char: 'b',
    field_num: 4,
};
const CHESS_TILE_POS_B5: ChessTilePosition = ChessTilePosition {
    field_char: 'b',
    field_num: 5,
};
const CHESS_TILE_POS_B6: ChessTilePosition = ChessTilePosition {
    field_char: 'b',
    field_num: 6,
};
const CHESS_TILE_POS_B7: ChessTilePosition = ChessTilePosition {
    field_char: 'b',
    field_num: 7,
};
const CHESS_TILE_POS_B8: ChessTilePosition = ChessTilePosition {
    field_char: 'b',
    field_num: 8,
};
const CHESS_TILE_POS_C1: ChessTilePosition = ChessTilePosition {
    field_char: 'c',
    field_num: 1,
};
const CHESS_TILE_POS_C2: ChessTilePosition = ChessTilePosition {
    field_char: 'c',
    field_num: 2,
};
const CHESS_TILE_POS_C3: ChessTilePosition = ChessTilePosition {
    field_char: 'c',
    field_num: 3,
};
const CHESS_TILE_POS_C4: ChessTilePosition = ChessTilePosition {
    field_char: 'c',
    field_num: 4,
};
const CHESS_TILE_POS_C5: ChessTilePosition = ChessTilePosition {
    field_char: 'c',
    field_num: 5,
};
const CHESS_TILE_POS_C6: ChessTilePosition = ChessTilePosition {
    field_char: 'c',
    field_num: 6,
};
const CHESS_TILE_POS_C7: ChessTilePosition = ChessTilePosition {
    field_char: 'c',
    field_num: 7,
};
const CHESS_TILE_POS_C8: ChessTilePosition = ChessTilePosition {
    field_char: 'c',
    field_num: 8,
};
const CHESS_TILE_POS_D1: ChessTilePosition = ChessTilePosition {
    field_char: 'd',
    field_num: 1,
};
const CHESS_TILE_POS_D2: ChessTilePosition = ChessTilePosition {
    field_char: 'd',
    field_num: 2,
};
const CHESS_TILE_POS_D3: ChessTilePosition = ChessTilePosition {
    field_char: 'd',
    field_num: 3,
};
const CHESS_TILE_POS_D4: ChessTilePosition = ChessTilePosition {
    field_char: 'd',
    field_num: 4,
};
const CHESS_TILE_POS_D5: ChessTilePosition = ChessTilePosition {
    field_char: 'd',
    field_num: 5,
};
const CHESS_TILE_POS_D6: ChessTilePosition = ChessTilePosition {
    field_char: 'd',
    field_num: 6,
};
const CHESS_TILE_POS_D7: ChessTilePosition = ChessTilePosition {
    field_char: 'd',
    field_num: 7,
};
const CHESS_TILE_POS_D8: ChessTilePosition = ChessTilePosition {
    field_char: 'd',
    field_num: 8,
};
const CHESS_TILE_POS_E1: ChessTilePosition = ChessTilePosition {
    field_char: 'e',
    field_num: 1,
};
const CHESS_TILE_POS_E2: ChessTilePosition = ChessTilePosition {
    field_char: 'e',
    field_num: 2,
};
const CHESS_TILE_POS_E3: ChessTilePosition = ChessTilePosition {
    field_char: 'e',
    field_num: 3,
};
const CHESS_TILE_POS_E4: ChessTilePosition = ChessTilePosition {
    field_char: 'e',
    field_num: 4,
};
const CHESS_TILE_POS_E5: ChessTilePosition = ChessTilePosition {
    field_char: 'e',
    field_num: 5,
};
const CHESS_TILE_POS_E6: ChessTilePosition = ChessTilePosition {
    field_char: 'e',
    field_num: 6,
};
const CHESS_TILE_POS_E7: ChessTilePosition = ChessTilePosition {
    field_char: 'e',
    field_num: 7,
};
const CHESS_TILE_POS_E8: ChessTilePosition = ChessTilePosition {
    field_char: 'e',
    field_num: 8,
};
const CHESS_TILE_POS_F1: ChessTilePosition = ChessTilePosition {
    field_char: 'f',
    field_num: 1,
};
const CHESS_TILE_POS_F2: ChessTilePosition = ChessTilePosition {
    field_char: 'f',
    field_num: 2,
};
const CHESS_TILE_POS_F3: ChessTilePosition = ChessTilePosition {
    field_char: 'f',
    field_num: 3,
};
const CHESS_TILE_POS_F4: ChessTilePosition = ChessTilePosition {
    field_char: 'f',
    field_num: 4,
};
const CHESS_TILE_POS_F5: ChessTilePosition = ChessTilePosition {
    field_char: 'f',
    field_num: 5,
};
const CHESS_TILE_POS_F6: ChessTilePosition = ChessTilePosition {
    field_char: 'f',
    field_num: 6,
};
const CHESS_TILE_POS_F7: ChessTilePosition = ChessTilePosition {
    field_char: 'f',
    field_num: 7,
};
const CHESS_TILE_POS_F8: ChessTilePosition = ChessTilePosition {
    field_char: 'f',
    field_num: 8,
};
const CHESS_TILE_POS_G1: ChessTilePosition = ChessTilePosition {
    field_char: 'g',
    field_num: 1,
};
const CHESS_TILE_POS_G2: ChessTilePosition = ChessTilePosition {
    field_char: 'g',
    field_num: 2,
};
const CHESS_TILE_POS_G3: ChessTilePosition = ChessTilePosition {
    field_char: 'g',
    field_num: 3,
};
const CHESS_TILE_POS_G4: ChessTilePosition = ChessTilePosition {
    field_char: 'g',
    field_num: 4,
};
const CHESS_TILE_POS_G5: ChessTilePosition = ChessTilePosition {
    field_char: 'g',
    field_num: 5,
};
const CHESS_TILE_POS_G6: ChessTilePosition = ChessTilePosition {
    field_char: 'g',
    field_num: 6,
};
const CHESS_TILE_POS_G7: ChessTilePosition = ChessTilePosition {
    field_char: 'g',
    field_num: 7,
};
const CHESS_TILE_POS_G8: ChessTilePosition = ChessTilePosition {
    field_char: 'g',
    field_num: 8,
};
const CHESS_TILE_POS_H1: ChessTilePosition = ChessTilePosition {
    field_char: 'h',
    field_num: 1,
};
const CHESS_TILE_POS_H2: ChessTilePosition = ChessTilePosition {
    field_char: 'h',
    field_num: 2,
};
const CHESS_TILE_POS_H3: ChessTilePosition = ChessTilePosition {
    field_char: 'h',
    field_num: 3,
};
const CHESS_TILE_POS_H4: ChessTilePosition = ChessTilePosition {
    field_char: 'h',
    field_num: 4,
};
const CHESS_TILE_POS_H5: ChessTilePosition = ChessTilePosition {
    field_char: 'h',
    field_num: 5,
};
const CHESS_TILE_POS_H6: ChessTilePosition = ChessTilePosition {
    field_char: 'h',
    field_num: 6,
};
const CHESS_TILE_POS_H7: ChessTilePosition = ChessTilePosition {
    field_char: 'h',
    field_num: 7,
};
const CHESS_TILE_POS_H8: ChessTilePosition = ChessTilePosition {
    field_char: 'h',
    field_num: 8,
};
