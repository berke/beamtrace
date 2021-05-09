use std::collections::HashMap;

//         +---------+ --/-----/-
//         |         |         |
//         |         |         |
//         |         |         |
//         |         |         |
//         |         |         | H
// (x0,y0) +---------+ --/--   |
//         |         |   |     |
//         |         |   | D   |
//         |         |   |     |
//         +---------+ --/-----/-
//
//         |         |
//         |         |
//         /---------/
//         |    W    |

pub const W : f64 = 5.0; // 7.0
pub const D : f64 = 4.0;
pub const H : f64 = 11.0;

pub type Glyph = Vec<(bool,i8,i8)>;

pub struct Font {
    pub glyphs:HashMap<char,Glyph>
}

pub const FONT0 : &[&[(bool,i8,i8)]] = &[
    // 0x00
    &[],
    // 0x01
    &[],
    // 0x02
    &[],
    // 0x03
    &[],
    // 0x04
    &[],
    // 0x05
    &[],
    // 0x06
    &[],
    // 0x07
    &[],
    // 0x08
    &[],
    // 0x09
    &[],
    // 0x0a
    &[],
    // 0x0b
    &[],
    // 0x0c
    &[],
    // 0x0d
    &[],
    // 0x0e
    &[],
    // 0x0f
    &[],
    // 0x10
    &[],
    // 0x11
    &[],
    // 0x12
    &[],
    // 0x13
    &[],
    // 0x14
    &[],
    // 0x15
    &[],
    // 0x16
    &[],
    // 0x17
    &[],
    // 0x18
    &[],
    // 0x19
    &[],
    // 0x1a
    &[],
    // 0x1b
    &[],
    // 0x1c
    &[],
    // 0x1d
    &[],
    // 0x1e
    &[],
    // 0x1f
    &[],
    // 0x20
    &[],
    // 0x21 '!'
    &[(false,3,4),(true,3,5),(false,3,6),(true,3,10)],
    // 0x22 '"'
    &[(false,2,9),(true,2,10),(false,4,9),(true,4,10)],
    // 0x23 '#'
    &[(false,2,4),(true,2,10),(false,4,4),(true,4,10),(false,1,6),(true,5,6),(false,1,8),(true,5,8)],
    // 0x24 '$'
    &[(false,3,4),(true,3,10),(false,5,9),(true,2,9),(true,1,8),(true,2,7),(true,4,7),(true,5,6),(true,4,5),(true,1,5)],
    // 0x25 '%'
    &[(false,1,4),(true,5,10),(false,2,9),(true,2,8),(true,3,8),(true,3,9),(true,2,9),(false,3,6),(true,3,5),(true,4,5),(true,4,6),(true,3,6)],
    // 0x26 '&'
    &[(false,5,4),(true,1,8),(true,1,9),(true,2,10),(true,3,9),(true,3,8),(true,1,6),(true,1,5),(true,2,4),(true,3,4),(true,5,6)],
    // 0x27 '''
    &[(false,3,9),(true,4,10)],
    // 0x28 '('
    &[(false,5,10),(true,3,8),(true,3,6),(true,5,4)],
    // 0x29 ')'
    &[(false,1,4),(true,3,6),(true,3,8),(true,1,10)],
    // 0x2a '*'
    &[(false,0,5),(true,6,9),(false,6,5),(true,0,9),(false,3,10),(true,3,4)],
    // 0x2b '+'
    &[(false,3,5),(true,3,9),(false,1,7),(true,5,7)],
    // 0x2c ','
    &[(false,3,4),(true,2,4),(true,2,5),(true,3,5),(true,3,3),(true,2,2)],
    // 0x2d '-'
    &[(false,1,7),(true,5,7)],
    // 0x2e '.'
    &[(false,3,4),(true,2,4),(true,2,5),(true,3,5),(true,3,4)],
    // 0x2f '/'
    &[(true,6,10)],
    // 0x30 '0'
    &[(false,1,5),(true,2,4),(true,4,4),(true,5,5),(true,5,9),(true,4,10),(true,2,10),(true,1,9),(true,1,5),(true,5,9)],
    // 0x31 '1'
    &[(false,2,4),(true,4,4),(false,3,4),(true,3,10),(true,2,9)],
    // 0x32 '2'
    &[(false,1,9),(true,2,10),(true,4,10),(true,5,9),(true,5,8),(true,1,5),(true,1,4),(true,5,4)],
    // 0x33 '3'
    &[(false,1,5),(true,2,4),(true,4,4),(true,5,5),(true,5,6),(true,4,7),(true,3,7),(true,5,10),(true,1,10)],
    // 0x34 '4'
    &[(false,4,10),(true,1,7),(true,1,6),(true,5,6),(false,4,7),(true,4,4)],
    // 0x35 '5'
    &[(false,5,10),(true,1,10),(true,1,8),(true,4,8),(true,5,7),(true,5,5),(true,4,4),(true,2,4),(true,1,5)],
    // 0x36 '6'
    &[(false,5,10),(true,3,10),(true,1,8),(true,1,5),(true,2,4),(true,4,4),(true,5,5),(true,5,6),(true,4,7),(true,1,7)],
    // 0x37 '7'
    &[(false,1,10),(true,5,10),(true,5,9),(true,1,6),(true,1,4)],
    // 0x38 '8'
    &[(false,4,7),(true,5,8),(true,5,9),(true,4,10),(true,2,10),(true,1,9),(true,1,8),(true,2,7),(true,4,7),(true,5,6),(true,5,5),(true,4,4),(true,2,4),(true,1,5),(true,1,6),(true,2,7)],
    // 0x39 '9'
    &[(false,2,4),(true,3,4),(true,5,6),(true,5,9),(true,4,10),(true,2,10),(true,1,9),(true,1,8),(true,2,7),(true,5,7)],
    // 0x3a ':'
    &[(false,2,7),(true,2,8),(true,3,8),(true,3,7),(true,2,7),(false,2,5),(true,3,5),(true,3,4),(true,2,4),(true,2,5)],
    // 0x3b ';'
    &[(false,2,6),(true,2,7),(true,3,7),(true,3,6),(true,2,6),(false,3,4),(true,2,4),(true,2,5),(true,3,5),(true,3,3),(true,2,2)],
    // 0x3c '<'
    &[(false,4,10),(true,1,7),(true,4,4)],
    // 0x3d '='
    &[(false,1,8),(true,5,8),(false,1,6),(true,5,6)],
    // 0x3e '>'
    &[(false,1,10),(true,4,7),(true,1,4)],
    // 0x3f '?'
    &[(false,1,9),(true,2,10),(true,4,10),(true,5,9),(true,5,8),(true,4,7),(true,3,7),(true,3,6),(false,3,5),(true,3,4)],
    // 0x40 '@'
    &[(false,4,3),(true,2,3),(true,1,4),(true,1,8),(true,2,10),(true,4,10),(true,5,9),(true,5,6),(true,4,5),(true,3,6),(true,3,7),(true,4,8),(true,5,8)],
    // 0x41 'A'
    &[(false,1,4),(true,1,9),(true,2,10),(true,4,10),(true,5,9),(true,5,4),(false,1,6),(true,5,6)],
    // 0x42 'B'
    &[(false,1,4),(true,1,10),(true,4,10),(true,5,9),(true,5,8),(true,4,7),(true,1,7),(false,1,4),(true,4,4),(true,5,5),(true,5,6),(true,4,7)],
    // 0x43 'C'
    &[(false,5,5),(true,4,4),(true,2,4),(true,1,5),(true,1,9),(true,2,10),(true,4,10),(true,5,9)],
    // 0x44 'D'
    &[(false,1,4),(true,1,10),(true,4,10),(true,5,9),(true,5,5),(true,4,4),(true,1,4)],
    // 0x45 'E'
    &[(false,5,4),(true,1,4),(true,1,10),(true,5,10),(false,1,7),(true,4,7)],
    // 0x46 'F'
    &[(false,1,4),(true,1,10),(true,5,10),(false,1,7),(true,4,7)],
    // 0x47 'G'
    &[(false,5,9),(true,4,10),(true,2,10),(true,1,9),(true,1,5),(true,2,4),(true,4,4),(true,5,5),(true,5,7),(true,2,7)],
    // 0x48 'H'
    &[(false,1,4),(true,1,10),(false,5,4),(true,5,10),(false,1,7),(true,5,7)],
    // 0x49 'I'
    &[(false,1,4),(true,5,4),(false,3,4),(true,3,10),(false,1,10),(true,5,10)],
    // 0x4a 'J'
    &[(false,1,5),(true,2,4),(true,4,4),(true,5,5),(true,5,10),(true,1,10)],
    // 0x4b 'K'
    &[(false,1,4),(true,1,10),(false,1,7),(true,2,7),(true,5,4),(false,2,7),(true,5,10)],
    // 0x4c 'L'
    &[(false,1,10),(true,1,4),(true,5,4)],
    // 0x4d 'M'
    &[(false,1,4),(true,1,10),(true,3,8),(true,5,10),(true,5,4)],
    // 0x4e 'N'
    &[(false,1,4),(true,1,10),(true,5,4),(true,5,10)],
    // 0x4f 'O'
    &[(false,2,4),(true,1,5),(true,1,9),(true,2,10),(true,4,10),(true,5,9),(true,5,5),(true,4,4),(true,2,4)],
    // 0x50 'P'
    &[(false,1,4),(true,1,10),(true,4,10),(true,5,9),(true,5,8),(true,4,7),(true,1,7)],
    // 0x51 'Q'
    &[(false,2,4),(true,1,5),(true,1,9),(true,2,10),(true,4,10),(true,5,9),(true,5,6),(true,3,4),(true,2,4),(false,3,6),(true,5,4)],
    // 0x52 'R'
    &[(false,1,4),(true,1,10),(true,4,10),(true,5,9),(true,5,8),(true,4,7),(true,1,7),(true,2,7),(true,5,4)],
    // 0x53 'S'
    &[(false,1,5),(true,2,4),(true,4,4),(true,5,5),(true,5,6),(true,4,7),(true,2,7),(true,1,8),(true,1,9),(true,2,10),(true,4,10),(true,5,9)],
    // 0x54 'T'
    &[(false,3,4),(true,3,10),(true,1,10),(true,5,10)],
    // 0x55 'U'
    &[(false,1,10),(true,1,5),(true,2,4),(true,4,4),(true,5,5),(true,5,10)],
    // 0x56 'V'
    &[(false,1,10),(true,1,8),(true,3,4),(true,5,8),(true,5,10)],
    // 0x57 'W'
    &[(false,1,10),(true,1,4),(true,3,7),(true,5,4),(true,5,10)],
    // 0x58 'X'
    &[(false,1,4),(true,5,10),(false,5,4),(true,1,10)],
    // 0x59 'Y'
    &[(false,1,10),(true,1,9),(true,3,6),(true,3,4),(false,3,6),(true,5,9),(true,5,10)],
    // 0x5a 'Z'
    &[(false,1,10),(true,5,10),(true,1,4),(true,5,4)],
    // 0x5b '['
    &[(false,5,4),(true,3,4),(true,3,10),(true,5,10)],
    // 0x5c '\'
    &[(false,1,10),(true,5,4)],
    // 0x5d ']'
    &[(false,1,4),(true,3,4),(true,3,10),(true,1,10)],
    // 0x5e '^'
    &[(false,1,8),(true,3,10),(true,5,8)],
    // 0x5f '_'
    &[(false,1,3),(true,5,3)],
    // 0x60 '`'
    &[(false,2,11),(true,4,8)],
    // 0x61 'a'
    &[(false,5,4),(true,2,4),(true,1,5),(true,1,7),(true,2,8),(true,4,8),(true,4,4)],
    // 0x62 'b'
    &[(false,1,4),(true,4,4),(true,5,5),(true,5,7),(true,4,8),(true,2,8),(false,2,10),(true,2,4)],
    // 0x63 'c'
    &[(false,5,5),(true,4,4),(true,3,4),(true,2,5),(true,2,7),(true,3,8),(true,4,8),(true,5,7)],
    // 0x64 'd'
    &[(false,4,10),(true,4,4),(true,2,4),(true,1,5),(true,1,7),(true,2,8),(true,4,8),(false,4,4),(true,5,4)],
    // 0x65 'e'
    &[(false,1,6),(true,4,6),(true,5,7),(true,4,8),(true,2,8),(true,1,7),(true,1,5),(true,2,4),(true,5,4)],
    // 0x66 'f'
    &[(false,3,4),(true,3,9),(true,4,10),(true,5,10),(false,2,7),(true,4,7)],
    // 0x67 'g'
    &[(false,1,2),(true,3,2),(true,4,3),(true,4,8),(true,2,8),(true,1,7),(true,1,5),(true,2,4),(true,4,4)],
    // 0x68 'h'
    &[(false,1,10),(true,1,4),(false,1,8),(true,3,8),(true,4,7),(true,4,4)],
    // 0x69 'i'
    &[(false,3,10),(true,3,9),(false,2,8),(true,3,8),(true,3,4),(false,2,4),(true,4,4)],
    // 0x6a 'j'
    &[(false,3,10),(true,3,9),(false,2,8),(true,3,8),(true,3,3),(true,2,2),(true,1,2)],
    // 0x6b 'k'
    &[(false,1,4),(true,1,10),(false,4,4),(true,1,6),(true,4,8)],
    // 0x6c 'l'
    &[(false,2,10),(true,3,10),(true,3,4),(false,2,4),(true,4,4)],
    // 0x6d 'm'
    &[(false,1,4),(true,1,8),(false,1,7),(true,2,8),(true,3,7),(true,3,4),(false,3,7),(true,4,8),(true,5,7),(true,5,4)],
    // 0x6e 'n'
    &[(false,1,4),(true,1,8),(false,1,7),(true,2,8),(true,3,8),(true,4,7),(true,4,4)],
    // 0x6f 'o'
    &[(false,2,4),(true,1,5),(true,1,7),(true,2,8),(true,3,8),(true,4,7),(true,4,5),(true,3,4),(true,2,4)],
    // 0x70 'p'
    &[(false,1,2),(true,1,8),(true,3,8),(true,4,7),(true,4,5),(true,3,4),(true,1,4)],
    // 0x71 'q'
    &[(false,4,4),(true,2,4),(true,1,5),(true,1,7),(true,2,8),(true,4,8),(true,4,2)],
    // 0x72 'r'
    &[(false,1,8),(true,1,4),(false,1,6),(true,3,8),(true,4,8)],
    // 0x73 's'
    &[(false,4,8),(true,2,8),(true,1,7),(true,2,6),(true,3,6),(true,4,5),(true,3,4),(true,1,4)],
    // 0x74 't'
    &[(false,2,10),(true,2,4),(true,4,4),(false,1,8),(true,4,8)],
    // 0x75 'u'
    &[(false,1,8),(true,1,5),(true,2,4),(true,4,4),(true,4,8)],
    // 0x76 'v'
    &[(false,1,8),(true,1,6),(true,3,4),(true,5,6),(true,5,8)],
    // 0x77 'w'
    &[(false,1,8),(true,1,5),(true,2,4),(true,3,5),(true,3,7),(false,3,5),(true,4,4),(true,5,5),(true,5,8)],
    // 0x78 'x'
    &[(false,1,8),(true,5,4),(false,1,4),(true,5,8)],
    // 0x79 'y'
    &[(false,1,2),(true,5,6),(true,5,8),(false,1,8),(true,1,6),(true,3,4)],
    // 0x7a 'z'
    &[(false,1,8),(true,4,8),(true,1,4),(true,4,4)],
    // 0x7b '{'
    &[(false,4,11),(true,3,11),(true,2,10),(true,2,8),(true,1,7),(true,2,6),(true,2,4),(true,3,3),(true,4,3)],
    // 0x7c '|'
    &[(false,3,11),(true,3,3)],
    // 0x7d '}'
    &[(false,2,11),(true,3,11),(true,4,10),(true,4,8),(true,5,7),(true,4,6),(true,4,4),(true,3,3),(true,2,3)],
    // 0x7e '~'
    &[(false,1,9),(true,2,10),(true,4,8),(true,5,9)],
    // 0x7f
    &[],
];

