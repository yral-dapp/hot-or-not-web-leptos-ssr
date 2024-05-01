import { chromium } from 'playwright';
import { test, expect } from '@playwright/test';

// test.beforeEach(async ({ page }) => {
//   await page.goto('https://yral.com/');
// });

test.describe('New test', () => {
 //test like functionality
 test('Test refferal', async () => {

    // login from wallet: using headfull mode
    // try {
      
      // const environment_pass = global.expect;
      // login for first user
      const environment_pass = process.env.TESTPARAM; 

      const browser1 = await chromium.launch(); // Launch Chromium browser
      const context = await browser1.newContext();
      const page = await context.newPage();
      await page.goto('https://yral.com/');

      await page.waitForTimeout(3000);

      await page.getByRole('navigation').getByRole('link').nth(4).click();
      await page.getByRole('button', { name: 'Login' }).click();
      const page1Promise = page.waitForEvent('popup');
      await page.getByRole('button', { name: 'Google Sign-In' }).click();
      const page1 = await page1Promise;
      await page.waitForTimeout(2000);

      await page1.getByLabel('Email or phone').click();
      await page1.getByLabel('Email or phone').fill('testautomationyral@gmail.com');
      await page1.getByLabel('Email or phone').press('Enter');
      
      await page1.getByLabel('Enter your password').click();
      await page1.getByLabel('Enter your password').fill(environment_pass);
      await page1.getByLabel('Enter your password').press('Enter');
      // await page1.pause();
      await page.waitForTimeout(3000);

    // store current users COYN balance
    //   await page.getByRole('link', { name: 'View Profile' }).click();
    //   await page.waitForTimeout(1000);

    //   let earnings = page.getByText('Earnings');
    //   let user_earnings = await earnings.textContent();

    //   console.log('user earnings : ', user_earnings);
      console.log('Log in test running');

      await page.getByRole('navigation').getByRole('link').first().click();
      // click on refferal button
      await page.locator('div:nth-child(2) > a').first().click();
      await page.waitForTimeout(3000);

      let refferal_link_lo = page.getByText('gzlng-jqzta-5kubz-4nyam-5so2e');
      let refferal_code = await refferal_link_lo.textContent();
      console.log('refferal code : ', refferal_code);

      let refferal_link = "https://yral.com/?user_refer="+refferal_code;
      console.log('refferal link : ', refferal_link);

    // login for second user
      const browser2 = await chromium.launch(); // Launch Chromium browser
      const new_context = await browser2.newContext();
      const page2 = await new_context.newPage();
      await page2.goto(refferal_link);

      const title = await page2.title();
      console.log(title);
      expect(title).toBe("Yral");

      console.log('reffered login ');

      // Todo: remove
      page2.close();
      await page.waitForTimeout(1000);
      await page.getByRole('navigation').getByRole('link').nth(4).click();
      await page.waitForTimeout(2000);
      
    // uncomment when adding 2nd user
    //   await page2.getByRole('navigation').getByRole('link').nth(4).click();
    //   await page2.getByRole('button', { name: 'Login' }).click();
    //   const page3Promise = page2.waitForEvent('popup');
    //   await page2.getByRole('button', { name: 'Google Sign-In' }).click();
    //   const page3 = await page3Promise;
    //   await page.waitForTimeout(2000);

    //   await page3.getByLabel('Email or phone').click();
    //   await page3.getByLabel('Email or phone').fill('');
      
    //   await page3.getByLabel('Email or phone').press('Enter');
      
    //   await page3.getByLabel('Enter your password').click();
    //   await page3.getByLabel('Enter your password').fill("");

    //   await page3.getByLabel('Enter your password').press('Enter');

    //   await page2.getByRole('link', { name: 'View Profile' }).click();
    //   await page2.waitForTimeout(1000);

    //   let earnings = page2.getByText('Earnings');
    //   let user_earnings = await earnings.textContent();

    //   console.log('user earnings : ', user_earnings);
    //   await page.getByRole('navigation').getByRole('link').first().click();

    //   await page.waitForTimeout(3000);
    //     //request to the API endpoint to fetch the response
    //   const response = await fetch('https://yral-metadata.fly.dev/metadata/gzlng-jqzta-5kubz-4nyam-5so2e-tsoio-ijv2s-47dsw-7ksd7-pe3eb-zqe');
    //   await page.waitForTimeout(2000);

    //   console.log(response.status);
    //   const responseData = await response.json();
    //   console.log(responseData);

    // } catch (error) {
    //   console.error("error in login"); 
    // }
  });

})
//css locator for earnings:      
//   let earnings = page.locator('body > main > div > div.grid.grid-cols-1.gap-5.justify-normal.justify-items-center.w-full > div.flex.flex-row.w-11\/12.sm\:w-7\/12.justify-center > div > div > div > p.text-primary-500');