describe("wallet page tests", function () {
    before(function () {
        browser.url(`${browser.launchUrl}/wallet`)
    })

    it("wallet page contains login button", async function (browser) {
        browser.element.findByText('Login to claim your COYNs').waitUntil('enabled', {
            timeout: 10000
        });
    })
    
    it("default wallet page contains 1000 COYNS", async function(browser){
        browser.element.findByText("COYNS", {timeout: 10000}).waitUntil('enabled', {timeout: 10000})
        browser.element.findByText("1000", {timeout: 10000}).waitUntil('enabled', {timeout: 10000});
    })

    it('wallet page snapshot test', function(browser) {
        browser.percySnapshot('Wallet Page')
    })
})