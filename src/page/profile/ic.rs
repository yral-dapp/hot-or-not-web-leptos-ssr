use std::{array, hash::Hash, marker::PhantomData};

use candid::Principal;
use futures::{Future, Stream, StreamExt};
use leptos::*;
use serde::{Deserialize, Serialize};

use crate::{
    canister::individual_user_template::{
        BetDirection, BetOutcomeForBetMaker, GetPostsOfUserProfileError, IndividualUserTemplate,
        PlacedBetDetail, PostDetailsForFrontend, Result4, UserProfileDetailsForFrontend,
    },
    component::bullet_loader::BulletLoader,
    consts::FALLBACK_PROPIC_BASE,
    state::canisters::Canisters,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct ProfileDetails {
    pub username: Option<String>,
    pub lifetime_earnings: u64,
    pub followers_cnt: u64,
    pub following_cnt: u64,
    pub profile_pic: Option<String>,
    pub display_name: Option<String>,
    pub principal: Principal,
    pub hots: u64,
    pub nots: u64,
}

impl From<UserProfileDetailsForFrontend> for ProfileDetails {
    fn from(user: UserProfileDetailsForFrontend) -> Self {
        Self {
            username: user.unique_user_name,
            lifetime_earnings: user.lifetime_earnings,
            followers_cnt: user.followers_count,
            following_cnt: user.following_count,
            profile_pic: user.profile_picture_url,
            display_name: user.display_name,
            principal: user.principal_id,
            hots: user.profile_stats.hot_bets_received,
            nots: user.profile_stats.not_bets_received,
        }
    }
}

fn color_from_principal(principal: Principal) -> String {
    let mut col_iter = principal
        .as_slice()
        .chunks(4)
        .map(|c| c.iter().fold(0u8, |acc, &x| acc.wrapping_add(x)));
    let colors: [u8; 3] = array::from_fn(|_| col_iter.next().unwrap());
    hex::encode(colors)
}

impl ProfileDetails {
    pub fn username_or_principal(&self) -> String {
        self.username
            .clone()
            .unwrap_or_else(|| self.principal.to_text())
    }

    pub fn display_name_or_fallback(&self) -> String {
        self.display_name
            .clone()
            .unwrap_or_else(|| self.username_or_principal())
    }

    pub fn profile_pic_or_random(&self) -> String {
        let propic = self.profile_pic.clone().unwrap_or_default();
        if !propic.is_empty() {
            return propic;
        }

        let background_color = color_from_principal(self.principal);
        format!(
            "{FALLBACK_PROPIC_BASE}?seed={}&backgroundColor={}&backgroundType=solid",
            self.principal.to_text(),
            background_color
        )
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PostDetails {
    pub id: u64,
    pub uid: String,
    pub likes: u64,
    pub views: u64,
}

impl From<&PostDetailsForFrontend> for PostDetails {
    fn from(post: &PostDetailsForFrontend) -> Self {
        Self {
            id: post.id,
            uid: post.video_uid.clone(),
            likes: post.like_count,
            views: post.total_view_count,
        }
    }
}

#[derive(Clone, Copy)]
pub enum BetOutcome {
    Won(u64),
    Draw(u64),
    Lost,
    AwaitingResult,
}

#[derive(Clone, Copy)]
pub enum BetKind {
    Hot,
    Not,
}

#[derive(Clone)]
pub struct BetDetails {
    pub outcome: BetOutcome,
    pub post_id: u64,
    pub canister_id: Principal,
    pub bet_kind: BetKind,
    pub bet_amount: u64,
}

impl From<&PlacedBetDetail> for BetDetails {
    fn from(bet: &PlacedBetDetail) -> Self {
        let outcome = match bet.outcome_received {
            BetOutcomeForBetMaker::Lost => BetOutcome::Lost,
            BetOutcomeForBetMaker::Draw(w) => BetOutcome::Draw(w),
            BetOutcomeForBetMaker::Won(w) => BetOutcome::Won(w),
            BetOutcomeForBetMaker::AwaitingResult => BetOutcome::AwaitingResult,
        };
        let bet_kind = match bet.bet_direction {
            BetDirection::Hot => BetKind::Hot,
            BetDirection::Not => BetKind::Not,
        };
        Self {
            outcome,
            post_id: bet.post_id,
            canister_id: bet.canister_id,
            bet_kind,
            bet_amount: bet.amount_bet,
        }
    }
}

const PROFILE_CHUNK_SZ: usize = 10;

trait GetterFn<Arg0>: Fn(Arg0, u64, u64) -> Self::OutputFuture {
    type OutputFuture: Future<Output = <Self as GetterFn<Arg0>>::Output>;
    type Output;
}

impl<Arg0, F: ?Sized, Fut> GetterFn<Arg0> for F
where
    F: Fn(Arg0, u64, u64) -> Fut,
    Fut: Future,
{
    type OutputFuture = Fut;
    type Output = Fut::Output;
}

fn profile_stream<R, I, C>(
    principal: Principal,
    getter: impl for<'x> GetterFn<
        &'x IndividualUserTemplate<'x>,
        Output = Result<R, ic_agent::AgentError>,
    >,
    conv: C,
) -> impl Stream<Item = Vec<I>>
where
    I: 'static,
    C: Fn(R) -> Option<Vec<I>> + 'static,
{
    let canisters = expect_context::<Canisters>();
    futures::stream::try_unfold(
        (getter, conv, canisters, 0usize, false),
        move |(getter, conv, canisters, mut cursor, mut ended)| async move {
            if ended {
                return Ok(None);
            }
            let user = canisters.individual_user(principal);

            let res = getter(&user, cursor as u64, (cursor + PROFILE_CHUNK_SZ) as u64).await?;
            let data = match conv(res) {
                Some(data) => data,
                None => return Ok(None),
            };
            cursor += PROFILE_CHUNK_SZ;
            if data.len() < PROFILE_CHUNK_SZ {
                ended = true;
            }
            Ok(Some((data, (getter, conv, canisters, cursor, ended))))
        },
    )
    .filter_map(move |res: Result<_, ic_agent::AgentError>| async move {
        match res {
            Ok(r) => Some(r),
            Err(e) => {
                log::warn!("failed to fetch data for {principal} due to {e}");
                None
            }
        }
    })
}

fn get_posts<'a>(
    user: &'a IndividualUserTemplate,
    cursor: u64,
    limit: u64,
) -> impl Future<Output = Result<Result4, ic_agent::AgentError>> + 'a {
    user.get_posts_of_this_user_profile_with_pagination(cursor, limit)
}

fn get_speculations<'a>(
    user: &'a IndividualUserTemplate,
    cursor: u64,
    limit: u64,
) -> impl Future<Output = Result<Vec<PlacedBetDetail>, ic_agent::AgentError>> + 'a {
    assert_eq!(limit, cursor + 10);
    user.get_hot_or_not_bets_placed_by_this_profile_with_pagination(cursor)
}

