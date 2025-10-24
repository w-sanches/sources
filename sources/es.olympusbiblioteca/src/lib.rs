#![no_std]
use crate::models::*;
use aidoku::{
	alloc::string::ToString,
	alloc::{vec, String, Vec},
	imports::net::Request,
	imports::std::{parse_date, send_partial_result},
	prelude::*,
	Chapter, FilterValue, Manga, MangaPageResult, Page, Result, Source,
};

mod home;
pub mod models;

const BASE_URL: &str = "https://olympusbiblioteca.com";
const API_URL: &str = "https://dashboard.olympusbiblioteca.com/api";

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
		mut manga: Manga,
		_needs_details: bool,
		needs_chapters: bool,
	) -> Result<Manga> {
		if needs_chapters {
			let mut current_page = 1;
			manga.chapters = Some(Vec::new());

			loop {
				let url = format!(
					"{API_URL}/series/{}/chapters?page={}",
					&manga.key, current_page
				);
				let result = Request::get(&url)?.send()?.get_json::<ChaptersResponse>()?;

				let chapters = result
					.data
					.into_iter()
					.map(|entry| Chapter {
						key: entry.id.to_string(),
						url: format!("{BASE_URL}/capitulo/{}/{}", entry.id, manga.key).into(),
						title: Some(entry.name.clone()),
						chapter_number: entry.name.parse::<f32>().ok(),
						date_uploaded: parse_date(
							entry.published_at,
							"yyyy-MM-dd'T'HH:mm:ss.SSSSSSZ",
						),
						..Default::default()
					})
					.collect::<Vec<_>>();

				if let Some(ref mut manga_chapters) = manga.chapters {
					manga_chapters.extend(chapters);
				}

				send_partial_result(&manga);

				if current_page >= result.meta.last_page {
					break;
				}

				current_page += 1;
			}
		}

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
