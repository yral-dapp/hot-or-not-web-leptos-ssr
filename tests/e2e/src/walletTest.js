describe("wallet page tests", function () {
    before(function () {
        browser.url(`${browser.launchUrl}/wallet`)
    })

    it("wallet page contains login button", async function (browser) {
        browser.element.findByText('Login to claim your COYNs').assert.enabled()
    })
})