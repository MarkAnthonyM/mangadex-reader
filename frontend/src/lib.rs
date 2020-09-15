extern crate seed;
use seed::{ prelude::*, * };
use mangadex_reader::{ JsonApiResponse, MangaJsonWrapper };

struct Model {
    task: Vec<MangaJsonWrapper>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
