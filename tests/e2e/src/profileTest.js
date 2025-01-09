describe('Profile page tests', function () {
    before(function () {
        browser.url(`${browser.launchUrl}/profile/tokens`)
        // Add wait for initial page load
        browser.waitForElementVisible('body', 10000);
    })

    it('profile page', async function (browser) {
        // Add more robust waits
        browser.waitForElementVisible('body', 10000);
        browser.element.findByText('Login', {timeout: 15000}).waitUntil('visible').click()
        // Add pause after click
        browser.pause(1000);
        browser.element.findByText('Google Sign-In', {timeout: 15000}).waitUntil('visible')
        // Add wait before snapshot
        browser.pause(2000);
        browser.percySnapshot('SignIn Modal')
    })
})