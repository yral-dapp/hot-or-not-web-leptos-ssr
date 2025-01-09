describe("wallet page tests", function () {
    before(async function(browser, done) {
        await browser.url(`${browser.launchUrl}/wallet`);
        await browser.waitForElementVisible('body', 10000);
        done();
    });

    it("wallet page contains login button", async function (browser) {
        await browser.element.findByText('Login to claim your COYNs', {timeout: 15000}).waitUntil('visible');
    });
    
    it("default wallet page contains 1000 COYNS", async function(browser) {
        await browser.waitForElementVisible('body', 10000);
        await browser.element.findByText("COYNS", {timeout: 15000}).waitUntil('visible').assert.enabled();
        await browser.element.findByText("1000", {timeout: 15000}).waitUntil('visible').assert.enabled();
    });

    it('wallet page snapshot test', async function(browser) {
        await browser.pause(2000);
        await browser.percySnapshot('Wallet Page');
    });

    it("check usdc loading", async function (browser) {
        await browser.url(`${browser.launchUrl}/wallet/34yzw-zrmgu-vg6ms-2uj2a-czql2-7y4bu-mt5so-ckrtz-znelw-yyvr4-2ae`);
        await browser.element.findByText('USDC', {timeout: 30000}).waitUntil('visible', { timeout: 20000 }).assert.enabled();
    });
});