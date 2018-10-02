
// implementation of manager for tokenizers for text

struct TokenizerManager {
    tokenizers: Arc<RwLock<HashMap<String, BasicTokenizer>>>,
}

impl TokenizerManager {
    pub fn make() -> TokenizerManager {
        TokenizerManager{tokenizers: Arc::new(RwLock::new(HashMap::new())),}
    }

    pub fn register(name: String, token: BasicTokenizer) -> bool {
        self.tokenizers.write(name, token)
        true
    }

    // deregister of tokenizer manager
    pub fn deregister(name: String) -> Option<bool> {
        self.tokenizer.remove(name)
    }
}