pub fn posts_stream(principal: Principal) -> impl Stream<Item = Vec<PostDetailsForFrontend>> {
    profile_stream(principal, get_posts, |res| match res {
        Result4::Ok(posts) => Some(posts),
        Result4::Err(e) => match e {
            GetPostsOfUserProfileError::ReachedEndOfItemsList => None,
            _ => panic!("unexpected error while fetching posts"),
        },
    })
}

pub fn speculations_stream(principal: Principal) -> impl Stream<Item = Vec<PlacedBetDetail>> {
    profile_stream(principal, get_speculations, Some)
}

#[component]
pub fn ProfileStream<T, I: 'static, S, K, KF, N, EF>(
    base_stream: S,
    key: KF,
    children: EF,
    #[prop(optional)] _ty: PhantomData<T>,
    #[prop(optional)] _ky: PhantomData<K>,
    #[prop(optional)] _child: PhantomData<N>,
) -> impl IntoView
where
    S: Stream<Item = Vec<I>> + 'static + Unpin,
    K: Eq + Hash + 'static,
    KF: Fn(&T) -> K + 'static,
    N: IntoView + 'static,
    EF: Fn(T) -> N + 'static,
    T: (for<'a> From<&'a I>) + 'static + Clone,
{
    let chunk_loaded = create_signal_from_stream(base_stream);
    let data = create_rw_signal(Vec::<T>::new());
    let data_loaded = create_rw_signal(false);

    create_effect(move |_| {
        with!(|chunk_loaded| {
            let Some(chunk) = chunk_loaded else {
                return;
            };
            if chunk.len() < PROFILE_CHUNK_SZ {
                data_loaded.set(true);
            }
            data.update(|data| data.extend(chunk.iter().map(T::from)));
        })
    });

    view! {
        <div class="flex flex-row-reverse gap-y-3 flex-wrap-reverse justify-center w-full">
            <For each=data key children/>
        </div>
        <Show when=move || !data_loaded()>
            <BulletLoader/>
        </Show>
    }
}
