import { useParams } from "react-router";
import { useEffect, useRef, useState } from "react";
import { fetchChatMessages } from "../../lib/api/messages";
import { isError } from "../../lib/api/error";
import { Role } from "../../lib/model/message";
import { Message } from "../../components/message";
import { useChatsStore } from "../../lib/state/chats";

export function Chat() {
  const scrollWrapper = useRef<HTMLDivElement>(null);
  const scrollableContainer = useRef<HTMLDivElement>(null);
  const params = useParams();
  const chatId = params["chatId"] as string;
  const messages = useChatsStore((state) =>
    state.chats[chatId]?.state.status === "success"
      ? state.chats[chatId].messages
      : null,
  );
  const chatLoading = useChatsStore(
    (state) => state.chats[chatId]?.state.status === "loading",
  );
  const setMessages = useChatsStore((state) => state.setChatMessages);
  const setChatState = useChatsStore((state) => state.setChatState);
  const pendingMessage = useChatsStore(state => state.pendingMessages[chatId]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!scrollWrapper.current || !scrollableContainer.current) return;
    scrollWrapper.current.scrollTo({
      top: scrollableContainer.current.clientHeight,
      behavior: "instant",
    });
  }, [messages]);

  useEffect(() => {
    if (!scrollWrapper.current || !scrollableContainer.current) return;
    if (
      scrollableContainer.current.getBoundingClientRect().height -
      scrollWrapper.current.scrollTop >
      120
    )
      return;
    scrollWrapper.current.scrollTo({
      top: scrollableContainer.current.clientHeight,
      behavior: "instant",
    });
  }, [pendingMessage]);

  useEffect(() => {
    if (!chatLoading && !messages) {
      setChatState(chatId, { status: "loading" });
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
        setChatState(chatId, { status: "success" });
        setMessages(chatId, result);
      });
    } else {
      setIsLoading(false);
    }
  }, [messages, chatId, setMessages, setChatState, chatLoading]);

  return (
    <div
      className="w-full h-full overflow-y-scroll p-6 pb-32 overscroll-contain"
      ref={scrollWrapper}
    >
      {isLoading ? "loading" : null}
      {error ? `error: ${error}` : null}
      <div
        className="max-w-4xl flex flex-col-reverse mx-auto"
        ref={scrollableContainer}
      >
        {pendingMessage ? (
          <Message
            message={{
              chat_id: chatId,
              content: pendingMessage.content,
              reasoning: pendingMessage.reasoning,
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
