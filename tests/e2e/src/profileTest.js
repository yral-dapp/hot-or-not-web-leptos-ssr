describe('Profile page tests', function () {
    before(async function(browser, done) {
        await browser.url(`${browser.launchUrl}/profile/tokens`);
        await browser.waitForElementVisible('body', 10000);
        done();
    });

    it('profile page', async function (browser) {
        await browser.waitForElementVisible('body', 10000);
        await browser.element.findByText('Login', {timeout: 15000}).waitUntil('visible').click();
        await browser.pause(1000);
        await browser.element.findByText('Google Sign-In', {timeout: 15000}).waitUntil('visible');
        await browser.pause(2000);
        await browser.percySnapshot('SignIn Modal');
    });
});