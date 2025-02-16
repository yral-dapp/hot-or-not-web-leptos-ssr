describe("Token Creation Tests", function () {
    before(function () {
        browser.url(`${browser.launchUrl}/token/create/settings`);
    })
    it("Advanced settings are read-only", browser => {
        browser.pause(5000); // let the browser load the page

        browser.assert.not.elementPresent("div#advanced-settings > input", "There should be no input under advanced settings");
    })
})

