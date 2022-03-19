const express = require('express');



const PORT = 8080;

const main = () => {
    const app = express();

    app.get('/', (_request, response) => {
        response.send(`
            <h1>
                a Node.js app deployed on plurid.app
            </h1>
        `);
    });

    app.listen(PORT);
}

main();
