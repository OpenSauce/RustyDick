import * as dotenv from 'dotenv';
dotenv.config()

import express from 'express';
import parser from 'body-parser';
import { startChatGPT, messageChatGPT } from './chatgpt.js';

const app = express();
const port = process.env.PORT;

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
                    console.error(err)
            res.sendStatus(err.statusCode);
            return;
        }
        console.error(err)
        res.sendStatus(500);
    });
});