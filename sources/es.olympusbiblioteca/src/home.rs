use crate::models::*;
use crate::{OlympusScanlation, BASE_URL};
use aidoku::{
	alloc::string::ToString,
	alloc::Vec,
	imports::net::Request,
	imports::std::{parse_date, send_partial_result},
	prelude::*,
	AidokuError, Chapter, Home, HomeComponent, HomeLayout, HomePartialResult, Link, Manga,
	MangaWithChapter, Result,
};

impl Home for OlympusScanlation {
	fn get_home(&self) -> Result<HomeLayout> {
		let url = format!("{BASE_URL}/api/homepage");
		let result = Request::get(&url)?.send()?.get_json::<HomepageResponse>()?;

		if !result.success {
			return Err(error!("Unable to load homepage"));
		}

		let popular = serde_json::from_str::<Vec<MangaResponse>>(&result.data.popular_comics)
			.map_err(AidokuError::JsonParseError)?
			.into_iter()
			.filter_map(|entry| {
				if entry.r#type != "comic" {
					return None;
				}

				Some(Manga {
					key: entry.slug.clone(),
					cover: Some(entry.cover),
					title: entry.name,
					url: format!("{BASE_URL}/series/{}", entry.slug).into(),
					..Default::default()
				})
			})
			.map(Into::into)
			.collect::<Vec<Link>>();

		send_partial_result(&HomePartialResult::Component(HomeComponent {
			title: Some("Popular Del Dia".to_string()),
			value: aidoku::HomeComponentValue::Scroller {
				entries: popular,
				listing: None,
			},
			..Default::default()
		}));

		let nuevos_lanzamientos = result
			.data
			.new_chapters
			.into_iter()
			.filter_map(|entry| {
				if entry.r#type != "comic" {
					return None;
				}

				let last_chapter = &entry.last_chapters[0];
				let manga_url = format!("{BASE_URL}/series/{}", entry.slug).into();
				let chapter_url =
					format!("{BASE_URL}/capitulo/{}/{}", last_chapter.id, entry.slug).into();

				Some(MangaWithChapter {
					manga: Manga {
						key: entry.slug,
						cover: Some(entry.cover),
						title: entry.name,
						url: manga_url,
						..Default::default()
					},
					chapter: Chapter {
						key: last_chapter.id.to_string(),
						url: chapter_url,
						title: Some("".to_string()),
						chapter_number: Some(last_chapter.name.parse::<f32>().ok()?),
						date_uploaded: parse_date(
							last_chapter.published_at.clone(),
							"yyyy-MM-dd'T'HH:mm:ss.SSSSSSZ",
						),
						..Default::default()
					},
				})
			})
			.collect::<Vec<MangaWithChapter>>();

		send_partial_result(&HomePartialResult::Component(HomeComponent {
			title: Some("Nuevos Lanzamientos".to_string()),
			value: aidoku::HomeComponentValue::MangaChapterList {
				entries: nuevos_lanzamientos,
				listing: None,
				page_size: Some(5),
			},
			..Default::default()
		}));

		let top_series = result
			.rankings
			.into_iter()
			.filter_map(|entry| {
				if entry.r#type != "comic" {
					return None;
				}

				Some(Manga {
					key: entry.slug.clone(),
					cover: Some(entry.cover),
					title: entry.name,
					url: format!("{BASE_URL}/series/{}", entry.slug).into(),
					..Default::default()
				})
			})
			.map(Into::into)
			.collect::<Vec<Link>>();

		send_partial_result(&HomePartialResult::Component(HomeComponent {
			title: Some("Top Series".to_string()),
			value: aidoku::HomeComponentValue::MangaList {
				ranking: true,
				entries: top_series,
				listing: None,
				page_size: Some(5),
			},
			..Default::default()
		}));

		Ok(HomeLayout::default())
	}
}
