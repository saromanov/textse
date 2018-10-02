
// Searcher provides implementation of search
// on documents
pub struct Searcher {
    terms: Vec<Term>,
    ranges: Vec<Range>,
    datetime: i64,
    start: i32,
}

impl Searcher {
    fn search(&self) -> &SearchResults {
        if self.terms.len() > 0 {
            self.search_in_terms()
        }
    }

    fn search_in_terms(&self) {
        for term in self.terms {
            let field = term.get_field()
            let text = term.get_text()
        }
    }
}

// SearchResults returns results from Searcher
pub struct SearchResults;