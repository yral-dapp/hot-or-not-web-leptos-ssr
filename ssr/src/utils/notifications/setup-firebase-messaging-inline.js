import "https://www.gstatic.com/firebasejs/9.2.0/firebase-app-compat.js";
import "https://www.gstatic.com/firebasejs/9.2.0/firebase-messaging-compat.js";

let initialized = false;

function showNotification(payload) {
  const notificationTitle = payload.notification.title;
  const notificationOptions = {
    body: payload.notification.body
  };

  if (!("Notification" in window)) {
    console.log("This browser does not support desktop notification");
    return;
  }

  if (Notification.permission === "granted") {
    new Notification(notificationTitle, notificationOptions);
  }
}

export function init_firebase() {
  if (initialized) {
    return;
  }

  firebase.initializeApp({
    apiKey: "AIzaSyCwo0EWTJz_w-J1lUf9w9NcEBdLNmGUaIo",
    authDomain: "hot-or-not-feed-intelligence.firebaseapp.com",
    projectId: "hot-or-not-feed-intelligence",
    storageBucket: "hot-or-not-feed-intelligence.appspot.com",
    messagingSenderId: "82502260393",
    appId: "1:82502260393:web:390e9d4e588cba65237bb8",
  });

  // Set up foreground message handler
  const messaging = firebase.messaging();
  messaging.onMessage((payload) => {
    console.log('Message received in foreground:', payload);
    showNotification(payload);
  });

  initialized = true;
}

const vapidKey =
  "BOmsEya6dANYUoElzlUWv3Jekmw08_nqDEUFu06aTak-HQGd-G_Lsk8y4Bs9B4kcEjBM8FXF0IQ_oOpJDmU3zMs";

export default function get_token() {
  init_firebase();
  return new Promise((resolve, reject) => {
    const messaging = firebase.messaging();
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
