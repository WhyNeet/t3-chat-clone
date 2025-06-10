import { useParams } from "react-router";
import { useEffect, useState } from "react";
import { fetchChatMessages } from "../../lib/api/messages";
import { isError } from "../../lib/api/error";
import { Role } from "../../lib/model/message";
import { Message } from "../../components/message";
import { useChatsStore } from "../../lib/state/chats";

export function Chat() {
  const params = useParams();
  const chatId = params["chatId"] as string;
  const messages = useChatsStore((state) => state.chats[chatId]?.state.status === "success" ? state.chats[chatId].state.messages : null);
  const setMessages = useChatsStore((state) => state.addChatMessages);
  const setChatState = useChatsStore((state) => state.setChatState);
  const pendingMessage = useChatsStore((state) =>
    state.pendingMessages[chatId],
  );
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!messages) {
      setChatState(chatId, { status: "loading" })
      fetchChatMessages(chatId, {
        start: 0,
        take: window.innerHeight / 100,
      }).then((result) => {
        setIsLoading(false);
        if (isError(result)) {
          setChatState(chatId, { status: "error", error: result.error });
          setError(result.error);
          return;
        }
        setChatState(chatId, { status: "success", messages: result });
        setMessages(chatId, result);
      });
    } else {
      setIsLoading(false);
    }
  }, [messages, chatId, setMessages, setChatState]);

  return (
    <div className="w-full h-full overflow-y-scroll p-6 pb-24 overscroll-contain">
      {isLoading ? "loading" : null}
      {error ? `error: ${error}` : null}
      <div className="max-w-4xl flex flex-col-reverse mx-auto">
        {pendingMessage ? (
          <Message
            message={{
              chat_id: chatId,
              content: pendingMessage,
              id: "pending-message",
              role: Role.Assistant,
              timestamp: "",
            }}
          />
        ) : null}
        {messages
          ? messages.map((message) => (
            <Message key={message.id} message={message} />
          ))
          : null}
      </div>
    </div>
  );
}
