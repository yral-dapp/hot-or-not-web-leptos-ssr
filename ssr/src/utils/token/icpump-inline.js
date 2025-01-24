import { initializeApp } from "https://www.gstatic.com/firebasejs/10.14.1/firebase-app.js";
import {
  getFirestore,
  collection,
  query,
  limit,
  where,
  orderBy,
  getDocs,
  onSnapshot,
  Timestamp,
  Firestore,
  QuerySnapshot,
  QueryDocumentSnapshot,
} from "https://www.gstatic.com/firebasejs/10.14.1/firebase-firestore.js";

// Create a firebase object to export
const firebase = {
  initializeApp,
  getFirestore,
  firestore: {
    collection,
    query,
    where,
    limit,
    orderBy,
    getDocs,
    onSnapshot,
    Timestamp,
  },
};

// Export the firebase object
export { firebase };

// Also export the types that are used in the Rust code
export { Firestore, QuerySnapshot, QueryDocumentSnapshot };
