use aidoku::alloc::{String, Vec};
use aidoku::serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ChaptersMetadata {
	pub current_page: i32,
	pub last_page: i32,
}

#[derive(Deserialize, Debug)]
pub struct ChaptersResponse {
	pub data: Vec<ChapterResponse>,
	pub meta: ChaptersMetadata,
}

#[derive(Deserialize, Debug)]
pub struct StatusResponse {
	pub id: i32,
	pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct MangaResponse {
	pub id: i32,
	pub cover: String,
	pub name: String,
	pub slug: String,
	pub status: StatusResponse,
	pub r#type: String,
}

#[derive(Deserialize, Debug)]
pub struct ChapterResponse {
	pub id: i32,
	pub name: String,
	pub published_at: String,
}

#[derive(Deserialize, Debug)]
pub struct MangaWithChapterResponse {
	pub id: i32,
	pub cover: String,
	pub last_chapters: Vec<ChapterResponse>,
	pub name: String,
	pub r#type: String,
	pub slug: String,
	pub status: Option<StatusResponse>,
}

#[derive(Deserialize, Debug)]
pub struct DataResponse {
	pub popular_comics: String,
	pub new_chapters: Vec<MangaWithChapterResponse>,
}

#[derive(Deserialize, Debug)]
pub struct HomepageResponse {
	pub data: DataResponse,
	pub rankings: Vec<MangaResponse>,
	pub success: bool,
}
