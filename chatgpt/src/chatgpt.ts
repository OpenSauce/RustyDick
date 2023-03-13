import { ChatGPTUnofficialProxyAPI } from 'chatgpt'


var api: ChatGPTUnofficialProxyAPI = null;

export function startChatGPT(token: string) {
    api = new ChatGPTUnofficialProxyAPI({
        accessToken: token,
        apiReverseProxyUrl: 'https://bypass.duti.tech/api/conversation'
    })
}

export async function messageChatGPT(query: string): Promise<String> {
    if (api == null) {
        return Promise.reject("ChatGPT Proxy has not started.");
    }

    return api.sendMessage(query)
        .then(res => {
            return res.text
        })
        .catch(err => {
            return Promise.reject(err);
        })
}