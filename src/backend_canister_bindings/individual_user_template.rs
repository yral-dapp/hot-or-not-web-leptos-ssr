// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Decode, Deserialize, Encode, Principal};
type Result<T> = std::result::Result<T, ic_agent::AgentError>;

#[derive(CandidType, Deserialize)]
pub enum KnownPrincipalType {
    CanisterIdUserIndex,
    CanisterIdProjectMemberIndex,
    CanisterIdTopicCacheIndex,
    CanisterIdRootCanister,
    CanisterIdPostCache,
    #[serde(rename = "CanisterIdSNSController")]
    CanisterIdSnsController,
    UserIdGlobalSuperAdmin,
}

#[derive(CandidType, Deserialize)]
pub struct IndividualUserTemplateInitArgs {
    pub known_principal_ids: Vec<(KnownPrincipalType, Principal)>,
    pub profile_owner: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct PostDetailsFromFrontend {
    pub hashtags: Vec<String>,
    pub description: String,
    pub video_uid: String,
    pub creator_consent_for_inclusion_in_hot_or_not: bool,
}

#[derive(CandidType, Deserialize)]
pub enum PostStatus {
    BannedForExplicitness,
    BannedDueToUserReporting,
    Uploaded,
    CheckingExplicitness,
    ReadyToView,
    Transcoding,
    Deleted,
}

#[derive(CandidType, Deserialize)]
pub struct PostDetailsForFrontend {
    pub id: u64,
    pub status: PostStatus,
    pub home_feed_ranking_score: u64,
    pub hashtags: Vec<String>,
    pub like_count: u64,
    pub description: String,
    pub total_view_count: u64,
    pub created_by_display_name: Option<String>,
    pub created_by_unique_user_name: Option<String>,
    pub video_uid: String,
    pub created_by_user_principal_id: Principal,
    pub hot_or_not_feed_ranking_score: Option<u64>,
    pub liked_by_me: bool,
    pub created_by_profile_photo_url: Option<String>,
}

#[derive(CandidType, Deserialize)]
pub enum GetPostsOfUserProfileError {
    ReachedEndOfItemsList,
    InvalidBoundsPassed,
    ExceededMaxNumberOfItemsAllowedInOneRequest,
}

#[derive(CandidType, Deserialize)]
pub enum Result_ {
    Ok(Vec<PostDetailsForFrontend>),
    Err(GetPostsOfUserProfileError),
}

#[derive(CandidType, Deserialize)]
pub enum GetFollowerOrFollowingError {
    ReachedEndOfItemsList,
    InvalidBoundsPassed,
    ExceededMaxNumberOfItemsAllowedInOneRequest,
}

#[derive(CandidType, Deserialize)]
pub enum Result1 {
    Ok(Vec<Principal>),
    Err(GetFollowerOrFollowingError),
}

#[derive(CandidType, Deserialize)]
pub struct UserProfileGlobalStats {
    pub lifetime_earnings: u64,
    pub hots_earned_count: u64,
    pub nots_earned_count: u64,
}

#[derive(CandidType, Deserialize)]
pub struct UserProfileDetailsForFrontend {
    pub unique_user_name: Option<String>,
    pub following_count: u64,
    pub profile_picture_url: Option<String>,
    pub display_name: Option<String>,
    pub principal_id: Principal,
    pub profile_stats: UserProfileGlobalStats,
    pub followers_count: u64,
}

#[derive(CandidType, Deserialize)]
pub enum UserAccessRole {
    CanisterController,
    ProfileOwner,
    CanisterAdmin,
    ProjectCanister,
}

#[derive(CandidType, Deserialize)]
pub struct SystemTime {
    pub nanos_since_epoch: u32,
    pub secs_since_epoch: u64,
}

#[derive(CandidType, Deserialize)]
pub enum MintEvent {
    NewUserSignup {
        new_user_principal_id: Principal,
    },
    Referral {
        referrer_user_principal_id: Principal,
        referee_user_principal_id: Principal,
    },
}

#[derive(CandidType, Deserialize)]
pub enum TokenEventV1 {
    Stake,
    Burn,
    Mint {
        timestamp: SystemTime,
        details: MintEvent,
    },
    Transfer,
}

#[derive(CandidType, Deserialize)]
pub enum Result2 {
    Ok(Vec<(u64, TokenEventV1)>),
    Err(GetFollowerOrFollowingError),
}

#[derive(CandidType, Deserialize)]
pub enum PostViewDetailsFromFrontend {
    WatchedMultipleTimes {
        percentage_watched: u8,
        watch_count: u8,
    },
    WatchedPartially {
        percentage_watched: u8,
    },
}

#[derive(CandidType, Deserialize)]
pub enum FollowAnotherUserProfileError {
    UserToFollowDoesNotExist,
    UserIndexCrossCanisterCallFailed,
    UserITriedToFollowCrossCanisterCallFailed,
    UsersICanFollowListIsFull,
    #[serde(
        rename = "MyCanisterIDDoesNotMatchMyPrincipalCanisterIDMappingSeenByUserITriedToFollow"
    )]
    MyCanisterIdDoesNotMatchMyPrincipalCanisterIdMappingSeenByUserITriedToFollow,
    UserITriedToFollowDidNotFindMe,
    NotAuthorized,
    UserITriedToFollowHasTheirFollowersListFull,
}

