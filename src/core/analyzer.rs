extern crate regex;
use regex::Regex;

struct AnalyzerOptions {

}

trait Analyzer {
    fn new(&self, text:&str) -> Analyzer;
    fn analyze(&self, opt:AnalyzerOptions) -> AnalyzerResult;
}

struct KeywordAnalyzer {
    text:&str;
}

// KeywordAnalyzer defines simple analyzer
// for splitting of text with commas and spaces 
impl Analyzer for KeywordAnalyzer {
    fn new(&self, text:&str) -> SimpleAnalyzer {
        SimpleAnalyzer{text:text}
    }

    fn analyze(&self, opt:AnalyzerOptions) -> AnalyzerResult {
        self.text.splitn(2, |x|x == ' ' || x == ',').unwrap()
    }
}

struct RegexpAnalyzer {
    text: &str;
}

impl Analyzer for RegexpAnalyzer {
    fn analyze(&self, opt:AnalyzerOptions) -> AnalyzerResult {
        let re = Regex::new(self.text).unwrap();
        re.is_match(opt.Text)
    }
} 