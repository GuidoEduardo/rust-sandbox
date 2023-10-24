const fibonacci = () => {
    let results = [], b = 1, next = a = 0;

    for (let i = 1; i <= 10000; i++) {
        results.push(a);
        
        next = a + b;
        a = b;
        b = next;
    }

    return results;
};

fibonacci();