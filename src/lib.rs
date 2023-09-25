use std::path::{Path, PathBuf};
use clap::Parser;
use lopdf::Document;

/// Partition a vector 'v', splitting at each index in 'splits'.
pub(crate) fn partition_vec<T: Clone>(v: Vec<T>, splits: &[usize]) -> Vec<Vec<T>> {
    let mut partitions = Vec::new();
    let mut prev_split = 0;
    for split in splits {
        partitions.push(v[prev_split..*split].to_vec());
        prev_split = *split;
    }
    partitions.push(v[prev_split..].to_vec());
    partitions
}

// todo: this really should return an iterator, lol
pub fn split_document(mut doc: &Document, split_at_pages: &[usize]) -> Vec<Document> {
    let doc_pages = doc.get_pages();
    let splits = partition_vec(doc_pages.keys().clone().collect(), split_at_pages);
    let mut new_docs = Vec::new();

    for spl in splits {
        let mut new_doc = Document::with_version(&doc.version);
        for page in spl {
            let page = doc_pages.get(page).unwrap();
            new_doc.add_object(*page);
        }
        new_docs.push(new_doc);
    }
    new_docs
}

#[derive(Parser, Debug)]
pub struct Args {
    /// Input path to PDF file
    #[clap(short, long)]
    pub input: PathBuf,

    /// Output folder for split PDFs
    #[clap(short, long)]
    pub output: PathBuf,
    #[clap(short, long)]
    pub split_at_pages: Vec<usize>,
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_partition_vec() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let splits = vec![2, 5, 7];
        let partitions = partition_vec(v, &splits);
        assert_eq!(partitions, vec![vec![1, 2], vec![3, 4, 5], vec![6, 7], vec![8, 9]]);
    }
}
