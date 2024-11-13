import "https://www.gstatic.com/firebasejs/9.2.0/firebase-app-compat.js";
import "https://www.gstatic.com/firebasejs/9.2.0/firebase-messaging-compat.js";

firebase.initializeApp({
  apiKey: "AIzaSyCwo0EWTJz_w-J1lUf9w9NcEBdLNmGUaIo",
  authDomain: "hot-or-not-feed-intelligence.firebaseapp.com",
  projectId: "hot-or-not-feed-intelligence",
  storageBucket: "hot-or-not-feed-intelligence.appspot.com",
  messagingSenderId: "82502260393",
  appId: "1:82502260393:web:390e9d4e588cba65237bb8",
});

let vapidKey =
  "BOmsEya6dANYUoElzlUWv3Jekmw08_nqDEUFu06aTak-HQGd-G_Lsk8y4Bs9B4kcEjBM8FXF0IQ_oOpJDmU3zMs";

export function get_token() {
  return new Promise((resolve, reject) => {
    const messaging = firebase.messaging();

    // Handle foreground messages
    messaging.onMessage((payload) => {
      if (!("Notification" in window)) {
        console.log("This browser does not support notifications");
        return;
      }

      if (Notification.permission === "granted") {
        const notification = new Notification(payload.notification.title, {
          body: payload.notification.body,
          icon: '/img/android-chrome-192x192.png',
          badge: '/img/android-chrome-192x192.png',
          vibrate: [200, 100, 200],
          tag: 'yral-notification'
        });

        notification.onclick = function() {
          window.focus();
          notification.close();
        };
      }
    });

    messaging
      .getToken({ vapidKey: vapidKey })
      .then((currentToken) => {
        resolve(currentToken);
      })
      .catch((err) => {
        console.log("An error occurred while retrieving token. ", err);
        return reject("An error occurred while retrieving token.");
      });
  });
}
