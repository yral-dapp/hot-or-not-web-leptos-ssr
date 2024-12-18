describe("Menu page test", function () {
    before(function () {
        browser.url(`${browser.launchUrl}/menu`)
    });

    it('menu page has option to enable notification', function (browser) {

        browser.element.findByText('Menu').waitUntil('visible')
        browser.element.findByText("Login").waitUntil("enabled")
        browser.perform(function () {
            const actions = this.actions({ async: true })

            return actions.keyDown(this.Keys.ARROW_DOWN)
                .press()
                .release()
        })

        let settingsOptions = locateWith(browser.element.findByText("Settings")).toRightOf(browser.element("svg"));
        browser.click(settingsOptions)

        browser.element.findByText("Enable Notifications").assert.visible()

    })
})