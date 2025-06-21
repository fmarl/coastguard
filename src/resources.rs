use std::collections::HashMap;

include!(concat!(env!("OUT_DIR"), "/tlds.rs"));

pub fn homoglyphs() -> HashMap<char, Vec<char>> {
    let mut map = HashMap::new();

    map.insert('A', vec!['А', 'Ꭺ', 'Α', 'ᴀ', 'ꓮ', 'Ɐ']);
    map.insert('B', vec!['Β', 'В', 'Ᏼ', 'ʙ', 'Ꞗ']);
    map.insert('C', vec!['Ϲ', 'С', 'Ꮯ', 'Ⅽ', 'Ⲥ']);
    map.insert('D', vec!['Ꭰ', 'Ⅾ', 'ԁ', 'ꓓ']);
    map.insert('E', vec!['Ε', 'Е', 'Ꭼ', 'ⅇ', 'ꓰ']);
    map.insert('F', vec!['Ϝ', 'Ғ', 'ᖴ']);
    map.insert('G', vec!['Ԍ', 'ɢ', 'Ɠ']);
    map.insert('H', vec!['Η', 'Н', 'Ꮋ', 'ʜ', 'Ⲏ']);
    map.insert('I', vec!['Ι', 'І', 'Ӏ', 'Ꭵ', 'ꓲ']);
    map.insert('J', vec!['Ј', 'Ϳ', 'ʝ']);
    map.insert('K', vec!['Κ', 'К', 'Ꮶ', 'Ⲕ']);
    map.insert('L', vec!['Ꮮ', 'ʟ', 'Ⳑ']);
    map.insert('M', vec!['Μ', 'М', 'Ꮇ', 'Ⅿ', 'Ⲙ']);
    map.insert('N', vec!['Ν', 'Ⲛ', 'Ꮋ', 'ꓠ']);
    map.insert('O', vec!['Ο', 'О', 'Օ', '०', 'Ⲟ', 'ꓳ']);
    map.insert('P', vec!['Ρ', 'Р', 'Ꮲ', 'Ⲣ']);
    map.insert('Q', vec!['Ⴓ']);
    map.insert('R', vec!['Ꮢ', 'ʀ', 'Ⲣ']);
    map.insert('S', vec!['Ѕ', 'Տ', 'Ꮪ', 'ꓢ']);
    map.insert('T', vec!['Τ', 'Т', 'Ꭲ', 'Ⲧ']);
    map.insert('U', vec!['Ս', 'Ս', 'Ս', '⋃', 'ꓴ']);
    map.insert('V', vec!['Ѵ', 'Ꮩ', 'ⴸ']);
    map.insert('W', vec!['Ԝ', 'Ꮃ']);
    map.insert('X', vec!['Χ', 'Х', 'Ⅹ', 'Ⲭ']);
    map.insert('Y', vec!['Υ', 'Ү', 'Ⲩ', 'ʏ']);
    map.insert('Z', vec!['Ζ', 'Ꮓ', 'Ⲍ']);
    map.insert('a', vec!['а', 'ɑ', 'ᴀ']);
    map.insert('b', vec!['Ь', 'Ꮟ', 'Ƅ']);
    map.insert('c', vec!['ϲ', 'с', 'ⅽ']);
    map.insert('d', vec!['ԁ', 'ɗ']);
    map.insert('e', vec!['е', 'ҽ']);
    map.insert('f', vec!['ϝ', 'ғ']);
    map.insert('g', vec!['ɡ', 'ɢ']);
    map.insert('h', vec!['һ', 'հ']);
    map.insert('i', vec!['і', 'Ꭵ', 'ı']);
    map.insert('j', vec!['ј']);
    map.insert('k', vec!['κ', 'к']);
    map.insert('l', vec!['ⅼ', 'ӏ', 'Ɩ']);
    map.insert('m', vec!['м']);
    map.insert('n', vec!['п', 'ո']);
    map.insert('o', vec!['ο', 'о', 'օ']);
    map.insert('p', vec!['ρ', 'р']);
    map.insert('q', vec!['զ']);
    map.insert('r', vec!['г', 'ᴦ']);
    map.insert('s', vec!['ѕ']);
    map.insert('t', vec!['τ', 'т']);
    map.insert('u', vec!['υ', 'ս']);
    map.insert('v', vec!['ѵ']);
    map.insert('w', vec!['ѡ']);
    map.insert('x', vec!['х']);
    map.insert('y', vec!['у', 'ү']);
    map.insert('z', vec!['ᴢ', 'ȥ']);

    map
}
