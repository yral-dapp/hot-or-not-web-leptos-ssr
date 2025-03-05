describe("Token Creation Tests", function () {
    before(function () {
        browser.url(`${browser.launchUrl}/token/create/settings`);
    })
    it("Advanced settings are read-only", browser => {
        browser.waitForElementVisible("div#advanced-settings", { timeout: 5000 });

        browser.assert.not.elementPresent("div#advanced-settings > input", "There should be no input under advanced settings");
    })
})

