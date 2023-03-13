import * as dotenv from 'dotenv';
dotenv.config()

import express from 'express';
import parser from 'body-parser';
import { startChatGPT, messageChatGPT } from './chatgpt.js';

const app = express();
const port = 3000;

app.use(parser.json())

app.listen(port, () => {
    startChatGPT(process.env.OPENAI_ACCESS_TOKEN)
    return console.log(`Express is listening at http://localhost:${port}`);
});

app.post('/', (req, res) => {
    messageChatGPT(req.body.query).then(result => {
        res.send(result);
    }).catch(err => {
        if (err.statusCode != null) {
            res.sendStatus(err.statusCode);
            return;
        }

        res.sendStatus(500);
    });
});