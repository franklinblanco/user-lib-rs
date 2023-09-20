pub struct HashResult {
    pub salt: String,
    pub hash: String
}

impl HashResult{
    pub fn new(salt: String, hash: String) -> HashResult{
        HashResult { salt, hash }
    }
}