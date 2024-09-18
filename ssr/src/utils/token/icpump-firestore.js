// const firebaseConfig = {
//   apiKey: "AIzaSyASDBb33McHVMntTDAJripGRpAmubygwFo",
//   authDomain: "icpump.firebaseapp.com",
//   projectId: "icpump",
//   storageBucket: "icpump.appspot.com",
//   messagingSenderId: "1038497022920",
//   appId: "1:1038497022920:web:c5b2322dd133fa95a036fd",
//   measurementId: "G-8BNY3XQJKZ"
// };

// firestore

// import { initializeApp } from "https://www.gstatic.com/firebasejs/9.2.0/firebase-app.js";
// import {
//   getFirestore,
//   collection,
//   getDocs,
// } from "https://www.gstatic.com/firebasejs/9.2.0/firebase-firestore.js";

// const firebaseConfig = {
//   apiKey: "AIzaSyASDBb33McHVMntTDAJripGRpAmubygwFo",
//   authDomain: "icpump.firebaseapp.com",
//   projectId: "icpump",
//   storageBucket: "icpump.appspot.com",
//   messagingSenderId: "1038497022920",
//   appId: "1:1038497022920:web:c5b2322dd133fa95a036fd",
//   measurementId: "G-8BNY3XQJKZ",
// };

// const app = initializeApp(firebaseConfig, "secondary");
// const db = getFirestore(app);

// async function getDocsFromCollection(collectionName, db) {
//   try {
//     const querySnapshot = await getDocs(collection(db, collectionName));
//     querySnapshot.forEach((doc) => {
//       console.log(`${doc.id} => ${JSON.stringify(doc.data())}`);
//     });
//     return querySnapshot.docs.map((doc) => ({ id: doc.id, ...doc.data() }));
//   } catch (error) {
//     console.error("Error getting documents: ", error);
//     return [];
//   }
// }

// export async function get_token_list() {
//   return getDocsFromCollection("tokens", db)
//     .then((documents) => {
//       console.log("Retrieved documents:", documents);
//     })
//     .catch((error) => {
//       console.error("Error:", error);
//     });
// }
