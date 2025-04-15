import { initializeApp } from "https://www.gstatic.com/firebasejs/10.14.1/firebase-app.js";
import { getMessaging } from "https://www.gstatic.com/firebasejs/10.14.1/firebase-messaging.js";

const app = initializeApp({
  apiKey: "AIzaSyCwo0EWTJz_w-J1lUf9w9NcEBdLNmGUaIo",
  authDomain: "hot-or-not-feed-intelligence.firebaseapp.com",
  projectId: "hot-or-not-feed-intelligence",
  storageBucket: "hot-or-not-feed-intelligence.appspot.com",
  messagingSenderId: "82502260393",
  appId: "1:82502260393:web:390e9d4e588cba65237bb8",
});

const messaging = getMessaging(app);

// This is called when a message is received while the app is in the background
messaging.onBackgroundMessage((payload) => {
  const { notification } = payload;

  // We can modify the notification here

  const { title, body, image } = notification;
  const notificationOptions = {
    body,
    icon: image,
  };

  self.registration.showNotification(title, notificationOptions);
});