const FONT173 : &[(char,&[u8])] = &[
    // ('x',b"064265066272"), // exclam 
    // ('x',b"031264331047307"), // forall 
    // ('x',b"044252104312026326030330"), // hash 
    // ('x',b"031271265225067227"), // exists 
    // ('x',b"024332051250270271251066265305306266"), // percent 
    // ('x',b"124230231252271270226225244264326"), // ampersand 
    // ('x',b"031251270267266225067227"), // ni 
    // ('x',b"132270266324"), // opening brace 
    // ('x',b"024266270232"), // closing brace 
    // ('x',b"005351145211072264"), // asterisk 
    // ('x',b"065271027327"), // plus 
    // ('x',b"064244245265263242"), // comma 
    // ('x',b"027327"), // minus 
    // ('x',b"064244245265264"), // dot 
    // ('x',b"352"), // slash 

    // ('x',b"025325027327030251270310331"), // congruent 
    // ('x',b"024231252312331324026326"), // A 
    // ('x',b"024232312331330307227024304325326307"), // B
    // ('x',b"024332124232"), // Chi (X) 
    // ('x',b"024272324224"), // Delta 
    // ('x',b"124224232332027307"), // E
    // ('x',b"052312072264044304027251311327305245227"), // Phi 
    // ('x',b"052332330072264044304"), // Gamma 
    // ('x',b"024232124332027327"), // H
    // ('x',b"024324064272032332"), // I
    // ('x',b"027304330331271270326"), // vartheta 
    // ('x',b"024232027247324047332"), // K
    // ('x',b"024272324"), // Lambda 
    // ('x',b"024232270332324"), // M
    // ('x',b"024232324332"), // N
    // ('x',b"044225231252312331325304244"), // O

    // ('x',b"044252032332112304"), // Pi 
    // ('x',b"044225231252312331325304244027327"), // Theta 
    // ('x',b"024232312331330307227"), // Rho (P) 
    // ('x',b"044252032332112304"), // Sigma 
    // ('x',b"064272232332"), // T
    // ('x',b"032231266264066331332"), // Y
    // ('x',b"042262303304225226247307"), // varsigma 
    // ('x',b"024244226231252312331326304324"), // Omega 
    // ('x',b"031232332331050246047307110306025224324325"), // Xi 
    // ('x',b"030250246306310330052312072264044304"), // Psi 
    // ('x',b"032332224324"), // Z
    // ('x',b"124264272332"), // opening bracket 
    // ('x',b"004224225205204104324325305304051271272252251"), // point triangle 
    // ('x',b"024264272232"), // closing bracket 
    // ('x',b"024324064270"), // bottom 
    // ('x',b"023323"), // underline 

    // ('x',b"034334"), // overline 

    ('α',b"025227250270307305264244225107330105324"), // alpha 
    ('β',b"044251272311310267306304264245"), // beta 
    ('χ',b"027250267265304325024330"), // chi 
    ('δ',b"045246267307326325304264245107271312331"), // delta 
    ('ε',b"127310250227246266046225244304325"), // epsilon 
    ('ϕ',b"026247307326305245226064270"), // phi 
    ('γ',b"027226303262243326327"), // gamma 
    ('η',b"030247270310327322047245"), // eta 
    ('ι',b"067264304305"), // iota 
    ('φ',b"030227246306327310270264244"), // varphi 
    ('κ',b"050244046266330066305304324"), // kappa 
    ('λ',b"031251250324024266"), // lambda 
    ('μ',b"024250246265306310106325"), // mu 
    ('ν',b"030226264326327310"), // nu 
    // ('x',b"044225227250270307305264244"), // o 

    ('π',b"044247104307027250267307330"), // pi 
    ('θ',b"045264305311272251245047307"), // theta 
    ('ρ',b"023246267307326325304264245"), // rho 
    ('σ',b"045246267307326325304264245107330"), // sigma 
    ('τ',b"026267327067264324"), // tau 
    // ('x',b"030226264326330"), // v 
    ('ϖ',b"050227225244265266065304325327310047307"), // var pi 
    ('ω',b"050227225244265266065304325327310"), // omega 
    ('ξ',b"051250267307330270246265305326266244243262322321301"), // xi 
    ('ψ',b"027247265305327110264"), // psi 
    ('ζ',b"051250267307330270246244243262322321301"), // zeta 
    // ('x',b"113273252250227246244263303"), // opening curly brace 
    // ('x',b"073263"), // vertical bar 
    // ('x',b"053273312310327306304263243"), // closing curly brace 
    // ('x',b"027250267307330"), // similar 
    // ('x',b""), // blank 

    // ('x',b""), // space 
    // ('x',b"030251270311330070264044304"), // Upsilon 
    // ('x',b"071312"), // prime 
    // ('x',b"025325227331"), // lessorequal 
    // ('x',b"352"), // slash 
    // ('x',b"02622624724726626624524522666266307326305266"), // infinity 
    // ('x',b"131332312270265244224225050310"), // math function 
    // ('x',b"026247266245226066307326305266067250271310267067264"), // clubsuit 
    // ('x',b"026271326263226"), // diamondsuit 
    // ('x',b"030252271312330265230"), // heartsuit 
    // ('x',b"026271326305267245226067264"), // spadesuit 
    // ('x',b"027327106327310046227250"), // leftrightarrow 
    // ('x',b"027327046227250"), // leftarrow 
    // ('x',b"072264030272330"), // uparrow 
    // ('x',b"027327106327310"), // rightarrow 
    // ('x',b"072264026264326"), // downarrow 

    // ('x',b"050251272311310267250"), // degree 
    // ('x',b"065271027327025325"), // plusminus 
    // ('x',b"051252111312"), // two primes 
    // ('x',b"125225327231"), // greaterorequal 
    // ('x',b"045310050305"), // times 
    // ('x',b"02622624724726626624524522666266307066305"), // propto 
    // ('x',b"050271311305264244225226267306"), // partial 
    // ('x',b"047270307266247"), // circle 
    // ('x',b"027327065266071272"), // divide 
    // ('x',b"030330026326045311"), // notequal 
    // ('x',b"025325027327031331"), // identity 
    // ('x',b"026247266306327030251270310331"), // approx 
    // ('x',b"024225064265124325"), // ellipsis 
    // ('x',b"072264"), // center part of braces and brackets 
    // ('x',b"027327"), // horiz line 
    // ('x',b"027327046227250127330"), // leftanglearrow 

    // ('x',b"024245247130307305030324"), // aleph 
    // ('x',b"050231252272330132310305264244225"), // Im 
    // ('x',b"050231252312330307305324107267072265244225"), // Re 
    // ('x',b"030227245243222224246247270310327325304264245"), // wp 
    // ('x',b"027230251311330326305245226227045310050305"), // otimes 
    // ('x',b"027230251311330326305245226227071265027327"), // oplus 
    // ('x',b"027251311327305245227024332"), // oslash 
    // ('x',b"025230251311330325"), // cap 
    // ('x',b"031226245305326331"), // cup 
    // ('x',b"025305326330311231"), // superset 
    // ('x',b"026306327310230024324"), // supsetequal 
    // ('x',b"125245226230251331045311"), // not subset 
    // ('x',b"125245226230251331"), // subset 
    // ('x',b"126246227250330024324"), // subsetequal 
    // ('x',b"125245226230251331027327"), // in 
    // ('x',b"125245226230251331027327045311"), // not in 

    // ('x',b"132224324"), // angle 
    // ('x',b"032332264232"), // nabla 
    // ('x',b"027230251311330326305245226227046250270267247266"), // registered 
    // ('x',b"027230251311330326305245226227110270247266306"), // copyright 
    // ('X',b"032272052250070272311332330"), // TM 
    // ('x',b"024231124331031331"), // prod 
    // ('x',b"026246264311331"), // root 
    // ('x',b"066267"), // centered dot 
    // ('x',b"026326325"), // neg 
    // ('x',b"024267324"), // wedge 
    // ('x',b"030264330"), // vee 
    // ('x',b"045305047307070226264070326264"), // leftright doublearrow 
    // ('x',b"045325047327070226264"), // left doublearrow 
    // ('x',b"044250104310027272327"), // up doublearrow 
    // ('x',b"025305027307070326264"), // right doublearrow 
    // ('x',b"052245112305026264326"), // down doublearrow 

    // ('x',b"026270326264226"), // diamond 
    // ('x',b"112247304"), // opening angle bracket 
    // ('x',b"027230251311330326305245226227046250270267247266"), // registered 
    // ('x',b"027230251311330326305245226227110270247266306"), // copyright 
    // ('X',b"032272052250070272311332330"), // TM 
    // ('x',b"044252032332112304"), // Sigma 
    // ('x',b"112270264"), // top third of opening brace 
    // ('x',b"072264"), // center part of braces and brackets 
    // ('x',b"072266304"), // lower third of opening brace 
    // ('x',b"112272264"), // upper third of opening bracket 
    // ('x',b"072264"), // center part of braces and brackets 
    // ('x',b"072264304"), // lower third of opening bracket 
    // ('x',b"112270264"), // top third of opening brace 
    // ('x',b"072270247266264"), // center part of opening curly brace 
    // ('x',b"072266324"), // lower third of opening round brace 
    // ('x',b"072264"), // center part of braces and brackets 

    // ('x',b""), // empty 
    // ('x',b"052307244"), // closing angle bracket 
    // ('x',b"024243264272313332"), // integral 
    // ('x',b"112270264"), // top third of opening brace 
    // ('x',b"072264"), // center part of braces 
    // ('x',b"072266224"), // lower third of closing brace 
    // ('x',b"052270264"), // top third of closing brace 
    // ('x',b"072264"), // center part of braces 
    // ('x',b"072266224"), // lower third of closing brace 
    // ('x',b"052272264"), // upper third of closing bracket 
    // ('x',b"072264"), // center part of braces 
    // ('x',b"072264244"), // lower third of closing bracket 
    // ('x',b"052270264"), // top third of closing brace 
    // ('x',b"072270307266264"), // center part of closing curly brace 
    // ('x',b"072266224"), // lower third of closing brace 
];

fn decode(u:&[u8])->Vec<(bool,i8,i8)> {
    let m = u.len() / 3;
    let mut w = Vec::with_capacity(m);
    for v in u.chunks(3) {
	let b = u8::from_str_radix(std::str::from_utf8(v).unwrap(),8).unwrap();
	let pen = b >= 128;
	let x = ((b >> 4) & 7) as i8;
	let y = (b & 15) as i8;
	w.push((pen,x,y));
    }
    w
}

impl Font {
    pub fn new()->Self {
	let mut this = Self{ glyphs:HashMap::new() };
	this.add_ascii();
	this.add_math();
	this
    }

    pub fn add(&mut self,c:char,g:Glyph) {
	self.glyphs.insert(c,g);
    }

    fn add_ascii(&mut self) {
	for k in 0..128 {
	    self.add(k.into(),Vec::from(FONT0[k as usize]));
	}
    }

    fn add_math(&mut self) {
	self.add_table(FONT173);
    }

    fn add_table(&mut self,tbl:&[(char,&[u8])]) {
	for &(c,h) in tbl.iter() {
	    self.add(c,decode(h));
	}
    }

    pub fn get(&self,c:char)->Option<&Glyph> {
	self.glyphs.get(&c)
    }
}

