use crate::db::id::UrlID;
use crate::db::models::Url;
use crate::Config;
use anyhow::Result;
use std::convert::TryInto;
use tantivy::{
    collector::TopDocs,
    doc,
    query::{BooleanQuery, FuzzyTermQuery},
    schema::{Field, Schema},
    Index, IndexReader, Term,
};
use tokio::task::block_in_place;

#[derive(Clone)]
pub struct SearchIndex {
    index: Index,
    reader: IndexReader,
    f_id: Field,
    f_title: Field,
    f_description: Field,
}

impl SearchIndex {
    /// Creates the search index.
    pub async fn new(conf: &Config) -> Result<SearchIndex> {
        use tantivy::schema::{STORED, TEXT};
        let mut builder = Schema::builder();
        let f_id = builder.add_bytes_field("id", STORED);
        let f_title = builder.add_text_field("title", TEXT);
        let f_description = builder.add_text_field("description", TEXT);
        let schema = builder.build();

        let index = if let Some(path) = conf.search_index() {
            tokio::fs::create_dir_all(path).await?;
            Index::create_in_dir(path, schema)?
        } else {
            Index::create_in_ram(schema)
        };

        Ok(SearchIndex {
            reader: index.reader()?,
            index,
            f_id,
            f_title,
            f_description,
        })
    }

    /// Adds a given `Url` to the index.
    pub fn index_url(&self, url: &Url) -> Result<()> {
        self.index_urls(std::iter::once(url))
    }

    /// Adds a list of given `Url`s to the index.
    pub fn index_urls<'a, I>(&self, urls: I) -> Result<()>
    where
        I: std::iter::Iterator<Item = &'a Url>,
    {
        block_in_place(|| {
            let mut writer = self.index.writer(100_000_000)?;
            for url in urls {
                writer.add_document(doc! {
                    self.f_id => url.id().as_bytes(),
                    self.f_title => url.title().unwrap_or(""),
                    self.f_description => url.description().unwrap_or(""),
                });
            }
            writer.commit()?;
            Ok(())
        })
    }

    /// Searches the index and returns all url IDs
    /// matching the given query.
    pub fn find(&self, query: &str, offset: usize, limit: usize) -> Result<Vec<UrlID>> {
        block_in_place(|| {
            let title = Term::from_field_text(self.f_title, query);
            let title = FuzzyTermQuery::new(title, 2, false);
            let desc = Term::from_field_text(self.f_description, query);
            let desc = FuzzyTermQuery::new(desc, 2, false);
            let query = BooleanQuery::union(vec![Box::new(title), Box::new(desc)]);

            let searcher = self.reader.searcher();
            let top_docs = TopDocs::with_limit(limit).and_offset(offset);
            let docs = searcher.search(&query, &top_docs)?;
            let ids = docs
                .into_iter()
                .filter_map(|(_, addr)| searcher.doc(addr).ok())
                .filter_map(|doc| {
                    let id = doc.get_first(self.f_id)?.bytes_value()?.try_into().ok()?;
                    Some(id)
                })
                .collect();
            Ok(ids)
        })
    }
}
