module.exports = {
    ci: {
        collect: {
        url: ['http://127.0.0.1:3000'],
        },
        upload: {
            target: 'temporary-public-storage',
        },
        assert: {
            assertions: {
                "categories:performance": ["warn", {"minScore": 0.3}],
                "categories:accessibility": ["warn", {"minScore": 0.5}]
            },
        },
    },
};
