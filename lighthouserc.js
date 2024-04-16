module.exports = {
    ci: {
        collect: {
        url: ['https://yral.com/'],
        },
        upload: {
            target: 'temporary-public-storage',
        },
        assert: {
            "categories:performance": ["warn", {"minScore": 0.3}],
            "categories:accessibility": ["warn", {"minScore": 0.5}]
        },
    },
};
