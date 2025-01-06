describe("wallet page tests", function () {
    before(function () {
        browser.url(`${browser.launchUrl}/wallet`)
    })

    it("wallet page contains login button", async function (browser) {
        browser.element.findByText('Login to claim your COYNs').assert.enabled()
    })

    it("default wallet page contains 1000 COYNS", async function(browser){
        browser.element.findByText("COYNS").assert.enabled();
        browser.element.findByText("1000").assert.enabled();
    })

    it("check ckusdc  loading", async function (browser){
        browser.url(`${browser.launchUrl}/wallet/34yzw-zrmgu-vg6ms-2uj2a-czql2-7y4bu-mt5so-ckrtz-znelw-yyvr4-2ae`);

        browser.element.findByText('USDC').waitUntil('visible');
    })
})