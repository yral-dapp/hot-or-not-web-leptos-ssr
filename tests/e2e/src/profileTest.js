

describe('Profile page tests', function () {

    before(function () {

        browser.url(`${browser.launchUrl}/profile/tokens`)
    })


    it('profile page', async function (browser) {
        browser.element.findByText('Login').waitUntil('visible', { timeout: 50000 }).click()
        browser.element.findByText('Google Sign-In').waitUntil('visible')
        browser.percySnapshot('SignIn Modal')
    })


})