import 'https://www.gstatic.com/firebasejs/9.2.0/firebase-app-compat.js';
import 'https://www.gstatic.com/firebasejs/9.2.0/firebase-messaging-compat.js';

// AUTH HERE

const messaging = firebase.messaging();

export function get_token(flag) {
    console.log('get_token flag: ', flag);
    return new Promise((resolve, reject) => {
        messaging.getToken({ vapidKey: 'BOmsEya6dANYUoElzlUWv3Jekmw08_nqDEUFu06aTak-HQGd-G_Lsk8y4Bs9B4kcEjBM8FXF0IQ_oOpJDmU3zMs' }).then((currentToken) => {
            resolve(currentToken);
            // if (currentToken) {
            //     // Send the token to your server and update the UI if necessary
            //     console.log('currentToken', currentToken);
            //     console.log(typeof currentToken);
            //     return currentToken;
            // } else {
            //     // Show permission request UI
            //     console.log('No registration token available. Request permission to generate one.');
            //     return reject('No registration token available. Request permission to generate one.');
            // }
        }).catch((err) => {
            console.log('An error occurred while retrieving token. ', err);
            return reject('An error occurred while retrieving token.');
        });
    });
}