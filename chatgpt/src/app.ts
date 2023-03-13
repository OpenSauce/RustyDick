import express from 'express';
import * as dotenv from 'dotenv';
dotenv.config()

import { ChatGPTUnofficialProxyAPI } from 'chatgpt'

import parser from 'body-parser';

const app = express();
const port = 3000;

app.use(parser.json())

async function messageChatGPT(query: string): Promise<String> {
    const api = new ChatGPTUnofficialProxyAPI({
        accessToken: process.env.OPENAI_ACCESS_TOKEN,
        apiReverseProxyUrl: 'https://bypass.duti.tech/api/conversation'
    })

    return api.sendMessage(query)
        .then(res => {
            return res.text
        })
        .catch(err => {
            return Promise.reject(err);
        })
}

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

app.listen(port, () => {
    return console.log(`Express is listening at http://localhost:${port}`);
});