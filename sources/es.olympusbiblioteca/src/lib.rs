#![no_std]
use aidoku::{
	alloc::{vec, String, Vec},
	prelude::*,
	Chapter, FilterValue, Manga, MangaPageResult, Page, Result, Source,
};

mod home;
pub mod models;

const BASE_URL: &str = "https://olympusbiblioteca.com";

struct OlympusScanlation;

impl Source for OlympusScanlation {
	fn new() -> Self {
		Self
	}

	fn get_search_manga_list(
		&self,
		_query: Option<String>,
		_page: i32,
		_filters: Vec<FilterValue>,
	) -> Result<MangaPageResult> {
		let entries = vec![Manga {
			..Default::default()
		}];

		Ok(MangaPageResult {
			entries,
			has_next_page: false,
		})
	}

	fn get_manga_update(
		&self,
		_manga: Manga,
		_needs_details: bool,
		_needs_chapters: bool,
	) -> Result<Manga> {
		let manga = Manga {
			..Default::default()
		};

		Ok(manga)
	}

	fn get_page_list(&self, _manga: Manga, _chapter: Chapter) -> Result<Vec<Page>> {
		let pages = vec![Page {
			..Default::default()
		}];

		Ok(pages)
	}
}

register_source!(OlympusScanlation, Home);
