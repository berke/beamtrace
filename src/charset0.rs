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
