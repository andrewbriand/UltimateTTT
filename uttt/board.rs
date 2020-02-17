enum Player {
    X,
    O,
}

struct Board {
    x_spaces: Vec<bool>,
    o_spaces: Vec<bool>,
    to_move: Player,

}
