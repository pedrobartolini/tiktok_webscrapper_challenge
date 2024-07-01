async fn get_tiktok_stats(user: &str) -> anyhow::Result<(i32, i32, i32)> {
   const DEFAULT_ERROR: &str = "Error getting TikTok stats";

   let url = format!("https://www.tiktok.com/@{}", user);
   let response = reqwest::get(&url).await?.text().await?;

   let document = scraper::Html::parse_document(&response);
   let doc_as_string = document.root_element().inner_html();

   let follower_count_pos = doc_as_string.find(r#""followerCount":"#).ok_or(anyhow::anyhow!(DEFAULT_ERROR))?;
   let follower_count_end_pos = doc_as_string[follower_count_pos + 15..].find(",").ok_or(anyhow::anyhow!(DEFAULT_ERROR))?;

   let following_count_pos = doc_as_string.find(r#""followingCount":"#).ok_or(anyhow::anyhow!(DEFAULT_ERROR))?;
   let following_count_end_pos = doc_as_string[following_count_pos + 17..].find(",").ok_or(anyhow::anyhow!(DEFAULT_ERROR))?;

   let likes_count_pos = doc_as_string.find(r#""heartCount":"#).ok_or(anyhow::anyhow!(DEFAULT_ERROR))?;
   let likes_count_end_pos = doc_as_string[likes_count_pos + 13..].find(",").ok_or(anyhow::anyhow!(DEFAULT_ERROR))?;

   let followers = &doc_as_string[follower_count_pos + 16..follower_count_pos + 15 + follower_count_end_pos];
   let following = &doc_as_string[following_count_pos + 17..following_count_pos + 17 + following_count_end_pos];
   let likes = &doc_as_string[likes_count_pos + 13..likes_count_pos + 13 + likes_count_end_pos];

   Ok((followers.parse::<i32>()?, following.parse::<i32>()?, likes.parse::<i32>()?))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
   let now = tokio::time::Instant::now();

   let (followers, following, likes) = get_tiktok_stats("instagram").await?;

   println!("Followers: {}", followers);
   println!("Following: {}", following);
   println!("Likes: {}", likes);

   println!("Elapsed time: {:?}", now.elapsed());

   Ok(())
}
