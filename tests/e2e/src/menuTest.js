describe("Menu page test", function () {
    before(async function () {
        await browser.url(`${browser.launchUrl}/menu`)
    });


    it('menu page has option to enable notification', async function (browser) {

        browser.element.findByText('Menu').waitUntil('visible')
        browser.element.findByText("Login").waitUntil("enabled")
        browser.percySnapshot('Menu Page');
        let settingsRow = browser.element.findByText("Settings");
        let scrollY = (await settingsRow.getRect()).y;

        browser.perform(function () {
            const actions = this.actions({ async: true })

            return actions.scroll(0, 0, 0, Math.ceil(scrollY))

        })



        let settingsOptions = locateWith(settingsRow).toRightOf(browser.element("svg"));
        browser.click(settingsOptions)

        browser.element.findByText("Enable Notifications").assert.visible()

    })
})