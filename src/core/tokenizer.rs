
trait BasicTokenizer {
    fn process();
}

struct TokenizerPunkt {
    text: &'a str,
    chars: Vec<'a>,
}

impl TokenizerPunkt for BasicTokenizer {
    pub fn new(&self, text: &'a str) -> TokenizerPunkt {
        TokenizerPunkt{text: text}
    }

    // splitting of text by pinctuation and defined punctuation
    pub fn process(&mut self) -> TokenizerPunkt {
        self.text.splitn(2, |x| x == ','||x == '.'||x == ':'||x == ' ').collect()
    }
}