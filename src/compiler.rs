use std::collections::HashMap;

pub struct Compiler {
    map: HashMap<char, String>,
}

impl Compiler {
    fn number(n: usize) -> String {
        let zero = "+[]";
        let one = "+!![]";

        if n == 0 {
            zero.to_owned()
        } else {
            let v = vec![one; n];
            v.join("+ ")
        }
    }

    fn string(&self, s: impl Into<String>) -> String {
        let s: String = s.into();

        s.chars()
            .map(|z| {
                if !self.map.contains_key(&z) {
                    // FIXME: we assume we need just one u16, we just panic otherwise
                    let mut b = [0; 1];
                    z.encode_utf16(&mut b);

                    format!(
                        "([]+[])[{}][{}]({})",
                        self.string("constructor"),
                        self.string("fromCharCode"),
                        Self::number(b[0].into())
                    )
                } else {
                    self.map.get(&z).expect("unreachable").to_owned()
                }
            })
            .collect::<Vec<String>>()
            .join("+")
    }

    fn populate_map(&mut self) {
        self.map
            .insert('a', format!("(+{{}}+[])[{}]", Self::number(1)));
        self.map
            .insert('b', format!("({{}}+[])[{}]", Self::number(2)));
        self.map
            .insert('o', format!("({{}}+[])[{}]", Self::number(1)));
        self.map
            .insert('e', format!("({{}}+[])[{}]", Self::number(4)));
        self.map
            .insert('c', format!("({{}}+[])[{}]", Self::number(5)));
        self.map
            .insert('t', format!("({{}}+[])[{}]", Self::number(6)));
        self.map
            .insert(' ', format!("({{}}+[])[{}]", Self::number(7)));
        self.map
            .insert('f', format!("(![]+[])[{}]", Self::number(0)));
        self.map
            .insert('s', format!("(![]+[])[{}]", Self::number(3)));
        self.map
            .insert('r', format!("(!![]+[])[{}]", Self::number(1)));
        self.map
            .insert('u', format!("(!![]+[])[{}]", Self::number(2)));
        self.map
            .insert('i', format!("((+!![]/+[])+[])[{}]", Self::number(3)));
        self.map
            .insert('n', format!("((+!![]/+[])+[])[{}]", Self::number(4)));
        self.map.insert(
            'S',
            format!(
                "([]+([]+[])[{}])[{}]",
                self.string("constructor"),
                Self::number(9)
            ),
        );
        self.map.insert(
            'g',
            format!(
                "([]+([]+[])[{}])[{}]",
                self.string("constructor"),
                Self::number(14)
            ),
        );
        self.map
            .insert('\\', format!("(/\\\\/+[])[{}]", Self::number(1)));
        self.map.insert(
            'p',
            format!(
                "([]+(/-/)[{}])[{}]",
                self.string("constructor"),
                Self::number(14)
            ),
        );
        self.map.insert(
            'd',
            format!(
                "({})[{}]({})",
                Self::number(13),
                self.string("toString"),
                Self::number(14)
            ),
        );
        self.map.insert(
            'h',
            format!(
                "({})[{}]({})",
                Self::number(17),
                self.string("toString"),
                Self::number(18)
            ),
        );
        self.map.insert(
            'm',
            format!(
                "({})[{}]({})",
                Self::number(22),
                self.string("toString"),
                Self::number(23)
            ),
        );
        self.map.insert(
            'C',
            format!(
                "((()=>{{}})[{}]({})()({}))[{}]",
                self.string("constructor"),
                self.string("return escape"),
                self.map.get(&'\\').expect("unreachable"),
                Self::number(2)
            ),
        );
    }

    pub fn compile(&self, code: impl Into<String>) -> String {
        format!(
            "(()=>{{}})[{}]({})()",
            self.string("constructor"),
            self.string(code)
        )
    }

    pub fn new() -> Self {
        let mut result = Self {
            map: HashMap::new(),
        };

        result.populate_map();
        result
    }
}
