describe("wallet page tests", function () {
    before(function () {
        browser.url(`${browser.launchUrl}/wallet`)
    })

    it("wallet page contains login button", async function (browser) {
        browser.element.findByText('Login to claim your COYNs').assert.enabled()
    })

    it("default wallet page contains 1000 COYNS", async function(browser){
        browser.useXpath().assert.containsText("/html/body/main/div/div[3]/div/div[2]/div[1]/div[1]/div[2]/div[1]", "1000");
        browser.useXpath().assert.containsText("/html/body/main/div/div[3]/div/div[2]/div[1]/div[1]/div[2]/div[2]", "COYNS");
    })
})