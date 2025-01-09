describe("wallet page tests", function () {
    before(function () {
        browser.url(`${browser.launchUrl}/wallet`)
    })

    it("wallet page contains login button", async function (browser) {
        // Add explicit wait for page load
        browser.waitForElementVisible('body', 10000);
        browser.element.findByText('Login to claim your COYNs', {timeout: 10000}).waitUntil('enabled');
    })
    
    it("default wallet page contains 1000 COYNS", async function(browser){
        browser.waitForElementVisible('body', 10000);
        browser.element.findByText("COYNS", {timeout: 15000}).waitUntil('enabled')
        browser.element.findByText("1000", {timeout: 15000}).waitUntil('enabled');
    })

    it('wallet page snapshot test', function(browser) {
        browser.pause(2000);
        browser.percySnapshot('Wallet Page')
    })


    it("check usdc  loading", async function (browser){
        browser.url(`${browser.launchUrl}/wallet/34yzw-zrmgu-vg6ms-2uj2a-czql2-7y4bu-mt5so-ckrtz-znelw-yyvr4-2ae`);
        browser.element.findByText('USDC', {timeout: 30000}).waitUntil('visible', { timeout: 20000 }).assert.enabled()
    })
})