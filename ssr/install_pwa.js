let installPrompt;

// Function to trigger PWA installation

// Function to check if the PWA is already installed
// export async function isPwaInstalled() {
//   try {
//     const relatedApps = await navigator.getInstalledRelatedApps();
//     const isInstalled = relatedApps.some((app) => app.id === "your-app-id");
//     return isInstalled;
//   } catch (error) {
//     console.error("Error checking if PWA is installed:", error);
//     return false;
//   }
// }

const installButton = document.querySelector("#installApp");

window.addEventListener("beforeinstallprompt", (event) => {
  console.log("beforeinstallprompt event fired", event);
  console.log("PWA not  installed");
  localStorage.setItem("pwainstalled", "false");

  event.preventDefault(); // Prevent default prompt from showing
  installPrompt = event; // Save the event for triggering later

  if (installButton) {
    installButton.removeAttribute("hidden"); // Show the install button
  }
});

window.addEventListener("appinstalled", () => {
  console.log("PWA installed");
  localStorage.setItem("pwainstalled", "true");

  installPrompt = null;
  if (installButton) {
    installButton.setAttribute("hidden", ""); // Hide the install button
  }
});

window.addEventListener("load", () => {
  console.log("Page loaded");

  if (!installPrompt) {
    console.warn("Install prompt never fired. Check installability criteria.");
    console.log("pwa installed");
    // const pwainstalled = localStorage.getItem("pwainstalled") === "true";

    localStorage.setItem("pwainstalled", "true");
  }
});
export function triggerPwaInstall() {
  return new Promise((resolve, reject) => {
    if (installPrompt) {
      console.log("Prompt available, triggering...");
      installPrompt.prompt();
      installPrompt.userChoice
        .then((choice) => {
          if (choice.outcome === "accepted") {
            console.log("User accepted the install prompt");
            resolve("accepted");
          } else {
            console.log("User dismissed the install prompt");
            resolve("dismissed");
          }
          installPrompt = null; // Reset the prompt
        })
        .catch(reject);
    } else {
      console.error("Install prompt not available.");
      reject("Install prompt not available.");
    }
  });
}
