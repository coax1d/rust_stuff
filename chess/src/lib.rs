mod piece;
use piece::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
        let new_piece = Piece::new(PieceType::King, true);
        let output = new_piece.move_piece(1).unwrap();
        println!("{}", output);
    }
}
