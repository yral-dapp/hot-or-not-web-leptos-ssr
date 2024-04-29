import { chromium } from 'playwright';
import { test, expect } from '@playwright/test';
import nock from 'nock';

test.beforeEach(async ({ page }) => {
  await page.goto('https://yral.com/');
});

test.describe('New test', () => {
  test('Has title', async ({page}) => {

    try {    // Check opening status
      const title = await page.title();
      console.log(title); // Log the page title
      // expect(title).toContain('Expected Title'); // Replace with expected title
      expect(title).toBe("Yral");
    } catch (error) {
          console.error("error checking title",error); // Handle errors gracefully
    }
    
  });

  test('Test video playing', async ({page}) => {

    //try with 'video' element
    let video = page.locator('video').nth(0);
    await page.waitForTimeout(3000);

    await expect(video).toBeVisible();
    console.log("is visible");

    await expect(video).not.toHaveAttribute('paused');
    console.log("is not paused");

    await expect(video).toHaveAttribute('muted'); 
    console.log("is muted");

    // click unmute button
    await page.locator('.fixed').first().click();
    await page.waitForTimeout(1000);
    //  // css selector of unmute button
    // const buttonLocator = page.locator('body > main > div.h-full.w-full.overflow-hidden.overflow-y-auto > div > button > svg > path').click();
    // await expect(video).toHaveAttribute('muted'); 
    // console.log("is muted");

    let video_new = page.locator('video').nth(0);
    // await expect(video_new).not.toHaveAttribute('muted'); 
    // console.log("is not muted");

    const duration = await video.evaluate(videoEle => videoEle.duration);
    console.log('Video Duration:', duration);

    // const isVideoPlaying = await video.evaluate(() => document.querySelector('video').autoplay); // Check playback state
    // console.log("is video playing", isVideoPlaying);

    //check if video is playing after 5 seconds
    await page.waitForTimeout(5000);
    try {  
        await expect(video).not.toHaveAttribute('paused');
        console.log("after 5 seconds, not paused");
    } catch (error) {
      console.error("video paused after 5 seconds",error); // Handle errors gracefully
    }

    //scroll to new video based on its locator
    let new_video = page.locator('video').nth(2);
    await new_video.scrollIntoViewIfNeeded();
    console.log("scroll to new video");

    await expect(new_video).toBeVisible();
    console.log("2nd video is visible");

    await expect(new_video).not.toHaveAttribute('paused');
    console.log("2nd video is not paused");

    const new_duration = await new_video.evaluate(new_video => new_video.duration);
    console.log('2nd Video Duration:', new_duration);

    // await page.waitForTimeout(3000);

    //scroll to new video based on its locator
    let third_video = page.locator('video').nth(5);
    await third_video.scrollIntoViewIfNeeded();
    console.log("scroll to third video");

    await expect(third_video).toBeVisible();
    console.log("3rd video is visible");

    await expect(third_video).not.toHaveAttribute('paused');
    console.log("3rd video is not paused");
    
    const third_duration = await third_video.evaluate(new_video => new_video.duration);
    console.log('3rd Video Duration:', third_duration);

  });

  test('Test google login', async ({page}) => {

    // login from wallet: using headfull mode
    try {
      console.log('Log in test');
      
      // const environment_pass = global.expect;
      const environment_pass = process.env.TESTPARAM; 

      await page.waitForTimeout(3000);

      await page.getByRole('navigation').getByRole('link').nth(3).click();
      await page.getByRole('button', { name: 'Login to claim your COYNs' }).click();
      const page1Promise = page.waitForEvent('popup');
      await page.getByRole('button', { name: 'Google Sign-In' }).click();
      const page1 = await page1Promise;
      await page1.getByLabel('Email or phone').click();
      await page1.getByLabel('Email or phone').fill('testautomationyral@googlemail.com');
      
      await page.waitForTimeout(2000);
      await page1.getByLabel('Email or phone').press('Enter');
      
      await page1.getByLabel('Enter your password').click();
      await page1.getByLabel('Enter your password').fill(environment_pass);
      await page.waitForTimeout(2000);

      await page1.getByLabel('Enter your password').press('Enter');
      // await page1.pause();
      await page.waitForTimeout(3000);

      console.log('Logged in succesfully');
      // await page.getByRole('navigation').getByRole('link').first().click();

      await page.waitForTimeout(3000);
        //request to the API endpoint to fetch the response
      const response = await fetch('https://yral-metadata.fly.dev/metadata/gzlng-jqzta-5kubz-4nyam-5so2e-tsoio-ijv2s-47dsw-7ksd7-pe3eb-zqe');
      await page.waitForTimeout(2000);

      console.log(response.status);
      const responseData = await response.json();
      console.log(responseData);

    } catch (error) {
      console.error("error in login"); 
    }
  });

  // test('TEST UPLOAD FLOW', async ({page}) => {

  //   // login from wallet: using headfull mode
  //   console.log('Uploading test');

  //   await page.waitForTimeout(2000);

  //   await page.getByRole('navigation').getByRole('link').nth(2).click();

  //   await page.setInputFiles('#dropzone-file', './test3.mp4');

  //   // TODO
  //   // //endpoint to fetch the response
  //   // const response = await fetch('https://yral.com/api/stream_to_offchain_agent11958048345285885967');
  //   // console.log(response);
  //   // const responseData = await response.json();

  //   // console.log(responseData);
  //   // // Perform assertions based on the API response
  //   // expect(responseData.status).toBe(200);

  //   await page.getByPlaceholder('Write your description here..').click();
  //   await page.getByPlaceholder('Write your description here..').fill('Monica Dance video');
  //   await page.getByPlaceholder('#hashtag1,#hashtag2,#hashtag3').click();
  //   await page.getByPlaceholder('#hashtag1,#hashtag2,#hashtag3').fill('#try');

  //   // const button = 
  //   // await button.click('button[data-api-trigger="true"]');

  //   await page.getByRole('button', { name: 'Upload Video' }).click();
    
  //   console.log('Uploading .. .. ..');

  //   //wait for upload to complete, Continue Browsing button to appear
  //   // await page.waitForSelector('button[name="Continue Browsing"]', { state: 'visible' });

  //   // Wait for the button with the name 'Continue Browsing' to become clickable
  //   await page.waitForFunction(() => {
  //     const button = document.querySelector('button[name="Continue Browsing"]');
  //     return button && !button.disabled;
  //   });
    
  //   // await page.waitForTimeout(20000);

  //   // await getByRole('button', { name: 'Continue Browsing' }).click();
  //   // // await page.pause();

  //   console.log('Uploaded succesfully');

  //   // await button.click('button[data-api-trigger="true"]');

  //   await page.getByRole('navigation').getByRole('link').first().click();
  //   await page.waitForTimeout(5000);

  // });

  // //test like functionality
  // test('Test like', async ({page}) => {

  //   // login from wallet: using headfull mode
  //   // try {
  //     console.log('test like');

  //     await page.waitForTimeout(1000);

  //     let likes_locator = page.locator('.flex > div:nth-child(2) > .flex > .text-sm').first();
  //     let total_likes = await likes_locator.textContent();
  //     console.log("total likes: ", total_likes );

  //     await page.waitForTimeout(15000);

  //     await expect(page.locator('button').first()).toBeVisible();
  //     console.log('like button visible');

  //     //click like button
  //     await page.locator('button').first().click();
  //     // await page.locator('body > main > div.h-full.w-full.overflow-hidden.overflow-y-auto > div > div:nth-child(2) > div > div.flex.flex-row.flex-nowrap.justify-between.items-end.pb-16.px-2.md\:px-6.w-full.text-white.absolute.bottom-0.left-0.bg-transparent.z-\[4\] > div.flex.flex-col.gap-6.items-end.w-3\/12.text-4xl > div > button > svg > path').click();
  //     await page.waitForTimeout(2000);

  //     let new_likes_locator = page.locator('.flex > div:nth-child(2) > .flex > .text-sm').first();
  //     let updated_likes = await new_likes_locator.textContent();
  //     console.log("updated total likes: ", updated_likes );

  //     expect(parseInt(updated_likes)).toBe(parseInt(total_likes) + 1);
  //     // let new_like_count = expect(page.getByRole('main'));
  //     // console.log("new like count: ", new_like_count );

  //     // await expect(page.getByRole('main')).toContainText(like_count + 1);

  //   // } catch (error) {
  //   //   console.error("error in login");
  //   // }
  // });

});