#[derive(CandidType, Deserialize)]
pub enum Result3 {
    Ok(bool),
    Err(FollowAnotherUserProfileError),
}

#[derive(CandidType, Deserialize)]
pub enum AnotherUserFollowedMeError {
    UserIndexCrossCanisterCallFailed,
    FollowersListFull,
    NotAuthorized,
    UserTryingToFollowMeDoesNotExist,
}

#[derive(CandidType, Deserialize)]
pub enum Result4 {
    Ok(bool),
    Err(AnotherUserFollowedMeError),
}

#[derive(CandidType, Deserialize)]
pub struct UserProfileUpdateDetailsFromFrontend {
    pub profile_picture_url: Option<String>,
    pub display_name: Option<String>,
}

#[derive(CandidType, Deserialize)]
pub enum UpdateProfileDetailsError {
    NotAuthorized,
}

#[derive(CandidType, Deserialize)]
pub enum Result5 {
    Ok(UserProfileDetailsForFrontend),
    Err(UpdateProfileDetailsError),
}

#[derive(CandidType, Deserialize)]
pub enum UpdateProfileSetUniqueUsernameError {
    UsernameAlreadyTaken,
    UserIndexCrossCanisterCallFailed,
    SendingCanisterDoesNotMatchUserCanisterId,
    NotAuthorized,
    UserCanisterEntryDoesNotExist,
}

#[derive(CandidType, Deserialize)]
pub enum Result6 {
    Ok,
    Err(UpdateProfileSetUniqueUsernameError),
}

