// import { test, expect } from '@playwright/test';

// test.beforeEach(async ({ page }) => {
//   await page.goto('https://yral.com/');
// });

// test.describe('New test', () => {
//  //test like functionality
//   test('Test like', async ({page}) => {

//     // login from wallet: using headfull mode
//     // try {
//       console.log('test like');

//       await page.waitForTimeout(1000);

//       let likes_locator = page.locator('.flex > div:nth-child(2) > .flex > .text-sm').first();
//       let total_likes = await likes_locator.textContent();
//       console.log("total likes: ", total_likes );

//       await page.waitForTimeout(15000);

//       await expect(page.locator('button').first()).toBeVisible();
//       console.log('like button visible');

//       //click like button
//       await page.locator('button').first().click();
//       // await page.locator('body > main > div.h-full.w-full.overflow-hidden.overflow-y-auto > div > div:nth-child(2) > div > div.flex.flex-row.flex-nowrap.justify-between.items-end.pb-16.px-2.md\:px-6.w-full.text-white.absolute.bottom-0.left-0.bg-transparent.z-\[4\] > div.flex.flex-col.gap-6.items-end.w-3\/12.text-4xl > div > button > svg > path').click();
//       await page.waitForTimeout(2000);

//       let new_likes_locator = page.locator('.flex > div:nth-child(2) > .flex > .text-sm').first();
//       let updated_likes = await new_likes_locator.textContent();
//       console.log("updated total likes: ", updated_likes );

//       expect(parseInt(updated_likes)).toBe(parseInt(total_likes) + 1);
//       // let new_like_count = expect(page.getByRole('main'));
//       // console.log("new like count: ", new_like_count );

//       // await expect(page.getByRole('main')).toContainText(like_count + 1);

//     // } catch (error) {
//     //   console.error("error in login");
//     // }
//   });

// })