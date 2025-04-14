use component::{back_btn::BackButton, title::TitleText};
use leptos::prelude::*;
use leptos_meta::*;
use state::app_state::AppState;

#[component]
pub fn TermsIos() -> impl IntoView {
    let app_state = use_context::<AppState>();
    let page_title = app_state.unwrap().name.to_owned() + " - Terms & Conditions for iOS";
    view! {
        <Title text=page_title />
        <div class="w-screen min-h-screen bg-black pt-4 pb-12 text-white flex flex-col items-center">
            <TitleText justify_center=false>
                <div class="flex flex-row justify-between">
                    <BackButton fallback="/menu".to_string() />
                    <span class="font-bold">Terms & Conditions for iOS</span>
                    <div></div>
                </div>
            </TitleText>
            <div class="px-8 flex h-full w-full flex-col space-y-8 overflow-hidden overflow-y-auto py-16">
                <div class="flex flex-col space-y-2">
                    <div class="text-sm font-semibold">Effective Date</div>
                    <div class="text-xs">13th July 2023</div>
                </div>
                <div class="flex flex-col space-y-2">
                    <div class="text-sm font-semibold">Last Updated</div>
                    <div class="text-xs">11th April 2025</div>
                </div>
                <div class="flex flex-col space-y-2">
                    <div class="text-sm font-semibold">Welcome to Yral</div>
                    <div class="text-xs whitespace-pre-line">
                        {r#"Welcome to Yral, a community-driven platform where users share and discover short videos.
                        These Terms of Use (\"Terms\") govern your use of the Yral mobile and web application (\"App\", \"Services\"), operated by HotorNot (HON) Gmbh (\"Company\", \"we,\" \"our,\" or \"us\"). By using the App, you agree to these Terms. If you do not agree, please do not use the App."#}
                    </div>
                </div>
                <div class="flex flex-col space-y-2">
                    <div class="text-sm font-semibold">Your Account & Registration</div>
                    <div class="text-xs whitespace-pre-line">
                        {r#"You must be at least 13 years old to use the App and you must be represented by a legal guardian if you are below the legal age in your respective jurisdiction to register or use the App. You represent and warrant that you will provide accurate and up-to-date information while creating an account with us and you agree to keep the information accurate at all times. You must keep your account password confidential. If at any time, if you fail to comply with the provision of these terms or if activities occur on your account which might cause damage to the Services or infringe or violate any third-party rights or violate any laws or regulations, or for any other reason, we reserve the right to, at our discretion, disable your account and remove or disable any content you upload or share."#}
                    </div>
                </div>
                <div class="flex flex-col space-y-2">
                    <div class="text-sm font-semibold">Community Guidelines on Yral Content</div>
                    <div class="text-xs whitespace-pre-line">
                        {r#"Yral is a platform for user-generated video content. By posting any content on or through our Services, you hereby grant us a non-exclusive, fully paid and royalty-free, worldwide, limited licence to display such content as felt appropriate, in any media formats through any media channels, and delete such content solely for the purpose of operating and improving the Yral platform."#}
                    </div>
                </div>
                <div class="flex flex-col space-y-2">
                    <div class="text-sm font-semibold">Community Guidelines</div>
                    <div class="text-xs whitespace-pre-line">
                        {r#"You agree to follow Yral's Community Guidelines, which prohibit any sort of objectionable content that is:
                        Defamatory, discriminatory, or mean-spirited content, including references or commentary about religion, race, sexual orientation, gender, national/ethnic origin, or other targeted groups—particularly if the app could humiliate, intimidate, or harm a targeted individual or group. Professional political satirists and humorists are exempt from this requirement."#}
                    </div>
                </div>
                <div class="flex flex-col space-y-2">
                    <div class="text-sm font-semibold">Content Moderation & Safety</div>
                    <div class="text-xs whitespace-pre-line">
                        {r#"Yral uses a mix of AI enabled automated filters and human moderation to detect and manage objectionable content. Our automated filters automatically delete Not Safe For Work (NSFW) content containing nudity, pornography, or other harmful content that violates our community guidelines. Further, users have the ability to report NSFW content while using the App. Users are encouraged to report NSFW content that our automated filters miss. Such reports are prioritized by our moderation team."#}
                    </div>
                </div>
                <div class="flex flex-col space-y-2">
                    <div class="text-sm font-semibold">NSFW Content Policy</div>
                    <div class="text-xs whitespace-pre-line">
                        {r#"By enabling the NSFW (Not Safe for Work) content toggle, you acknowledge and consent to viewing adult-oriented content within your feed. This content may include themes that are intended for mature audiences. You agree that:
                        ● You are of legal age to view such content as per the laws and regulations of your jurisdiction.
                        ● The platform is not responsible for any distress or offense caused by NSFW content.
                        ● You may disable NSFW content at any time by turning off the toggle in your settings."#}
                    </div>
                </div>
                <div class="flex flex-col space-y-2">
                    <div class="text-sm font-semibold">Use License (EULA Equivalent)</div>
                    <div class="text-xs whitespace-pre-line">
                        {r#"We grant you a limited, non-exclusive, non-transferable, revocable license to use the Yral mobile and web application (the \"App\") solely for your personal, non-commercial use on any Apple-branded device that you own or control, and as permitted by the Usage Rules set forth in the Apple App Store Terms of Service."#}
                    </div>
                </div>
                <div class="flex flex-col space-y-2">
                    <div class="text-sm font-semibold">Content Removal and Disabling or Terminating Your Account</div>
                    <div class="text-xs whitespace-pre-line">
                        {r#"We may remove any content, suspend or terminate your account without notice if:
                        ● You breach these Terms
                        ● You violate laws or community accepted standards
                        ● Your behaviour puts other users at risk
                        You may delete your account at any time via the app’s settings."#}
                    </div>
                </div>
                <div class="flex flex-col space-y-2">
                    <div class="text-sm font-semibold">Privacy Policy</div>
                    <div class="text-xs whitespace-pre-line">
                        {r#"We take your privacy seriously. Please refer to Privacy Policy for detailed information on the data we collect, how we use it, your rights, and how to exercise them. You can access the full policy from the menu."#}
                    </div>
                </div>
                <div class="flex flex-col space-y-2">
                    <div class="text-sm font-semibold">Disclaimers</div>
                    <div class="text-xs whitespace-pre-line">
                        {r#"To the maximum extent permitted by applicable law, the App is licensed ‘as is,’ and you use it at your sole risk. The developer disclaims all warranties, express or implied. We do not guarantee:
                        ● The accuracy or reliability of content. All opinions expressed belong solely to the users
                        ● Uninterrupted or error-free service.
                        ● That content will remain available or accessible.
                        Use Yral at your own risk."#}
                    </div>
                </div>
                <div class="flex flex-col space-y-2">
                    <div class="text-sm font-semibold">Limitation of Liability</div>
                    <div class="text-xs whitespace-pre-line">
                        {r#"We shall not be liable to you for any loss or damages or claims including but not limited to the following:
                        Any loss of profit, opportunity or goodwill;
                        Any loss of data;
                        Any damage incurred as a result of your reliance on any advertisement appearing on the Service;
                        Any damage incurred as a result of any changes to the Service, its features and any temporary or permanent termination of our Services;
                        Any damage incurred as a result of your failure to provide us with accurate information and your failure to keep your account details and password confidential and secure;
                        Any damage incurred as a result of the actions of another user."#}
                    </div>
                </div>
                <div class="flex flex-col space-y-2">
                    <div class="text-sm font-semibold">Changes to These Terms</div>
                    <div class="text-xs whitespace-pre-line">
                        {r#"We may update these Terms from time to time. We’ll notify you of major changes through the App or email. Continued use of the App after changes means you accept the revised Terms."#}
                    </div>
                </div>
                <div class="flex flex-col space-y-2">
                    <div class="text-sm font-semibold">Contact Us</div>
                    <div class="text-xs">
                        Have questions or concerns? <br />
                        📧 Email us at: support@yral.com
                    </div>
                </div>
            </div>
        </div>
    }
}