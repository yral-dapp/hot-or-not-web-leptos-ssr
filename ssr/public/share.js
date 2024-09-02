
export function shareImage(title, text, url, imageUrl) {
    
    // Check if imageUrl is null
    if (imageUrl === null) {
        // Call shareContent with no files
        shareContent(title, text, url);
        return;
    }

    // If imageUrl is an array of URLs
    if (Array.isArray(imageUrl)) {
         Promise.all(imageUrl.map(fetchImage))
            .then(files => {
                shareContent(title, text, url, files);
            })
            .catch(error => {
                console.error('Error fetching images:', error);
            });

               
                           } else {
                                               shareContent(title, text, url);            
                           }
}

// Helper function to fetch and convert a single image URL to a File object
    function fetchImage(url) {
        return fetch(url)
            .then(response => response.blob())
            .then(blob => new File([blob], 'image.jpg', { type: 'image/jpeg' }));
    }


function shareContent(title, text, url, files = []) {
    if (navigator.share) {
        navigator.share({
            title: title,
            text: text,
            url: url,
            files: files,
        }).then(() => {
            console.log('Share successful');
        }).catch((error) => {
            console.error('Error sharing:', error);
        });
    } else {
        console.log('Web Share API not supported in this browser.');
    }
}
