mod charset0;
mod text;

use beamtrace::{point,Point,Book,Page,Plot,Command};
use text::Text;
use charset0::Font;

fn main() {
    let mut font = Font::new();
    font.add_ascii();
    font.add_math();
    let mut bk = Book::new();
    let mut pg = Page::new();
    let mut pl = Plot::new();
    let p0 = point(0.0,0.0);
    let r = text::text(&font,
			&mut pl,p0,1.0,
			&Text::parse("e^{2πft + j(x^2+y^2)} + x^{2 + αy^{3 + z^{x + 33}}} - 5").unwrap());
    pl.rgb12(0xff0);
    text::rect(&mut pl,r);
    pg.plot(pl);
    bk.page(pg);
    bk.save_to_file("traj.mpk").unwrap();
    println!("BK:\n{:?}",bk);
}
