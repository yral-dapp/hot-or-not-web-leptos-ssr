use leptos::*;

#[component]
pub fn PrivacyPolicy() -> impl IntoView {
    view! {
        <div class="w-screen min-h-screen px-8 bg-black py-4 text-white flex flex-col items-center">
            <span class="font-bold">Privacy Policy</span>
            <div class="flex h-full w-full flex-col space-y-8 overflow-hidden overflow-y-auto py-16">
                <div class="text-xs">
                    {r#"Thanks for choosing "Hot or Not" . We are pleased to make our Service
                    available to you. Hot or Not includes the Hot or Not App, website,
                    features, and other associated services. Hot or Not provides Image
                    Posting, Video Posting and other services to users. Hot or Not includes
                    social and interactive features. This is our Privacy Policy, which helps
                    explain our information practices. If you install, use, access Hot or
                    Not or any of its features, you have read and accepted the terms of this
                    Policy. You have also accepted the Hot or Not 'Terms of Service',
                    'Community Guidelines' and other terms which are available on the Hot or
                    Not App, Hot or Not Website, and otherwise. Please make sure to read
                    these documents. By using Hot or Not, you are entering into a binding
                    contract(s) with Hot or Not. Your contract with us includes this Policy,
                    the Terms and Conditions, the Community Guidelines, etc. If you don't
                    agree with and accept the above, please do not use, install, access Hot
                    or Not or any of its features. This Policy is applicable to information
                    provided by users while using Hot or Not, and not to information made
                    available to Hot or Not from others. Should Hot or Not obtain
                    information pertaining to you from websites, apps, landing pages, third
                    parties, etc. that may be accessed by Hot or Not users through Hot or
                    Not or otherwise, such information will be subject to the privacy and
                    other policies and terms of the said websites, apps, landing pages, etc.
                    This Policy shall not apply to websites, apps, landing pages, etc. or
                    any third-party services accessed by you through Hot or Not including in
                    case such websites, apps, landing pages, etc. and/or third-party
                    services are advertised on Hot or Not. You also warrant and represent
                    that any registration information and other information that you submit
                    or otherwise provide to Hot or Not is true, accurate and complete, and
                    you agree to keep it that way at all times."#}
                </div>
                <div class="flex flex-col space-y-2">
                    <div class="text-sm">1. Information We Collect</div>
                    <div class="text-xs">
                        The below information is collected by Hot or Not:
                        <ul class="list-decimal pl-6">
                            <li>
                                Information collected with a prompt: <ul class="list-disc pl-4">
                                    <li>
                                        In order to use Hot or Not, we may need you to authenticate
                                        your account using your internet identity, or otherwise, and
                                        we may collect your email address and name. We may also ask
                                        you to provide certain information including age, gender,
                                        telephone number, your content/category preferences, location,
                                        areas of interest, language, region preferences etc.
                                    </li>
                                    <li>
                                        {r#"We may ask you for the location information and regional
                                        preferences to customise your experience by showing you
                                        relevant content. Hot or Not may collect and store the current
                                        and past location of your device if you permit. You have full
                                        control over whether to allow Hot or Not to collect this
                                        information by making changes to your device’s settings. If
                                        Hot or Not is unable to access your location information, Hot
                                        or Not may become inaccessible or not function as intended."#}
                                    </li>
                                    <li>
                                        Customer Support. You may also provide us with information
                                        related to your use of Hot or Not, including copies of your
                                        messages / posts, and how to contact you so that we can
                                        provide you with customer support. For example, you may send
                                        us an email with information relating to our app performance,
                                        or other issues.
                                    </li>
                                </ul>
                            </li>
                            <li>
                                Information collected without a prompt: <ul class="list-disc pl-4">
                                    <li>
                                        Device Information: We may collect the below with respect to
                                        any device through which Hot or Not is used or accessed:
                                        <ul class="list-disc pl-4">
                                            <li>Device Identifiers.</li>
                                            <li>IP Address.</li>
                                            <li>Operating System.</li>
                                            <li>Operating System Version.</li>
                                            <li>Advertising Identifiers.</li>
                                            <li>Device Make.</li>
                                            <li>Display Features.</li>
                                            <li>Network Provider/Wi-Fi or other ISP.</li>
                                            <li>Battery level and usage.</li>
                                            <li>Memory usage.</li>
                                            <li>
                                                Other apps or services installed or used through the
                                                device.
                                            </li>
                                            <li>
                                                Content Type and Non-Personal information related to ads.
                                            </li>
                                            <li>Profile Information.</li>
                                        </ul>
                                    </li>
                                    <li>
                                        Information related to your use of Hot or Not such as content
                                        preferences and consumption, interaction with other users,
                                        user activity, etc.
                                    </li>
                                    <li>
                                        {r#"Hot or Not may access your device’s camera, microphone, photo
                                        and/or video and/or audio library or gallery."#}
                                    </li>
                                    <li>
                                        Hot or Not may also collect information provided by third
                                        parties who may disclose to Hot or Not personal or
                                        non-personal information collected by them. Hot or Not has no
                                        control over or responsibility in respect of third-party
                                        information practices or information collection methods.
                                    </li>
                                    <li>
                                        Hot or Not may also collect tracking information including
                                        cookies, DART, beacons, etc.
                                    </li>
                                    <li>
                                        Usage and Log Information. We collect service-related,
                                        diagnostic, and performance information. This includes
                                        information about your activity (such as how you use Hot or
                                        Not, how you interact with others using Hot or Not, etc.), log
                                        files, and diagnostics, crash information, website, and
                                        performance logs and reports, etc.
                                    </li>
                                </ul>
                            </li>
                        </ul>
                    </div>
                </div>
                <div class="flex flex-col space-y-2">
                    <div class="text-sm">2. Information We Do Not Collect</div>
                    <div class="text-xs">
                        We do not collect any Sensitive Personal Data or information such as
                        password related to other services, financial information such as bank
                        account or credit card or debit card or other payment instrument
                        details, physical, physiological and mental health condition, sexual
                        orientation, medical records and history, or biometric information.
                    </div>
                </div>
                <div class="flex flex-col space-y-2">
                    <div class="text-sm">3. How We Use Information</div>
                    <div class="text-xs">
                        We may use all the information we have to help us operate, provide,
                        improve, understand, customise, support, and market Hot or Not.
                        <ul class="list-decimal pl-6">
                            <li>
                                Our Services: We operate and provide Hot or Not, and improving,
                                fixing, and customising Hot or Not by using the said information.
                                We understand how users use Hot or Not, and analyse and use the
                                information that we have to evaluate and improve Hot or Not,
                                research, develop, and test new services and features, and conduct
                                troubleshooting activities. We also use your information to
                                respond to you when you contact us. We may also use your
                                information for (i) displaying content based on interest,
                                location, offers, etc., (ii) displaying or providing access to
                                promotions, advertisements, offer, etc. which may be based on your
                                interests and also ad-targeting, ad-placement, and ad-measurement
                                (iii) displaying location specific news and weather related
                                information, (iv) improving search results, content loading,
                                performance, etc.
                            </li>
                            <li>
                                Safety and Security: We also use your information to try and
                                ensure safety and security of our users and their information. We
                                verify accounts and activity, and promote safety and security on
                                and off Hot or Not, such as by investigating suspicious activity
                                or violations of our policies, terms, etc., and to try and ensure
                                that Hot or Not is being used legally.
                            </li>
                        </ul>
                    </div>
                </div>
                <div class="flex flex-col space-y-2">
                    <div class="text-sm">4. How We Share Information</div>
                    <div class="text-xs">
                        We may share your information in ways including the following:
                        <ul class="list-decimal pl-6">
                            <li>
                                With other Hot or Not users: your username, profile photograph,
                                content that you post or send or share. You may also be able to
                                control how your content is shared with other Hot or Not users by
                                adjusting personal settings.
                            </li>
                            <li>
                                With our business partners, affiliates, investors: public
                                information like your name, username and profile pictures and any
                                content posted by you.
                            </li>
                            <li>
                                We may share information about you with third party service
                                providers who perform services on our behalf, including to measure
                                and optimise the performance of ads and deliver more relevant ads,
                                including on third-party websites and apps.
                            </li>
                            <li>
                                We may share information about you, such as device and usage
                                information, to help us and others prevent fraud.
                            </li>
                            <li>
                                We may share information about you for legal, safety, and security
                                reasons.
                            </li>
                            <li>
                                We may share information about you if we reasonably believe that
                                disclosing the information is needed to: (i) comply with any valid
                                legal process, governmental request, or applicable law, rule, or
                                regulation, (ii) investigate, remedy, or enforce potential
                                violations of this Policy, the Terms of Service and the Community
                                Guidelines.
                            </li>
                            <li>
                                To protect the rights, property, or safety of us, our users, or
                                others.
                            </li>
                            <li>To detect and resolve any fraud or security concerns.</li>
                        </ul>
                    </div>
                </div>
                <div class="flex flex-col space-y-2">
                    <div class="text-sm">5. Security</div>
                    <div class="text-xs">
                        We undertake no responsibility for the security or safety of your
                        information. However, we take reasonable security measures to protect
                        your information in accordance with industry standards. This does not
                        guarantee safety or security of your information as events beyond our
                        control can also occur.
                    </div>
                </div>
                <div class="flex flex-col space-y-2">
                    <div class="text-sm">6. Assignment, Change Of Control, And Transfer</div>
                    <div class="text-xs">
                        All of our rights and obligations under this Policy, the Terms of
                        Service and the Community Guidelines are freely assignable by us to
                        any of our affiliates or any others, in connection with a merger,
                        acquisition, restructuring, or sale of assets, or by operation of law
                        or otherwise, and we may transfer your information to any of our
                        affiliates, any others successor entities, or new management, etc.
                    </div>
                </div>
                <div class="flex flex-col space-y-2">
                    <div class="text-sm">7. Law And Protection</div>
                    <div class="text-xs">
                        We may collect, use, preserve, and share your information if we
                        believe that it may be reasonably necessary to: (a) respond pursuant
                        to applicable law or regulations, to legal process, or to government
                        requests; (i) comply with any valid legal process, governmental
                        request, or applicable law, rule, or regulation, (ii) investigate,
                        remedy, or enforce potential violations of this Policy, the Terms of
                        Service and Community Guidelines, (iii) to protect the rights,
                        property, or safety of us, our users, or others, (iv) to detect and
                        resolve any fraud or security concerns or technical issues, etc., (iv)
                        for other legal requirements, etc.
                    </div>
                </div>
                <div class="flex flex-col space-y-2">
                    <div class="text-sm">8. Updates To Our Policy</div>
                    <div class="text-xs">
                        {r#"We may amend or update this Policy. We will provide you notice of
                        amendments to this Policy, as appropriate, and update the “Last
                        Modified” date at the top of this Policy. Your continued use of Hot or
                        Not confirms your acceptance of this Policy, as amended. If you do not
                        agree with or accept this Policy, as amended, you must stop using Hot
                        or Not. Please review our Privacy Policy from time to time.
                        <div>Contact Us</div>
                        If you have questions about this Policy, email at : utkarsh@gobazzinga.io"#}
                    </div>
                </div>
                <div class="flex flex-col space-y-2">
                    <div class="text-sm">9. Governing Law and Jurisdiction</div>
                    <div class="text-xs">
                        The validity, construction and enforceability of these Terms and
                        Content Policy, the Privacy Policy shall be governed and construed in
                        accordance with the laws of the state of Delaware, United States
                        ofAmerica.
                    </div>
                </div>
                <div class="flex flex-col space-y-2">
                    <div class="text-sm">10. Withdrawal of consent</div>
                    <div class="text-xs">
                        You also have the right to withdraw your consent at any time and
                        request us to stop processing your data. However, withdrawing your
                        consent will not affect the processing or use of your information
                        prior to fulfilling your consent withdrawal request. You can email us
                        with your registered email address.
                    </div>
                </div>
                <div class="flex flex-col space-y-2">
                    <div class="text-sm">11. Right to opt out</div>
                    <div class="text-xs">
                        {r#"You can opt out from our use of data from cookies and similar
                        technologies that track your behaviour on the sites of others for ad
                        targeting and other ad-related purposes. You can also opt out of
                        marketing communication. We will ask you to opt-in before we use GPS
                        or other tools to identify your precise location. However, you will
                        continue to receive important emails relating to your account. You can
                        email us with your registered email address to unsubscribe from such
                        communications."#}
                    </div>
                </div>
                <div class="flex flex-col space-y-2">
                    <div class="text-sm">12. Third party Links</div>
                    <div class="text-xs">
                        We are not responsible for any content nor any consequences either
                        directly or indirectly arising out of clicking on any third party link
                        you may come across posted by other users. We may share your
                        information with approved third parties in order to provide our
                        services through the Hot or Not App
                    </div>
                </div>
                <div class="flex flex-col space-y-2">
                    <div class="text-sm">13. Data Transfers</div>
                    <div class="text-xs">
                        We may transfer personal information to countries other than the
                        country in which the data was originally collected.
                    </div>
                </div>
                <div class="flex flex-col space-y-2">
                    <div class="text-sm">14. iOS Users</div>
                    <div class="text-xs">
                        Collection, Use, Disclosure, Sharing, and Retention of Face Data:
                        <ul class="list-decimal pl-6">
                            <li>
                                Collection of Face Data: Information to identify face & facial
                                expressions.
                            </li>
                            <li>
                                {r#"Usage of Face Data: To make Hot or Not app work well for their
                                users, we offer tools that apply filters and video effects fitting
                                the contours of the user's face, modifying the size and shape of
                                facial features and following facial movements."#}
                            </li>
                            <li>
                                {r#"Disclosure of Face Data: This data is used to create filters and
                                video effects within the App. The only use of this information is
                                to provide valuable user features. None of the information
                                collected ever leaves the user’s device."#}
                            </li>
                            <li>
                                Sharing of Face Data: We do not share this data with third
                                parties.
                            </li>
                            <li>
                                {r#"Retention of Face Data: We do not store any Facial Recognition
                                Data. Also none of the information collected by Face Data ever
                                leaves the user’s device. However few data like user registration
                                data, video recorded by users and user’s profile photo are stored
                                on our servers."#}
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    }
}
