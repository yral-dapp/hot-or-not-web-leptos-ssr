import 'https://www.gstatic.com/firebasejs/9.2.0/firebase-app-compat.js';
import 'https://www.gstatic.com/firebasejs/9.2.0/firebase-messaging-compat.js';

firebase.initializeApp({
    apiKey: "AIzaSyCwo0EWTJz_w-J1lUf9w9NcEBdLNmGUaIo",
    authDomain: "hot-or-not-feed-intelligence.firebaseapp.com",
    projectId: "hot-or-not-feed-intelligence",
    storageBucket: "hot-or-not-feed-intelligence.appspot.com",
    messagingSenderId: "82502260393",
    appId: "1:82502260393:web:390e9d4e588cba65237bb8"
});

let vapidKey = "BOmsEya6dANYUoElzlUWv3Jekmw08_nqDEUFu06aTak-HQGd-G_Lsk8y4Bs9B4kcEjBM8FXF0IQ_oOpJDmU3zMs";

export function get_token() {
    return new Promise((resolve, reject) => {
        const messaging = firebase.messaging();
        messaging.getToken({ vapidKey: vapidKey }).then((currentToken) => {
            resolve(currentToken);
        }).catch((err) => {
            console.log('An error occurred while retrieving token. ', err);
            return reject('An error occurred while retrieving token.');
        });
    });
}