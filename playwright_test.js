const { chromium } = require('playwright');
const { test, expect } = require ('@playwright/test');
const nock = require('nock');

(async () => {
    const browser = await chromium.launch({ headless: false });
    const context = await browser.newContext();

  try {
    // Open the website URL
    const page = await context.newPage();
    await page.goto('https://yral.com/'); 

    // Wait for the page to load  
    await page.waitForTimeout(10000); // Adjust timeout as needed

    // Check opening status
    const title = await page.title();
    console.log(title); // Log the page title
    // expect(title).toContain('Expected Title'); // Replace with expected title
    expect(title).toBe("Yral");

    // navigate to home page
    // await page.getByRole('navigation').  getByRole('link').first().click();

    //try with 'video' element
    let video = page.locator('video').nth(3);
    await expect(video).toBeVisible();
    console.log("is visible");

    await expect(video).not.toHaveAttribute('paused');
    console.log("is not paused");

    await expect(video).toHaveAttribute('muted'); 
    console.log("is muted");

    await page.locator('.fixed').first().click();
    await page.waitForTimeout(3000);

    let video_new = page.locator('video').nth(3);

    // await expect(video_new).not.toHaveAttribute('muted'); 
    // console.log("is not muted");


    const duration = await video.evaluate(videoEle => videoEle.duration);
    console.log('Video Duration:', duration);

    // const isVideoPlaying = await video.evaluate(() => document.querySelector('video').autoplay); // Check playback state
    // console.log("is video playing", isVideoPlaying);

    await page.waitForTimeout(3000);
    await expect(video).not.toHaveAttribute('paused');
    console.log("after 3 seconds, not paused");

    //scroll to new video based on its locator
    let new_video = page.locator('video').nth(2);
    await new_video.scrollIntoViewIfNeeded();
    console.log("scroll to new video");


    await expect(new_video).toBeVisible();
    console.log("2nd video is visible");

    await expect(new_video).not.toHaveAttribute('paused');
    console.log("2nd video is not paused");

    // await expect(new_video).toHaveAttribute('muted'); 
    // console.log("new video is muted");

    await page.waitForTimeout(3000);

    //scroll to new video based on its locator
    let third_video = page.locator('video').nth(5);
    await third_video.scrollIntoViewIfNeeded();
    console.log("scroll to third video");

    await expect(third_video).toBeVisible();
    console.log("3rd video is visible");

    await expect(third_video).not.toHaveAttribute('paused');
    console.log("3nd video is not paused");

    await page.waitForTimeout(3000);

    // await page.waitForTimeout(5000);

    // // login from wallet: using headfull mode
    // await page.getByRole('navigation').getByRole('link').nth(3).click();
    // await page.getByRole('button', { name: 'Login to claim your COYNs' }).click();
    // const page1Promise = page.waitForEvent('popup');
    // await page.getByRole('button', { name: 'Google Sign-In' }).click();
    // const page1 = await page1Promise;
    // await page1.getByLabel('Email or phone').click();
    // await page1.getByLabel('Email or phone').fill('testautomationyral@gmail.com');
    
    // await page.waitForTimeout(2000);
    // await page1.getByLabel('Email or phone').press('Enter');
    
    // await page1.getByLabel('Enter your password').click();
    // await page1.getByLabel('Enter your password').fill('test@yral');
    // await page.waitForTimeout(2000);

    // await page1.getByLabel('Enter your password').press('Enter');
    // await page.getByRole('navigation').getByRole('link').first().click();

    // await page.waitForTimeout(2000);

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


  } catch (error) {
    console.error(error); // Handle errors gracefully
  } finally {
    await browser.close(); // Always close the browser context
  }
})();
