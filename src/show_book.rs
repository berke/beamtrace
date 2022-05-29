use beambook::Book;

fn main() {
    let bk = Book::load_from_file("traj.mpk").unwrap();
    println!("{:#?}",bk);
}
