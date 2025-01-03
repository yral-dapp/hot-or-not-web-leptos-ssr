describe('Landing Page Count', function () {

    before(function () {

        browser.url(`${browser.launchUrl}/board`)
    })



    it('profile page', async function (browser) {
        const targetDivSelector = 'div.w-8.h-8.rounded-lg.flex.items-center.justify-center.text-white.bg-blue-500';
        
        await browser.execute(() => {
            window.scrollTo(0, document.body.scrollHeight);
          });
          console.log('Scrolled to the bottom of the page.');
    
          await browser.pause(1000);
    
          await browser.waitForElementVisible(targetDivSelector, 5000, `Element ${targetDivSelector} is visible`);
          console.log(`Element ${targetDivSelector} is visible.`);
    
          const result = await browser.getText(targetDivSelector);
          const numberText = result.value.trim();
          const number = parseInt(numberText, 10);
    
          if (!isNaN(number)) {
            console.log(`Number found in ${targetDivSelector}:`, number);
            await browser.assert.ok(number > 0, 'Number is greater than zero');
          } else {
            console.error(`The content of ${targetDivSelector} is not a valid number:`, numberText);
            await browser.assert.fail(`Invalid number in ${targetDivSelector}: ${numberText}`);
          }
        
    })


})