pub struct Service<'a>(pub Principal, pub &'a ic_agent::Agent);
impl<'a> Service<'a> {
    pub async fn add_post(&self, arg0: PostDetailsFromFrontend) -> Result<u64> {
        let args = Encode!(&arg0)?;
        let bytes = self
            .1
            .update(&self.0, "add_post")
            .with_arg(args)
            .call_and_wait()
            .await?;
        Ok(Decode!(&bytes, u64)?)
    }
    pub async fn get_following_status_do_i_follow_this_user(
        &self,
        arg0: Principal,
    ) -> Result<bool> {
        let args = Encode!(&arg0)?;
        let bytes = self
            .1
            .query(&self.0, "get_following_status_do_i_follow_this_user")
            .with_arg(args)
            .call()
            .await?;
        Ok(Decode!(&bytes, bool)?)
    }
    pub async fn get_individual_post_details_by_id(
        &self,
        arg0: u64,
    ) -> Result<PostDetailsForFrontend> {
        let args = Encode!(&arg0)?;
        let bytes = self
            .1
            .query(&self.0, "get_individual_post_details_by_id")
            .with_arg(args)
            .call()
            .await?;
        Ok(Decode!(&bytes, PostDetailsForFrontend)?)
    }
    pub async fn get_posts_of_this_user_profile_with_pagination(
        &self,
        arg0: u64,
        arg1: u64,
    ) -> Result<Result_> {
        let args = Encode!(&arg0, &arg1)?;
        let bytes = self
            .1
            .query(&self.0, "get_posts_of_this_user_profile_with_pagination")
            .with_arg(args)
            .call()
            .await?;
        Ok(Decode!(&bytes, Result_)?)
    }
    pub async fn get_principals_i_follow_paginated(&self, arg0: u64, arg1: u64) -> Result<Result1> {
        let args = Encode!(&arg0, &arg1)?;
        let bytes = self
            .1
            .query(&self.0, "get_principals_i_follow_paginated")
            .with_arg(args)
            .call()
            .await?;
        Ok(Decode!(&bytes, Result1)?)
    }
    pub async fn get_principals_that_follow_me_paginated(
        &self,
        arg0: u64,
        arg1: u64,
    ) -> Result<Result1> {
        let args = Encode!(&arg0, &arg1)?;
        let bytes = self
            .1
            .query(&self.0, "get_principals_that_follow_me_paginated")
            .with_arg(args)
            .call()
            .await?;
        Ok(Decode!(&bytes, Result1)?)
    }
    pub async fn get_profile_details(&self) -> Result<UserProfileDetailsForFrontend> {
        let args = Encode!()?;
        let bytes = self
            .1
            .query(&self.0, "get_profile_details")
            .with_arg(args)
            .call()
            .await?;
        Ok(Decode!(&bytes, UserProfileDetailsForFrontend)?)
    }
    pub async fn get_rewarded_for_referral(&self, arg0: Principal, arg1: Principal) -> Result<()> {
        let args = Encode!(&arg0, &arg1)?;
        let bytes = self
            .1
            .update(&self.0, "get_rewarded_for_referral")
            .with_arg(args)
            .call_and_wait()
            .await?;
        Ok(Decode!(&bytes)?)
    }
    pub async fn get_rewarded_for_signing_up(&self) -> Result<()> {
        let args = Encode!()?;
        let bytes = self
            .1
            .update(&self.0, "get_rewarded_for_signing_up")
            .with_arg(args)
            .call_and_wait()
            .await?;
        Ok(Decode!(&bytes)?)
    }
    pub async fn get_user_roles(&self, arg0: Principal) -> Result<Vec<UserAccessRole>> {
        let args = Encode!(&arg0)?;
        let bytes = self
            .1
            .query(&self.0, "get_user_roles")
            .with_arg(args)
            .call()
            .await?;
        Ok(Decode!(&bytes, Vec<UserAccessRole>)?)
    }
    pub async fn get_user_utility_token_transaction_history_with_pagination(
        &self,
        arg0: u64,
        arg1: u64,
    ) -> Result<Result2> {
        let args = Encode!(&arg0, &arg1)?;
        let bytes = self
            .1
            .query(
                &self.0,
                "get_user_utility_token_transaction_history_with_pagination",
            )
            .with_arg(args)
            .call()
            .await?;
        Ok(Decode!(&bytes, Result2)?)
    }
    pub async fn get_utility_token_balance(&self) -> Result<u64> {
        let args = Encode!()?;
        let bytes = self
            .1
            .query(&self.0, "get_utility_token_balance")
            .with_arg(args)
            .call()
            .await?;
        Ok(Decode!(&bytes, u64)?)
    }
    pub async fn return_cycles_to_user_index_canister(&self) -> Result<()> {
        let args = Encode!()?;
        let bytes = self
            .1
            .update(&self.0, "return_cycles_to_user_index_canister")
            .with_arg(args)
            .call_and_wait()
            .await?;
        Ok(Decode!(&bytes)?)
    }
    pub async fn update_post_add_view_details(
        &self,
        arg0: u64,
        arg1: PostViewDetailsFromFrontend,
    ) -> Result<()> {
        let args = Encode!(&arg0, &arg1)?;
        let bytes = self
            .1
            .update(&self.0, "update_post_add_view_details")
            .with_arg(args)
            .call_and_wait()
            .await?;
        Ok(Decode!(&bytes)?)
    }
    pub async fn update_post_as_ready_to_view(&self, arg0: u64) -> Result<()> {
        let args = Encode!(&arg0)?;
        let bytes = self
            .1
            .update(&self.0, "update_post_as_ready_to_view")
            .with_arg(args)
            .call_and_wait()
            .await?;
        Ok(Decode!(&bytes)?)
    }
    pub async fn update_post_increment_share_count(&self, arg0: u64) -> Result<u64> {
        let args = Encode!(&arg0)?;
        let bytes = self
            .1
            .update(&self.0, "update_post_increment_share_count")
            .with_arg(args)
            .call_and_wait()
            .await?;
        Ok(Decode!(&bytes, u64)?)
    }
    pub async fn update_post_toggle_like_status_by_caller(&self, arg0: u64) -> Result<bool> {
        let args = Encode!(&arg0)?;
        let bytes = self
            .1
            .update(&self.0, "update_post_toggle_like_status_by_caller")
            .with_arg(args)
            .call_and_wait()
            .await?;
        Ok(Decode!(&bytes, bool)?)
    }
    pub async fn update_principals_i_follow_toggle_list_with_principal_specified(
        &self,
        arg0: Principal,
    ) -> Result<Result3> {
        let args = Encode!(&arg0)?;
        let bytes = self
            .1
            .update(
                &self.0,
                "update_principals_i_follow_toggle_list_with_principal_specified",
            )
            .with_arg(args)
            .call_and_wait()
            .await?;
        Ok(Decode!(&bytes, Result3)?)
    }
    pub async fn update_principals_that_follow_me_toggle_list_with_specified_principal(
        &self,
        arg0: Principal,
    ) -> Result<Result4> {
        let args = Encode!(&arg0)?;
        let bytes = self
            .1
            .update(
                &self.0,
                "update_principals_that_follow_me_toggle_list_with_specified_principal",
            )
            .with_arg(args)
            .call_and_wait()
            .await?;
        Ok(Decode!(&bytes, Result4)?)
    }
    pub async fn update_profile_display_details(
        &self,
        arg0: UserProfileUpdateDetailsFromFrontend,
    ) -> Result<Result5> {
        let args = Encode!(&arg0)?;
        let bytes = self
            .1
            .update(&self.0, "update_profile_display_details")
            .with_arg(args)
            .call_and_wait()
            .await?;
        Ok(Decode!(&bytes, Result5)?)
    }
    pub async fn update_profile_resend_username_to_user_index_canister(&self) -> Result<Result6> {
        let args = Encode!()?;
        let bytes = self
            .1
            .update(
                &self.0,
                "update_profile_resend_username_to_user_index_canister",
            )
            .with_arg(args)
            .call_and_wait()
            .await?;
        Ok(Decode!(&bytes, Result6)?)
    }
    pub async fn update_profile_set_unique_username_once(&self, arg0: String) -> Result<Result6> {
        let args = Encode!(&arg0)?;
        let bytes = self
            .1
            .update(&self.0, "update_profile_set_unique_username_once")
            .with_arg(args)
            .call_and_wait()
            .await?;
        Ok(Decode!(&bytes, Result6)?)
    }
    pub async fn update_user_add_role(&self, arg0: UserAccessRole, arg1: Principal) -> Result<()> {
        let args = Encode!(&arg0, &arg1)?;
        let bytes = self
            .1
            .update(&self.0, "update_user_add_role")
            .with_arg(args)
            .call_and_wait()
            .await?;
        Ok(Decode!(&bytes)?)
    }
    pub async fn update_user_remove_role(
        &self,
        arg0: UserAccessRole,
        arg1: Principal,
    ) -> Result<()> {
        let args = Encode!(&arg0, &arg1)?;
        let bytes = self
            .1
            .update(&self.0, "update_user_remove_role")
            .with_arg(args)
            .call_and_wait()
            .await?;
        Ok(Decode!(&bytes)?)
    }
}
