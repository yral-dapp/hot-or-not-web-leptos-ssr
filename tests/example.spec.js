import { chromium } from 'playwright';
import { test, expect } from '@playwright/test';
import nock from 'nock';

test.beforeEach(async ({ page }) => {
  await page.goto('https://yral.com/');
});

test.describe('New test', () => {
  test('Has title', async ({page}) => {

    // Check opening status
    const title = await page.title();
    console.log(title); // Log the page title
    // expect(title).toContain('Expected Title'); // Replace with expected title
    expect(title).toBe("Yral");

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
    await page.waitForTimeout(3000);
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

    await page.waitForTimeout(3000);

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

    await page.waitForTimeout(3000);

  });

  // await page.waitForTimeout(5000);
  test('Test google login', async ({page}) => {

    // login from wallet: using headfull mode
    console.log('Log in test');

    await page.getByRole('navigation').getByRole('link').nth(3).click();
    await page.getByRole('button', { name: 'Login to claim your COYNs' }).click();
    const page1Promise = page.waitForEvent('popup');
    await page.getByRole('button', { name: 'Google Sign-In' }).click();
    const page1 = await page1Promise;
    await page1.getByLabel('Email or phone').click();
    await page1.getByLabel('Email or phone').fill('testautomationyral@gmail.com');
    
    await page.waitForTimeout(2000);
    await page1.getByLabel('Email or phone').press('Enter');
    
    await page1.getByLabel('Enter your password').click();
    await page1.getByLabel('Enter your password').fill('test@yral');
    await page.waitForTimeout(2000);

    await page1.getByLabel('Enter your password').press('Enter');
    await page1.pause();

    console.log('Logged in succesfully');
    await page.getByRole('navigation').getByRole('link').first().click();

    await page.waitForTimeout(3000);
  });

    // const isPlaying = async () => {
    //     return await videoElement.evaluate(videoElement => !!(videoElement.currentTime > 0 && !videoElement.paused && !videoElement.ended && videoElement.readyState > 2));
    //   };

    // try {
    //     const value = isPlaying;
    //     console.log(value); // Will output: false
    // } catch (error) {
    //     console.error("An error occurred:"  , error);
    // }
    // const isPlaying = await videoElement.evaluate(() => document.querySelector('video.object-contain.h-dvh.max-h-dvh.cursor-pointer').paused === false); // Check playback state
    // console.log("is paused: ", isPlaying.context);

    // expect(isPlaying).toBeTruthy(); 

});
