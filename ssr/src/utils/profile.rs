use candid::Principal;
use ic_agent::AgentError;
use leptos::prelude::{RwSignal, UpdateUntracked};

use yral_canisters_client::individual_user_template::Result12;

use yral_canisters_common::{
    cursored_data::{CursoredDataProvider, PageEntry},
    utils::posts::PostDetails,
    Canisters,
};

pub const PROFILE_CHUNK_SZ: usize = 10;

#[derive(Clone)]
pub struct PostsProvider {
    canisters: Canisters<false>,
    video_queue: RwSignal<Vec<PostDetails>>,
    start_index: RwSignal<usize>,
    user: Principal,
}

impl PostsProvider {
    pub fn new(
        canisters: Canisters<false>,
        video_queue: RwSignal<Vec<PostDetails>>,
        start_index: RwSignal<usize>,
        user: Principal,
    ) -> Self {
        Self {
            canisters,
            video_queue,
            start_index,
            user,
        }
    }
}

impl CursoredDataProvider for PostsProvider {
    type Data = PostDetails;
    type Error = AgentError;

    async fn get_by_cursor_inner(
        &self,
        start: usize,
        end: usize,
    ) -> Result<PageEntry<PostDetails>, AgentError> {
        let user = self.canisters.individual_user(self.user).await;
        let limit = end - start;
        let posts = user
            .get_posts_of_this_user_profile_with_pagination_cursor(start as u64, limit as u64)
            .await?;
        let posts = match posts {
            Result12::Ok(v) => v,
            Result12::Err(_) => {
                log::warn!("failed to get posts");
                return Ok(PageEntry {
                    data: vec![],
                    end: true,
                });
            }
        };
        let list_end = posts.len() < (end - start);
        self.start_index.update_untracked(|c| *c = end);
        let post_details: Vec<PostDetails> = posts
            .into_iter()
            .map(|details| PostDetails::from_canister_post(false, self.user, details))
            .collect();
        self.video_queue.update_untracked(|vq| {
            vq.extend_from_slice(&post_details);
        });
        Ok(PageEntry {
            data: post_details,
            end: list_end,
        })
    }
}
