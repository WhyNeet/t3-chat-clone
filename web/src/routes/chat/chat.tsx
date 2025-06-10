import { useParams } from "react-router";
import { useMessagesStore } from "../../lib/state/messages";
import { useEffect, useState } from "react";
import { fetchChatMessages } from "../../lib/api/messages";
import { isError } from "../../lib/api/error";
import Markdown from "react-markdown";

export function Chat() {
  const params = useParams();
  const chatId = params["chatId"] as string;
  const messages = useMessagesStore(state => state.mapping.get(chatId));
  const setMessages = useMessagesStore(state => state.updateChatMessages);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    if (!messages) {
      fetchChatMessages(chatId, { start: 0, take: window.innerHeight / 100 }).then(result => {
        setIsLoading(false);
        if (isError(result)) {
          setError(result.error);
          return;
        }
        setMessages(chatId, result);
      })
    } else {
      setIsLoading(false);
    }
  }, [messages, chatId, setMessages])

  return <div className="w-full h-full">
    {isLoading ? "loading" : null}
    {error ? `error: ${error}` : null}
    {messages ? messages.map(message => <div key={message.id} className="mb-2 p-4 border border-transparent hover:border-pink-950 rounded-lg">
      <div className="prose">
        <Markdown>{message.content}</Markdown>
      </div>
    </div>) : null}
  </div>;
}
