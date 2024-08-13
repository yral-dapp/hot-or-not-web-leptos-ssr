import 'https://www.gstatic.com/firebasejs/9.2.0/firebase-app-compat.js';
import 'https://www.gstatic.com/firebasejs/9.2.0/firebase-messaging-compat.js';

export function get_token(
    apiKey,
    authDomain,
    projectId,
    storageBucket,
    messagingSenderId,
    appId,
    vapidKey,
) {
    firebase.initializeApp({
        apiKey: apiKey,
        authDomain: authDomain,
        projectId: projectId,
        storageBucket: storageBucket,
        messagingSenderId: messagingSenderId,
        appId: appId
    });
    const messaging = firebase.messaging();

    return new Promise((resolve, reject) => {
        messaging.getToken({ vapidKey: vapidKey }).then((currentToken) => {
            resolve(currentToken);
        }).catch((err) => {
            console.log('An error occurred while retrieving token. ', err);
            return reject('An error occurred while retrieving token.');
        });
    });
}