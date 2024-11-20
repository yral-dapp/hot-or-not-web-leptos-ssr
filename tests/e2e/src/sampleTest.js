describe('Yral basic test', function () {
    it('should be able to like videos on the feed', () => {
        browser.url("https://yral.com");
        browser.element.findByText('Home Feed').waitUntil('visible')
        let likeButton = browser.element.find('div.snap-always:nth-child(2) > div:nth-child(1) > div:nth-child(2) > div:nth-child(2) > div:nth-child(3) > button:nth-child(1)')
        likeButton.waitUntil('enabled', { timeout: 10000 });
        likeButton.click();
        likeButton.waitUntil('enabled');
        expect(likeButton.find('img')).property('src').that.contains('heart-icon-liked')
        browser.pause(1000)
    })
})