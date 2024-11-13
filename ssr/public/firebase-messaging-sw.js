importScripts(
  "https://www.gstatic.com/firebasejs/9.2.0/firebase-app-compat.js",
);
importScripts(
  "https://www.gstatic.com/firebasejs/9.2.0/firebase-messaging-compat.js",
);

firebase.initializeApp({
  // https://firebase.google.com/docs/projects/api-keys#:~:text=it%27s%20OK%20to%20include%20Firebase%20API%20keys%20in%20your%20code
  apiKey: "AIzaSyCwo0EWTJz_w-J1lUf9w9NcEBdLNmGUaIo",
  authDomain: "hot-or-not-feed-intelligence.firebaseapp.com",
  projectId: "hot-or-not-feed-intelligence",
  storageBucket: "hot-or-not-feed-intelligence.appspot.com",
  messagingSenderId: "82502260393",
  appId: "1:82502260393:web:390e9d4e588cba65237bb8",
});

const messaging = firebase.messaging();

// Handle foreground messages
messaging.onMessage((payload) => {
  const notificationTitle = payload.notification.title;
  const notificationOptions = {
    body: payload.notification.body,
    icon: '/img/android-chrome-192x192.png',
    badge: '/img/android-chrome-192x192.png',
    vibrate: [200, 100, 200],
    tag: 'yral-notification'
  };

  self.registration.showNotification(notificationTitle, notificationOptions);
});

// Handle background messages
messaging.onBackgroundMessage((payload) => {
  const notificationTitle = payload.notification.title;
  const notificationOptions = {
    body: payload.notification.body,
    icon: '/img/android-chrome-192x192.png',
    badge: '/img/android-chrome-192x192.png',
    vibrate: [200, 100, 200],
    tag: 'yral-notification'
  };

  self.registration.showNotification(notificationTitle, notificationOptions);
});