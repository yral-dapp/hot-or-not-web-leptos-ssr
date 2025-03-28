describe("wallet page tests", function () {
    before(function () {
        browser.url(`${browser.launchUrl}/wallet`)
    })

    it("wallet page contains login button", async function (browser) {
        browser.waitForElementVisible('body', 10000);
        browser.pause(10000);

        browser.element.findByText('Login to claim', {timeout: 50000, exact: false }).waitUntil('enabled');
    })
    
    // TODO: update this test so that either 1000 COYNS are present or a 1000 CENTS, never both
    it("default wallet page contains 1000 COYNS or 2000 CENTS", function(browser) {
        browser.waitForElementVisible('body', 10000);
        browser.pause(10000);
        
        // First try to find CENTS
        browser.element.findByText("CENTS").isPresent(function(result) {
            if (result.value === true) {
                // CENTS exists, verify the 2000 amount
                console.log('CENTS found, checking for 2000 value');
                browser.element.findByText("2000").waitUntil('visible', { timeout: 10000 })
                    .assert.enabled()
                    .perform(function() {
                        console.log('Successfully verified 2000 CENTS');
                    });
            } else {
                // CENTS doesn't exist, check for COYNS instead
                console.log('CENTS not found, checking for COYNS and 1000 value');
                browser.element.findByText("COYNS").waitUntil('visible', { timeout: 10000 })
                    .assert.enabled()
                    .perform(function() {
                        browser.element.findByText("1000").waitUntil('visible', { timeout: 10000 })
                            .assert.enabled()
                            .perform(function() {
                                console.log('Successfully verified 1000 COYNS');
                            });
                    });
            }
        });
    });
    
    it('wallet page snapshot test', function(browser) {
        browser.percySnapshot('Wallet Page')

    })


    it("check usdc  loading", async function (browser){
        browser.url(`${browser.launchUrl}/wallet/34yzw-zrmgu-vg6ms-2uj2a-czql2-7y4bu-mt5so-ckrtz-znelw-yyvr4-2ae`);
        browser.element.findByText('USDC', {timeout: 20000}).waitUntil('visible', { timeout: 10000 }).assert.enabled()
    })
})
